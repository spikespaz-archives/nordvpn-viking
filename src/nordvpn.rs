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
    #[error("a regex pattern failed to match")]
    RegexError(RegexError, Command),
}

#[derive(Debug)]
pub enum RegexError {
    Account,
    AccountEmail,
    AccountActive,
    AccountExpires,
    Cities,
    Connect,
    Countries,
    Groups,
    Login,
    Status,
    StatusHostname,
    StatusCountry,
    StatusCity,
    StatusIp,
    StatusTechnology,
    StatusProtocol,
    StatusTransfer,
    Version,
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
            None => return Err(CliError::RegexError(RegexError::Account, command)),
        };

        let account = Account {
            email: match captures.name("email") {
                Some(email) => email.as_str().to_owned(),
                None => return Err(CliError::RegexError(RegexError::AccountEmail, command)),
            },
            active: match captures.name("active") {
                Some(active) => active.as_str().to_lowercase() == "active",
                None => return Err(CliError::RegexError(RegexError::AccountActive, command)),
            },
            expires: NaiveDate::parse_from_str(
                &(|| {
                    Some(format!(
                        "{}-{:02}-{}",
                        captures.name("expires_month")?.as_str(),
                        captures.name("expires_day")?.as_str(),
                        captures.name("year")?.as_str(),
                    ))
                })()
                .ok_or(CliError::RegexError(RegexError::AccountExpires, command))?,
                "%b-%d-%Y",
            )
            .unwrap(),
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
            None => return Err(CliError::RegexError(RegexError::Cities, command)),
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
            None => return Err(CliError::RegexError(RegexError::Connect, command)),
        };

        let connected = (|| {
            Some(Connected {
                country: captures.name("country")?.as_str().to_owned(),
                server: captures.name("server")?.as_str().parse::<u32>().unwrap(),
                hostname: captures.name("hostname")?.as_str().to_owned(),
            })
        })()
        .ok_or(CliError::RegexError(RegexError::Connect, command))?;

        Ok(connected)
    }

    pub fn countries() -> CliResult<Vec<String>> {
        let (command, output, stdout) = Self::command(["nordvpn", "countries"])?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let countries = match cli_re::parse_list(&stdout) {
            Some(countries) => countries,
            None => return Err(CliError::RegexError(RegexError::Countries, command)),
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
            None => return Err(CliError::RegexError(RegexError::Groups, command)),
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
            None => return Err(CliError::RegexError(RegexError::Login, command)),
        };

        let url = captures.name("url").unwrap().as_str().to_owned();

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
            None => return Err(CliError::RegexError(RegexError::Status, command)),
        };

        let status = Status {
            hostname: match captures.name("hostname") {
                Some(hostname) => hostname.as_str().to_owned(),
                None => return Err(CliError::RegexError(RegexError::StatusHostname, command)),
            },
            country: match captures.name("country") {
                Some(country) => country.as_str().to_owned(),
                None => return Err(CliError::RegexError(RegexError::StatusCountry, command)),
            },
            city: match captures.name("city") {
                Some(city) => city.as_str().to_owned(),
                None => return Err(CliError::RegexError(RegexError::StatusCity, command)),
            },
            ip: match captures.name("ip") {
                Some(ip) => ip.as_str().parse::<IpAddr>().unwrap(),
                None => return Err(CliError::RegexError(RegexError::StatusIp, command)),
            },
            technology: match captures.name("technology") {
                Some(technology) => technology.as_str().parse::<Technology>().unwrap(),
                None => return Err(CliError::RegexError(RegexError::StatusTechnology, command)),
            },
            protocol: match captures.name("protocol") {
                Some(protocol) => protocol.as_str().parse::<Protocol>().unwrap(),
                None => return Err(CliError::RegexError(RegexError::StatusProtocol, command)),
            },
            transfer: (|| {
                Some(Transfer {
                    received: captures
                        .name("transfer_received")?
                        .as_str()
                        .parse::<Byte>()
                        .unwrap(),
                    sent: captures
                        .name("transfer_sent")?
                        .as_str()
                        .parse::<Byte>()
                        .unwrap(),
                })
            })()
            .ok_or(CliError::RegexError(RegexError::StatusTransfer, command))?,
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
            None => return Err(CliError::RegexError(RegexError::Version, command)),
        };

        let version = captures
            .name("version")
            .unwrap()
            .as_str()
            .parse::<Version>()
            .unwrap();

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
        println!("{:#?}", status);
    }
}
