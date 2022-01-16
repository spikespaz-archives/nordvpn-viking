use once_cell::sync::Lazy;
use regex::Regex;

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

pub static ACCOUNT: Lazy<Regex> = Lazy::new(|| Regex::new(strings::ACCOUNT).unwrap());
pub static CONNECT: Lazy<Regex> =
    Lazy::new(|| Regex::new(strings::connect::COUNTRY_SERVER_HOSTNAME).unwrap());
pub static LOGIN: Lazy<Regex> = Lazy::new(|| Regex::new(strings::login::URL).unwrap());
pub static INVALID_SETTING: Lazy<Regex> =
    Lazy::new(|| Regex::new(strings::settings::INVALID_NAME).unwrap());
pub static STATUS: Lazy<Regex> = Lazy::new(|| Regex::new(strings::STATUS).unwrap());
pub static VERSION: Lazy<Regex> = Lazy::new(|| Regex::new(strings::version::VERSION).unwrap());

mod strings {
    use const_format::*;

    pub const WORD_LIST: &str = r#"(\w+)(?:,\s*|\s*$)"#;

    pub const ACCOUNT: &str = formatcp!(
        r#"(?:{}|{}|{})+"#,
        account::EMAIL,
        account::ACTIVE,
        account::EXPIRES
    );
    pub const STATUS: &str = formatcp!(
        r#"(?:{}|{}|{}|{}|{}|{}|{}|{})+"#,
        status::HOSTNAME,
        status::COUNTRY,
        status::CITY,
        status::IP,
        status::TECHNOLOGY,
        status::PROTOCOL,
        status::TRANSFER,
        status::UPTIME
    );

    pub mod shared {
        pub const LINE_END_OR_NEWLINE: &str = r#"\s*(?:\n|$)"#;
        pub const IPV4_OR_IPV6: &str =
            r#"(?P<GROUP_NAME>(?i)(?:[\da-f]{0,4}:){1,7}[\da-f]{0,4}|(?:\d{1,3}\.){3}\d{1,3})"#;
        pub const OPENVPN_OR_NORDLYNX: &str = r#"(?P<GROUP_NAME>(?i)OPENVPN|NORDLYNX)"#;
        pub const TCP_OR_UDP: &str = r#"(?P<GROUP_NAME>(?i)TCP|UDP)"#;
    }

    pub mod account {
        use super::shared::*;
        use const_format::*;

        pub const EMAIL: &str = concatcp!(r#"Email Address:\s+(?P<email>.+)"#, LINE_END_OR_NEWLINE);
        pub const ACTIVE: &str = r#"VPN Service:\s+(?P<active>(?i)[a-z]+)\s*"#;
        pub const EXPIRES: &str = r#"\(Expires on\s+(?P<expires_month>(?i)[a-z]{3})\s+(?P<expires_day>\d+)(?i:st|nd|rd|th),\s+(?P<expires_year>\d{4})\)"#;
    }

    pub mod connect {
        pub const COUNTRY_SERVER_HOSTNAME: &str = r#"You are connected to\s+(?P<country>(?i)[a-z_ ]+)\s+#(?P<server>\d+)\s+\((?P<hostname>[\w\d\-\.]+)\)!"#;
    }

    pub mod login {
        use super::shared::*;
        use const_format::*;

        pub const URL: &str = concatcp!(
            r#"Continue in the browser:\s+(?P<url>.+)"#,
            LINE_END_OR_NEWLINE
        );
    }

    pub mod settings {
        pub const INVALID_NAME: &str = r#"Command '(?P<name>.+)' doesn't exist."#;
    }

    pub mod status {
        use super::shared::*;
        use const_format::*;

        pub const HOSTNAME: &str = concatcp!(
            r#"Current server:\s+(?P<hostname>[\w\d\-\.]+)"#,
            LINE_END_OR_NEWLINE
        );
        pub const COUNTRY: &str = concatcp!(
            r#"Country:\s+(?P<country>(?i)[a-z_ ]+[a-z_ ])"#,
            LINE_END_OR_NEWLINE
        );
        pub const CITY: &str = concatcp!(
            r#"City:\s+(?P<city>(?i)[a-z_ ]+[a-z_ ])"#,
            LINE_END_OR_NEWLINE
        );
        pub const IP: &str = concatcp!(
            r#"Server IP:\s+"#,
            str_replace!(IPV4_OR_IPV6, "GROUP_NAME", "ip"),
            LINE_END_OR_NEWLINE
        );
        pub const TECHNOLOGY: &str = concatcp!(
            r#"Current technology:\s+"#,
            str_replace!(OPENVPN_OR_NORDLYNX, "GROUP_NAME", "technology"),
            LINE_END_OR_NEWLINE
        );
        pub const PROTOCOL: &str = concatcp!(
            r#"Current protocol:\s+"#,
            str_replace!(TCP_OR_UDP, "GROUP_NAME", "protocol"),
            LINE_END_OR_NEWLINE
        );
        pub const TRANSFER: &str = concatcp!(
            r#"Transfer:\s+(?i:(?P<transfer_received>(?:\d+\.)?\d+\s+[a-z]+)\s+received,\s+(?P<transfer_sent>(?:\d+\.)?\d+\s+[a-z]+)\s+sent)"#,
            LINE_END_OR_NEWLINE
        );
        pub const UPTIME: &str = concatcp!(
            r#"Uptime:\s+(?i:(?:(?P<uptime_years>\d+)\s+years?\s*)?(?:(?P<uptime_months>\d+)\s+months?\s*)?(?:(?P<uptime_days>\d+)\s+days?\s*)?(?:(?P<uptime_hours>\d+)\s+hours?\s*)?(?:(?P<uptime_minutes>\d+)\s+minutes?\s*)?(?:(?P<uptime_seconds>\d+)\s+seconds?\s*)?)"#,
            LINE_END_OR_NEWLINE
        );
    }

    pub mod version {
        use super::shared::*;
        use const_format::*;

        pub const VERSION: &str = concatcp!(r#"(?P<version>\d+\.\d+.\d+)"#, LINE_END_OR_NEWLINE);
    }
}

pub fn parse_list(text: &str) -> Option<Vec<String>> {
    let mut captures = WORD_LIST.captures_iter(text).peekable();

    captures.peek()?;

    let items = captures.map(|capture| capture.get(1).unwrap().as_str().to_owned());

    Some(items.collect())
}
