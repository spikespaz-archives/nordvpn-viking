use chrono::NaiveDate;
use once_cell::sync::Lazy;
use regex::Regex;
use std::process::Command;

type CliResult<T> = Result<T, Box<dyn std::error::Error>>;

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

static RE_LIST: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(\w+)(?:,\s*|\s*$)"#).unwrap());

pub struct NordVPN;

impl NordVPN {
    pub fn account() -> CliResult<Account> {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r#"Email Address:\s+(.+)\s+VPN Service:\s+(\w+)\s+\(Expires on\s+(\w{3})\s+(\d+)(?:st|nd|rd|th),\s+(\d{4})\)"#
            ).unwrap()
        });

        let output = Command::new("nordvpn").arg("account").output()?;

        // if !output.status.success() {
        //     return Err(std::error::Error(output.status));
        // }

        let output = std::str::from_utf8(&output.stdout)?;
        let captures = RE.captures(output).unwrap();
        let expires = format!(
            "{}-{:02}-{}",
            captures.get(3).unwrap().as_str(),
            captures.get(4).unwrap().as_str(),
            captures.get(5).unwrap().as_str()
        );

        let account = Account {
            email: captures.get(1).unwrap().as_str().to_owned(),
            active: captures.get(2).unwrap().as_str() == "Active",
            expires: NaiveDate::parse_from_str(&expires, "%b-%d-%Y")?,
        };

        Ok(account)
    }

    pub fn cities(country: &str) -> CliResult<Vec<String>> {
        let output = Command::new("nordvpn")
            .arg("cities")
            .arg(country)
            .output()?;

        // if !output.status.success() {
        //     return Err(std::error::Error(output.status));
        // }

        let output = std::str::from_utf8(&output.stdout)?;
        let captures = RE_LIST
            .captures_iter(output)
            .map(|capture| capture[1].to_owned());

        Ok(captures.collect())
    }

    pub fn connect(option: &ConnectOption) -> CliResult<()> {
        todo!();
    }

    pub fn countries() -> CliResult<Vec<String>> {
        let output = Command::new("nordvpn").arg("countries").output()?;

        // if !output.status.success() {
        //     return Err(std::error::Error(output.status));
        // }

        let output = std::str::from_utf8(&output.stdout)?;
        let captures = RE_LIST
            .captures_iter(output)
            .map(|capture| capture[1].to_owned());

        Ok(captures.collect())
    }

    pub fn disconnect() -> CliResult<()> {
        todo!();
    }

    pub fn groups() -> CliResult<Vec<String>> {
        todo!();
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

        let output = Command::new("nordvpn").arg("version").output()?;

        // if !output.status.success() {
        //     return Err(std::error::Error(output.status));
        // }

        let output = std::str::from_utf8(&output.stdout)?;

        let capture = RE
            .captures(output)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_owned();

        Ok(capture)
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
