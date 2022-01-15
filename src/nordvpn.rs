use byte_unit::Byte;
use chrono::{Duration, NaiveDate};
use semver::Version;
use std::net::IpAddr;
use std::process::{Command, Output};
use strum;
use thiserror::Error;

type CliResult<T> = Result<T, CliError>;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("unable to create command")]
    IoError(#[from] std::io::Error),
    #[error("command terminated unsuccessfully")]
    FailedCommand(Command),
    #[error("failed to get command output as UTF-8")]
    BadEncoding(#[from] std::string::FromUtf8Error),
    #[error("command output did not match as expected")]
    BadOutput(Command),
    #[error("failed to parse string as `NaiveDate`")]
    BadDateFormat(#[from] chrono::ParseError),
    #[error("failed to parse semantic version")]
    BadVersion(#[from] semver::Error),
}

#[derive(Debug)]
pub enum ConnectOption {
    Country(String),
    Server(String),
    CountryCode(String),
    City(String),
    Group(String),
    CountryCity(String, String),
}

#[derive(Debug)]
pub struct Account {
    pub email: String,
    pub active: bool,
    pub expires: NaiveDate,
}

#[derive(Debug)]
pub struct Connected {
    pub country: String,
    pub server: u32,
    pub hostname: String,
}

#[derive(Debug)]
pub struct Status {
    pub hostname: String,
    pub country: String,
    pub city: String,
    pub ip: IpAddr,
    pub technology: Technology,
    pub protocol: Protocol,
    pub transfer: Transfer,
    pub uptime: Duration,
}

#[derive(Debug, strum::EnumString)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Technology {
    OpenVpn,
    NordLynx,
}

#[derive(Debug, strum::EnumString)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Debug)]
pub struct Transfer {
    pub recieved: Byte,
    pub sent: Byte,
}

mod cli_re {
    use once_cell::sync::Lazy;
    use regex::Regex;

    pub mod re {
        use super::*;

        pub const WORD_LIST: Lazy<Regex> = Lazy::new(|| Regex::new(generic::WORD_LIST).unwrap());

        pub const ACCOUNT: Lazy<Regex> = Lazy::new(|| {
            Regex::new(&format!(
                r#"(?:{}|{}|{})+"#,
                account::EMAIL,
                account::ACTIVE,
                account::EXPIRES
            ))
            .unwrap()
        });
        pub const CONNECT: Lazy<Regex> =
            Lazy::new(|| Regex::new(connect::COUNTRY_SERVER_HOSTNAME).unwrap());
        pub const LOGIN: Lazy<Regex> = Lazy::new(|| Regex::new(login::URL).unwrap());
        pub const STATUS: Lazy<Regex> = Lazy::new(|| {
            Regex::new(&format!(
                r#"(?:{}|{}|{}|{}|{}|{}|{}|{})+"#,
                status::HOSTNAME,
                status::COUNTRY,
                status::CITY,
                status::IP,
                status::TECHNOLOGY,
                status::PROTOCOL,
                status::TRANSFER,
                status::UPTIME
            ))
            .unwrap()
        });
        pub const VERSION: Lazy<Regex> = Lazy::new(|| Regex::new(version::VERSION).unwrap());
    }

    pub mod account {
        pub const EMAIL: &str = r#"Email Address:\s+(?P<email>.+)\s*(?:\n|$)"#;
        pub const ACTIVE: &str = r#"VPN Service:\s+(?P<active>(?i)[a-z]+)\s*"#;
        pub const EXPIRES: &str = r#"\(Expires on\s+(?P<expires_month>(?i)[a-z]{3})\s+(?P<expires_day>\d+)(?i:st|nd|rd|th),\s+(?P<expires_year>\d{4})\)"#;
    }

    pub mod connect {
        pub const COUNTRY_SERVER_HOSTNAME: &str = r#"You are connected to\s+(?P<country>(?i)[a-z_ ]+)\s+#(?P<server>\d+)\s+\((?P<hostname>[\w\d\-\.]+)\)!"#;
    }

    pub mod login {
        pub const URL: &str = r#"Continue in the browser:\s+(?P<url>.+)\s*(?:\n|$)"#;
    }

    pub mod status {
        pub const HOSTNAME: &str = r#"Current server:\s+(?P<hostname>[\w\d\-\.]+)\s*(?:\n|$)"#;
        pub const COUNTRY: &str = r#"Country:\s+(?P<country>(?i)[a-z_ ]+[a-z_ ])\s*(?:\n|$)"#;
        pub const CITY: &str = r#"City:\s+(?P<city>(?i)[a-z_ ]+[a-z_ ])\s*(?:\n|$)"#;
        pub const IP: &str = r#"Server IP:\s+(?P<ip>(?i)(?:[\da-f]{0,4}:){1,7}[\da-f]{0,4}|(?:\d{1,3}\.){3}\d{1,3})\s*(?:\n|$)"#;
        pub const TECHNOLOGY: &str =
            r#"Current technology:\s+(?P<technology>(?i)OPENVPN|NORDLYNX)\s*(?:\n|$)"#;
        pub const PROTOCOL: &str = r#"Current protocol:\s+(?P<protocol>(?i)TCP|UDP)\s*(?:\n|$)"#;
        pub const TRANSFER: &str = r#"Transfer:\s+(?i:(?P<transfer_received>(?:\d+\.)?\d+\s+[a-z]+)\s+received,\s+(?P<transfer_sent>(?:\d+\.)?\d+\s+[a-z]+)\s+sent)\s*(?:\n|$)"#;
        pub const UPTIME: &str = r#"Uptime:\s+(?i:(?:(?P<uptime_years>\d+)\s+years?\s*)?(?:(?P<uptime_months>\d+)\s+months?\s*)?(?:(?P<uptime_days>\d+)\s+days?\s*)?(?:(?P<uptime_hours>\d+)\s+hours?\s*)?(?:(?P<uptime_minutes>\d+)\s+minutes?\s*)?(?:(?P<uptime_seconds>\d+)\s+seconds?\s*)?)\s*(?:\n|$)"#;
    }

    pub mod version {
        pub const VERSION: &str = r#"(?P<version>\d+\.\d+.\d+)\s*(?:\n|$)"#;
    }

    pub mod generic {
        pub const WORD_LIST: &str = r#"(\w+)(?:,\s*|\s*$)"#;
    }

    pub fn parse_list(text: &str) -> Option<Vec<String>> {
        let mut captures = re::WORD_LIST.captures_iter(text).peekable();

        captures.peek()?;

        let items = captures.map(|capture| capture.get(1).unwrap().as_str().to_owned());

        Some(items.collect())
    }
}

pub struct NordVPN;

impl NordVPN {
    pub fn account() -> CliResult<Option<Account>> {
        static RE_EMAIL: Lazy<Regex> = Lazy::new(|| Regex::new(cli_re::account::EMAIL).unwrap());
        static RE_ACTIVE: Lazy<Regex> = Lazy::new(|| Regex::new(cli_re::account::ACTIVE).unwrap());
        static RE_EXPIRES: Lazy<Regex> =
            Lazy::new(|| Regex::new(cli_re::account::EXPIRES).unwrap());

        let (command, output, stdout) = Self::command(["nordvpn", "account"])?;

        if stdout.contains("You are not logged in.") {
            return Ok(None);
        } else if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let account = Account {
            email: if let Some(captures) = RE_EMAIL.captures(&stdout) {
                captures.get(1).unwrap().as_str().to_owned()
            } else {
                return Err(CliError::BadOutput(command));
            },
            active: if let Some(captures) = RE_ACTIVE.captures(&stdout) {
                captures.get(1).unwrap().as_str() == "Active"
            } else {
                return Err(CliError::BadOutput(command));
            },
            expires: if let Some(captures) = RE_EXPIRES.captures(&stdout) {
                NaiveDate::parse_from_str(
                    &format!(
                        "{}-{:02}-{}",
                        captures.get(1).unwrap().as_str(),
                        captures.get(2).unwrap().as_str(),
                        captures.get(3).unwrap().as_str()
                    ),
                    "%b-%d-%Y",
                )?
            } else {
                return Err(CliError::BadOutput(command));
            },
        };

        Ok(Some(account))
    }

    pub fn cities(country: &str) -> CliResult<Vec<String>> {
        let (command, output, stdout) = Self::command(["nordvpn", "cities", country])?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let cities = match cli_re::parse_list(&stdout) {
            Some(cities) => cities,
            None => return Err(CliError::BadOutput(command)),
        };

        Ok(cities)
    }

    pub fn connect(option: &ConnectOption) -> CliResult<Connected> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(cli_re::connect::SERVER_COUNTRY_HOSTNAME).unwrap());

        let mut run = vec!["nordvpn", "connect"];

        match option {
            ConnectOption::Country(country) => run.push(country),
            ConnectOption::Server(server) => run.push(server),
            ConnectOption::CountryCode(country_code) => run.push(country_code),
            ConnectOption::City(city) => run.push(city),
            ConnectOption::Group(group) => run.push(group),
            ConnectOption::CountryCity(country, city) => {
                run.push(country);
                run.push(city);
            }
        };

        let (command, output, stdout) = Self::command(run)?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let connected = match RE.captures(&stdout) {
            Some(captures) => Connected {
                country: captures.get(1).unwrap().as_str().to_owned(),
                server: captures.get(2).unwrap().as_str().parse().unwrap(),
                hostname: captures.get(3).unwrap().as_str().to_owned(),
            },
            None => return Err(CliError::BadOutput(command)),
        };

        Ok(connected)
    }

    pub fn countries() -> CliResult<Vec<String>> {
        let (command, output, stdout) = Self::command(["nordvpn", "countries"])?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let countries = match cli_re::parse_list(&stdout) {
            Some(countries) => countries,
            None => return Err(CliError::BadOutput(command)),
        };

        Ok(countries)
    }

    pub fn disconnect() -> CliResult<bool> {
        let (command, output, stdout) = Self::command(["nordvpn", "disconnect"])?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        if stdout.contains("You are not connected to NordVPN.") {
            return Ok(false);
        } else if stdout.contains("You are disconnected from NordVPN.") {
            return Ok(true);
        }

        Err(CliError::BadOutput(command))
    }

    pub fn groups() -> CliResult<Vec<String>> {
        let (command, output, stdout) = Self::command(["nordvpn", "groups"])?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let groups = match cli_re::parse_list(&stdout) {
            Some(groups) => groups,
            None => return Err(CliError::BadOutput(command)),
        };

        Ok(groups)
    }

    pub fn login() -> CliResult<Option<String>> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(cli_re::login::URL).unwrap());

        let (command, output, stdout) = Self::command(["nordvpn", "login"])?;

        if stdout.contains("You are already logged in.") {
            return Ok(None);
        } else if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let capture = match RE.captures(&stdout) {
            Some(captures) => captures.get(1).unwrap().as_str().to_owned(),
            None => return Err(CliError::BadOutput(command)),
        };

        Ok(Some(capture))
    }

    pub fn logout() -> CliResult<bool> {
        let (command, output, stdout) = Self::command(["nordvpn", "logout"])?;

        if stdout.contains("You are not logged in.") {
            return Ok(false);
        } else if stdout.contains("You are logged out.") {
            return Ok(true);
        } else if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        Err(CliError::BadOutput(command))
    }

    pub fn rate() -> CliResult<()> {
        todo!();
    }

    pub fn register() -> CliResult<()> {
        todo!();
    }

    // pub fn set() {}

    pub fn settings() -> CliResult<()> {
        todo!();
    }

    pub fn status() -> CliResult<Option<Status>> {
        static RE_HOSTNAME: Lazy<Regex> =
            Lazy::new(|| Regex::new(cli_re::status::HOSTNAME).unwrap());
        static RE_COUNTRY: Lazy<Regex> = Lazy::new(|| Regex::new(cli_re::status::COUNTRY).unwrap());
        static RE_CITY: Lazy<Regex> = Lazy::new(|| Regex::new(cli_re::status::CITY).unwrap());
        static RE_IP: Lazy<Regex> = Lazy::new(|| Regex::new(cli_re::status::IP).unwrap());
        static RE_TECHNOLOGY: Lazy<Regex> =
            Lazy::new(|| Regex::new(cli_re::status::TECHNOLOGY).unwrap());
        static RE_PROTOCOL: Lazy<Regex> =
            Lazy::new(|| Regex::new(cli_re::status::PROTOCOL).unwrap());
        static RE_TRANSFER: Lazy<Regex> =
            Lazy::new(|| Regex::new(cli_re::status::TRANSFER).unwrap());
        static RE_UPTIME: Lazy<Regex> = Lazy::new(|| Regex::new(cli_re::status::UPTIME).unwrap());

        let (command, output, stdout) = Self::command(["nordvpn", "status"])?;

        if stdout.contains("Disconnected") {
            return Ok(None);
        } else if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let status = Status {
            hostname: if let Some(captures) = RE_HOSTNAME.captures(&stdout) {
                captures.get(1).unwrap().as_str().to_owned()
            } else {
                return Err(CliError::BadOutput(command));
            },
            country: if let Some(captures) = RE_COUNTRY.captures(&stdout) {
                captures.get(1).unwrap().as_str().to_owned()
            } else {
                return Err(CliError::BadOutput(command));
            },
            city: if let Some(captures) = RE_CITY.captures(&stdout) {
                captures.get(1).unwrap().as_str().to_owned()
            } else {
                return Err(CliError::BadOutput(command));
            },
            ip: if let Some(captures) = RE_IP.captures(&stdout) {
                captures.get(1).unwrap().as_str().parse::<IpAddr>().unwrap()
            } else {
                return Err(CliError::BadOutput(command));
            },
            technology: if let Some(captures) = RE_TECHNOLOGY.captures(&stdout) {
                captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<Technology>()
                    .unwrap()
            } else {
                return Err(CliError::BadOutput(command));
            },
            protocol: if let Some(captures) = RE_PROTOCOL.captures(&stdout) {
                captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<Protocol>()
                    .unwrap()
            } else {
                return Err(CliError::BadOutput(command));
            },
            transfer: if let Some(captures) = RE_TRANSFER.captures(&stdout) {
                Transfer {
                    recieved: Byte::from_str(captures.get(1).unwrap().as_str()).unwrap(),
                    sent: Byte::from_str(captures.get(2).unwrap().as_str()).unwrap(),
                }
            } else {
                return Err(CliError::BadOutput(command));
            },
            uptime: if let Some(captures) = RE_UPTIME.captures(&stdout) {
                let years = captures
                    .name("years")
                    .map_or(0_f64, |value| value.as_str().parse::<f64>().unwrap());
                let months = captures
                    .name("months")
                    .map_or(0_f64, |value| value.as_str().parse::<f64>().unwrap());
                let days = captures
                    .name("days")
                    .map_or(0_f64, |value| value.as_str().parse::<f64>().unwrap());
                let hours = captures
                    .name("hours")
                    .map_or(0_f64, |value| value.as_str().parse::<f64>().unwrap());
                let minutes = captures
                    .name("minutes")
                    .map_or(0_f64, |value| value.as_str().parse::<f64>().unwrap());
                let seconds = captures
                    .name("seconds")
                    .map_or(0_f64, |value| value.as_str().parse::<f64>().unwrap());

                Duration::milliseconds(
                    (100_f64
                        * (seconds
                            + minutes * 60_f64
                            + hours * 3600_f64
                            + days * 86400_f64
                            + months * (2.628_f64 * 10_f64.powi(6))
                            + years * (3.154_f64 * 10_f64.powi(7))))
                    .round() as i64,
                )
            } else {
                return Err(CliError::BadOutput(command));
            },
        };

        Ok(Some(status))
    }

    pub fn whitelist() -> CliResult<()> {
        todo!();
    }

    pub fn version() -> CliResult<Version> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(cli_re::version::VERSION).unwrap());

        let (command, output, stdout) = Self::command(["nordvpn", "version"])?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let capture = match RE.captures(&stdout) {
            Some(captures) => captures.get(1).unwrap().as_str().to_owned(),
            None => return Err(CliError::BadOutput(command)),
        };

        Ok(Version::parse(&capture)?)
    }

    fn command<'a, I>(run: I) -> CliResult<(Command, Output, String)>
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut run = run.into_iter();
        let mut command = Command::new(run.next().unwrap());

        command.args(run);

        let output = command.output()?;
        let stdout = String::from_utf8(output.stdout.clone())?;

        Ok((command, output, stdout.clone()))
    }
}

#[cfg(test)]
mod tests {
    use crate::nordvpn::*;
    use semver::Version;

    #[test]
    fn test_nordvpn() {
        let version = NordVPN::version().unwrap();
        println!("Version: {}", version);
        assert!(version >= Version::new(3, 12, 0));

        let account = NordVPN::account().unwrap();
        println!("Account: {:#?}", account);

        let countries = NordVPN::countries().unwrap();
        println!("Countries: {:?}", countries);

        for country in countries {
            let cities = NordVPN::cities(&country).unwrap();
            println!("Cities in {}: {:?}", country, cities);
        }

        let status = NordVPN::status();

        match status {
            Ok(status) => println!("{:#?}", status.unwrap()),
            Err(error) => match error {
                CliError::BadOutput(mut command) => {
                    println!(
                        "{}",
                        String::from_utf8(command.output().unwrap().stdout).unwrap()
                    );
                }
                _ => panic!(),
            },
        }
    }
}
