use chrono::NaiveDate;
use once_cell::sync::Lazy;
use regex::Regex;
use semver::Version;
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
        let stdout = std::str::from_utf8(&output.stdout)?;

        if stdout.contains("You are not logged in.") {
            return Ok(None);
        } else if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let account = {
            let email: String;
            let active: bool;
            let expires: NaiveDate;

            if let Some(captures) = RE_EMAIL.captures(stdout) {
                email = captures.get(1).unwrap().as_str().to_owned();
            } else {
                return Err(CliError::BadOutput(command));
            }

            if let Some(captures) = RE_ACTIVE.captures(stdout) {
                active = captures.get(1).unwrap().as_str() == "Active";
            } else {
                return Err(CliError::BadOutput(command));
            }

            if let Some(captures) = RE_EXPIRES.captures(stdout) {
                let date = format!(
                    "{}-{:02}-{}",
                    captures.get(1).unwrap().as_str(),
                    captures.get(2).unwrap().as_str(),
                    captures.get(3).unwrap().as_str()
                );

                expires = NaiveDate::parse_from_str(&date, "%b-%d-%Y")?;
            } else {
                return Err(CliError::BadOutput(command));
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
        let stdout = std::str::from_utf8(&output.stdout)?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let cities = match Self::parse_list(stdout) {
            Some(cities) => cities,
            None => return Err(CliError::BadOutput(command)),
        };

        Ok(cities)
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
        let stdout = std::str::from_utf8(&output.stdout)?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let connected = match RE.captures(stdout) {
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
        let mut command = Command::new("nordvpn");
        let output = command.arg("countries").output()?;
        let stdout = std::str::from_utf8(&output.stdout)?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let countries = match Self::parse_list(stdout) {
            Some(countries) => countries,
            None => return Err(CliError::BadOutput(command)),
        };

        Ok(countries)
    }

    pub fn disconnect() -> CliResult<bool> {
        let mut command = Command::new("nordvpn");
        let output = command.arg("disconnect").output()?;
        let stdout = std::str::from_utf8(&output.stdout)?;

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
        let mut command = Command::new("nordvpn");
        let output = command.arg("groups").output()?;
        let stdout = std::str::from_utf8(&output.stdout)?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let groups = match Self::parse_list(stdout) {
            Some(groups) => groups,
            None => return Err(CliError::BadOutput(command)),
        };

        Ok(groups)
    }

    pub fn login() -> CliResult<Option<String>> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r#"Continue in the browser:\s+(.+)\s*$"#).unwrap());

        let mut command = Command::new("nordvpn");
        let output = command.arg("login").output()?;
        let stdout = std::str::from_utf8(&output.stdout)?;

        if stdout.contains("You are already logged in.") {
            return Ok(None);
        } else if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let capture = match RE.captures(stdout) {
            Some(captures) => captures.get(1).unwrap().as_str().to_owned(),
            None => return Err(CliError::BadOutput(command)),
        };

        Ok(Some(capture))
    }

    pub fn logout() -> CliResult<bool> {
        let mut command = Command::new("nordvpn");
        let output = command.arg("login").output()?;
        let stdout = std::str::from_utf8(&output.stdout)?;

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

    pub fn status() -> CliResult<()> {
        todo!();
    }

    pub fn whitelist() -> CliResult<()> {
        todo!();
    }

    pub fn version() -> CliResult<Version> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(\d+\.\d+.\d+)\s+$"#).unwrap());

        let mut command = Command::new("nordvpn");
        let output = command.arg("version").output()?;
        let stdout = std::str::from_utf8(&output.stdout)?;

        if !output.status.success() {
            return Err(CliError::FailedCommand(command));
        }

        let capture = match RE.captures(stdout) {
            Some(captures) => captures.get(1).unwrap().as_str().to_owned(),
            None => return Err(CliError::BadOutput(command)),
        };

        Ok(Version::parse(&capture)?)
    }

    fn parse_list(text: &str) -> Option<Vec<String>> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(\w+)(?:,\s*|\s*$)"#).unwrap());

        let mut captures = RE.captures_iter(text).map(|capture| capture).peekable();

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
    }
}
