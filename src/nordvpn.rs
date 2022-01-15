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
    pub received: Byte,
    pub sent: Byte,
}

mod cli_re {
    use once_cell::sync::Lazy;
    use regex::Regex;

    pub mod re {
        use super::*;

        pub static WORD_LIST: Lazy<Regex> = Lazy::new(|| Regex::new(generic::WORD_LIST).unwrap());

        pub static ACCOUNT: Lazy<Regex> = Lazy::new(|| {
            Regex::new(&format!(
                r#"(?:{}|{}|{})+"#,
                account::EMAIL,
                account::ACTIVE,
                account::EXPIRES
            ))
            .unwrap()
        });
        pub static CONNECT: Lazy<Regex> =
            Lazy::new(|| Regex::new(connect::COUNTRY_SERVER_HOSTNAME).unwrap());
        pub static LOGIN: Lazy<Regex> = Lazy::new(|| Regex::new(login::URL).unwrap());
        pub static STATUS: Lazy<Regex> = Lazy::new(|| {
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
        pub static VERSION: Lazy<Regex> = Lazy::new(|| Regex::new(version::VERSION).unwrap());
    }

    pub mod account {
        pub static EMAIL: &str = r#"Email Address:\s+(?P<email>.+)\s*(?:\n|$)"#;
        pub static ACTIVE: &str = r#"VPN Service:\s+(?P<active>(?i)[a-z]+)\s*"#;
        pub static EXPIRES: &str = r#"\(Expires on\s+(?P<expires_month>(?i)[a-z]{3})\s+(?P<expires_day>\d+)(?i:st|nd|rd|th),\s+(?P<expires_year>\d{4})\)"#;
    }

    pub mod connect {
        pub static COUNTRY_SERVER_HOSTNAME: &str = r#"You are connected to\s+(?P<country>(?i)[a-z_ ]+)\s+#(?P<server>\d+)\s+\((?P<hostname>[\w\d\-\.]+)\)!"#;
    }

    pub mod login {
        pub static URL: &str = r#"Continue in the browser:\s+(?P<url>.+)\s*(?:\n|$)"#;
    }

    pub mod status {
        pub static HOSTNAME: &str = r#"Current server:\s+(?P<hostname>[\w\d\-\.]+)\s*(?:\n|$)"#;
        pub static COUNTRY: &str = r#"Country:\s+(?P<country>(?i)[a-z_ ]+[a-z_ ])\s*(?:\n|$)"#;
        pub static CITY: &str = r#"City:\s+(?P<city>(?i)[a-z_ ]+[a-z_ ])\s*(?:\n|$)"#;
        pub static IP: &str = r#"Server IP:\s+(?P<ip>(?i)(?:[\da-f]{0,4}:){1,7}[\da-f]{0,4}|(?:\d{1,3}\.){3}\d{1,3})\s*(?:\n|$)"#;
        pub static TECHNOLOGY: &str =
            r#"Current technology:\s+(?P<technology>(?i)OPENVPN|NORDLYNX)\s*(?:\n|$)"#;
        pub static PROTOCOL: &str = r#"Current protocol:\s+(?P<protocol>(?i)TCP|UDP)\s*(?:\n|$)"#;
        pub static TRANSFER: &str = r#"Transfer:\s+(?i:(?P<transfer_received>(?:\d+\.)?\d+\s+[a-z]+)\s+received,\s+(?P<transfer_sent>(?:\d+\.)?\d+\s+[a-z]+)\s+sent)\s*(?:\n|$)"#;
        pub static UPTIME: &str = r#"Uptime:\s+(?i:(?:(?P<uptime_years>\d+)\s+years?\s*)?(?:(?P<uptime_months>\d+)\s+months?\s*)?(?:(?P<uptime_days>\d+)\s+days?\s*)?(?:(?P<uptime_hours>\d+)\s+hours?\s*)?(?:(?P<uptime_minutes>\d+)\s+minutes?\s*)?(?:(?P<uptime_seconds>\d+)\s+seconds?\s*)?)\s*(?:\n|$)"#;
    }

    pub mod version {
        pub static VERSION: &str = r#"(?P<version>\d+\.\d+.\d+)\s*(?:\n|$)"#;
    }

    pub mod generic {
        pub static WORD_LIST: &str = r#"(\w+)(?:,\s*|\s*$)"#;
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
        let (command, output, stdout) = Self::command(["nordvpn", "account"])?;

        if stdout.contains("You are not logged in.") {
            return Ok(None);
        } else if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let captures = match cli_re::re::ACCOUNT.captures(&stdout) {
            Some(captures) => captures,
            None => return Err(CliError::BadOutput(command)),
        };

        let account = Account {
            email: match captures.name("email") {
                Some(email) => email.as_str().to_owned(),
                None => return Err(CliError::BadOutput(command)),
            },
            active: match captures.name("active") {
                Some(active) => active.as_str().to_lowercase() == "active",
                None => return Err(CliError::BadOutput(command)),
            },
            expires: NaiveDate::parse_from_str(
                &format!(
                    "{}-{:02}-{}",
                    match captures.name("expires_month") {
                        Some(month) => month.as_str(),
                        None => return Err(CliError::BadOutput(command)),
                    },
                    match captures.name("expires_day") {
                        Some(day) => day.as_str(),
                        None => return Err(CliError::BadOutput(command)),
                    },
                    match captures.name("expires_year") {
                        Some(year) => year.as_str(),
                        None => return Err(CliError::BadOutput(command)),
                    }
                ),
                "%b-%d-%Y",
            )?,
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

        let captures = match cli_re::re::CONNECT.captures(&stdout) {
            Some(captures) => captures,
            None => return Err(CliError::BadOutput(command)),
        };

        let connected = Connected {
            country: match captures.name("country") {
                Some(country) => country.as_str().to_owned(),
                None => return Err(CliError::BadOutput(command)),
            },
            server: match captures.name("server") {
                Some(server) => server.as_str().parse::<u32>().unwrap(),
                None => return Err(CliError::BadOutput(command)),
            },
            hostname: match captures.name("hostname") {
                Some(hostname) => hostname.as_str().to_owned(),
                None => return Err(CliError::BadOutput(command)),
            },
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
        let (command, output, stdout) = Self::command(["nordvpn", "login"])?;

        if stdout.contains("You are already logged in.") {
            return Ok(None);
        } else if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let captures = match cli_re::re::LOGIN.captures(&stdout) {
            Some(captures) => captures,
            None => return Err(CliError::BadOutput(command)),
        };

        let url = match captures.name("url") {
            Some(url) => url.as_str().to_owned(),
            None => return Err(CliError::BadOutput(command)),
        };

        Ok(Some(url))
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
        let (command, output, stdout) = Self::command(["nordvpn", "status"])?;

        if stdout.contains("Disconnected") {
            return Ok(None);
        } else if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let captures = match cli_re::re::STATUS.captures(&stdout) {
            Some(captures) => captures,
            None => return Err(CliError::BadOutput(command)),
        };

        let status = Status {
            hostname: match captures.name("hostname") {
                Some(hostname) => hostname.as_str().to_owned(),
                None => return Err(CliError::BadOutput(command)),
            },
            country: match captures.name("country") {
                Some(country) => country.as_str().to_owned(),
                None => return Err(CliError::BadOutput(command)),
            },
            city: match captures.name("city") {
                Some(city) => city.as_str().to_owned(),
                None => return Err(CliError::BadOutput(command)),
            },
            ip: match captures.name("ip") {
                Some(ip) => ip.as_str().parse::<IpAddr>().unwrap(),
                None => return Err(CliError::BadOutput(command)),
            },
            technology: match captures.name("technology") {
                Some(technology) => technology.as_str().parse::<Technology>().unwrap(),
                None => return Err(CliError::BadOutput(command)),
            },
            protocol: match captures.name("protocol") {
                Some(protocol) => protocol.as_str().parse::<Protocol>().unwrap(),
                None => return Err(CliError::BadOutput(command)),
            },
            transfer: Transfer {
                received: match captures.name("transfer_received") {
                    Some(received) => received.as_str().parse::<Byte>().unwrap(),
                    None => return Err(CliError::BadOutput(command)),
                },
                sent: match captures.name("transfer_sent") {
                    Some(sent) => sent.as_str().parse::<Byte>().unwrap(),
                    None => return Err(CliError::BadOutput(command)),
                },
            },
            uptime: {
                let years = captures
                    .name("uptime_years")
                    .map_or(0_f64, |value| value.as_str().parse::<f64>().unwrap());
                let months = captures
                    .name("uptime_months")
                    .map_or(0_f64, |value| value.as_str().parse::<f64>().unwrap());
                let days = captures
                    .name("uptime_days")
                    .map_or(0_f64, |value| value.as_str().parse::<f64>().unwrap());
                let hours = captures
                    .name("uptime_hours")
                    .map_or(0_f64, |value| value.as_str().parse::<f64>().unwrap());
                let minutes = captures
                    .name("uptime_minutes")
                    .map_or(0_f64, |value| value.as_str().parse::<f64>().unwrap());
                let seconds = captures
                    .name("uptime_seconds")
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
            },
        };

        Ok(Some(status))
    }

    pub fn whitelist() -> CliResult<()> {
        todo!();
    }

    pub fn version() -> CliResult<Version> {
        let (command, output, stdout) = Self::command(["nordvpn", "version"])?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let captures = match cli_re::re::VERSION.captures(&stdout) {
            Some(captures) => captures,
            None => return Err(CliError::BadOutput(command)),
        };

        let version = match captures.name("version") {
            Some(version) => version.as_str().parse::<Version>()?,
            None => return Err(CliError::BadOutput(command)),
        };

        Ok(version)
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
