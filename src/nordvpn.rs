use chrono::NaiveDate;
use once_cell::sync::Lazy;
use regex::Regex;
use std::process::Command;
use thiserror::Error;

type CliResult<T> = Result<T, CliError>;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("unable to create command")]
    IoError(#[from] std::io::Error),
    #[error("command terminated unsuccessfully")]
    FailedCommand(Command),
    #[error("failed to get command output as UTF-8")]
    BadEncoding(#[from] std::str::Utf8Error),
    #[error("command output did not match as expected")]
    NoMatch(Command),
    #[error("failed to parse string as `NaiveDate`")]
    BadDateFormat(#[from] chrono::ParseError),
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

pub struct Connected {
    pub country: String,
    pub server: u32,
    pub hostname: String,
}

pub struct NordVPN;

impl NordVPN {
    pub fn account() -> CliResult<Option<Account>> {
        static RE_EMAIL: Lazy<Regex> =
            Lazy::new(|| Regex::new(r#"Email Address:\s+(.+)\s+"#).unwrap());

        static RE_ACTIVE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r#"VPN Service:\s+(\w+)\s+"#).unwrap());

        static RE_EXPIRES: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"\(Expires on\s+(\w{3})\s+(\d+)(?:st|nd|rd|th),\s+(\d{4})\)"#).unwrap()
        });

        let mut command = Command::new("nordvpn");
        let output = command.arg("account").output()?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let output = std::str::from_utf8(&output.stdout)?;

        if output.contains("You are not logged in.") {
            return Ok(None);
        }

        let account = {
            let email: String;
            let active: bool;
            let expires: NaiveDate;

            if let Some(captures) = RE_EMAIL.captures(output) {
                email = captures.get(1).unwrap().as_str().to_owned();
            } else {
                return Err(CliError::NoMatch(command));
            }

            if let Some(captures) = RE_ACTIVE.captures(output) {
                active = captures.get(1).unwrap().as_str() == "Active";
            } else {
                return Err(CliError::NoMatch(command));
            }

            if let Some(captures) = RE_EXPIRES.captures(output) {
                let date = format!(
                    "{}-{:02}-{}",
                    captures.get(1).unwrap().as_str(),
                    captures.get(2).unwrap().as_str(),
                    captures.get(3).unwrap().as_str()
                );

                expires = NaiveDate::parse_from_str(&date, "%b-%d-%Y")?;
            } else {
                return Err(CliError::NoMatch(command));
            }

            Account {
                email,
                active,
                expires,
            }
        };

        Ok(Some(account))
    }

    pub fn cities(country: &str) -> CliResult<Vec<String>> {
        let mut command = Command::new("nordvpn");
        let output = command.arg("cities").arg(country).output()?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let output = std::str::from_utf8(&output.stdout)?;

        Ok(Self::parse_list(output))
    }

    pub fn connect(option: &ConnectOption) -> CliResult<Connected> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"You are connected to\s+([\w ]+)\s+#(\d+)\s+\(([\w\d\.]+)\)!"#).unwrap()
        });

        let mut command = Command::new("nordvpn");

        command.arg("connect");

        match option {
            ConnectOption::Country(country) => command.arg(country),
            ConnectOption::Server(server) => command.arg(server),
            ConnectOption::CountryCode(country_code) => command.arg(country_code),
            ConnectOption::City(city) => command.arg(city),
            ConnectOption::Group(group) => command.arg(group),
            ConnectOption::CountryCity(country, city) => command.arg(country).arg(city),
        };

        let output = command.output()?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let output = std::str::from_utf8(&output.stdout)?;

        let connected = match RE.captures(output) {
            Some(captures) => Connected {
                country: captures.get(1).unwrap().as_str().to_owned(),
                server: captures.get(2).unwrap().as_str().parse().unwrap(),
                hostname: captures.get(3).unwrap().as_str().to_owned(),
            },
            None => return Err(CliError::NoMatch(command)),
        };

        Ok(connected)
    }

    pub fn countries() -> CliResult<Vec<String>> {
        let mut command = Command::new("nordvpn");
        let output = command.arg("countries").output()?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let output = std::str::from_utf8(&output.stdout)?;

        Ok(Self::parse_list(output))
    }

    pub fn disconnect() -> CliResult<()> {
        todo!();
    }

    pub fn groups() -> CliResult<Vec<String>> {
        let mut command = Command::new("nordvpn");
        let output = command.arg("groups").output()?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let output = std::str::from_utf8(&output.stdout)?;

        Ok(Self::parse_list(output))
    }

    pub fn login() -> CliResult<()> {
        todo!();
    }

    pub fn logout() -> CliResult<()> {
        todo!();
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

    pub fn status() -> CliResult<()> {
        todo!();
    }

    pub fn whitelist() -> CliResult<()> {
        todo!();
    }

    pub fn version() -> CliResult<String> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(\d+\.\d+.\d+)\s+$"#).unwrap());

        let mut command = Command::new("nordvpn");
        let output = command.arg("version").output()?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let output = std::str::from_utf8(&output.stdout)?;

        let capture = match RE.captures(output) {
            Some(captures) => captures.get(1).unwrap().as_str().to_owned(),
            None => return Err(CliError::NoMatch(command)),
        };

        Ok(capture)
    }

    fn parse_list(output: &str) -> Vec<String> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(\w+)(?:,\s*|\s*$)"#).unwrap());

        let captures = RE
            .captures_iter(output)
            .map(|capture| capture[1].to_owned());

        captures.collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::nordvpn::*;

    #[test]
    fn test_nordvpn_account() {
        let result = NordVPN::account().unwrap();
        println!("{:#?}", result);
    }

    // #[test]
    // fn test_nordvpn_version() {
    //     let result = NordVPN::version().unwrap();
    //     println!("{}", result);
    // }
}
