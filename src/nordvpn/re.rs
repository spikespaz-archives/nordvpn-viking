use once_cell::sync::Lazy;
use regex::Regex;
use strings::*;

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

pub static WORD_LIST: Lazy<Regex> = Lazy::new(|| Regex::new(strings::WORD_LIST).unwrap());

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

mod strings {
    pub const WORD_LIST: &str = r#"(\w+)(?:,\s*|\s*$)"#;

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
}

pub fn parse_list(text: &str) -> Option<Vec<String>> {
    let mut captures = WORD_LIST.captures_iter(text).peekable();

    captures.peek()?;

    let items = captures.map(|capture| capture.get(1).unwrap().as_str().to_owned());

    Some(items.collect())
}
