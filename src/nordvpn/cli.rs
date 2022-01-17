use super::re::{self, RegexError};
use byte_unit::Byte;
use chrono::{Duration, NaiveDate};
use semver::Version;
use std::ffi::OsStr;
use std::net::IpAddr;
use std::process::{Command, Output};
use strum;
use thiserror::Error;

pub type CliResult<T> = Result<T, CliError>;

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
    #[error("setting does not exist")]
    InvalidSettingName(String),
    #[error("the provided value for a setting is malformed or invalid")]
    InvalidSettingValue(String, Vec<String>),
}

#[derive(Debug, Clone)]
pub struct Account {
    pub email: String,
    pub active: bool,
    pub expires: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct Connected {
    pub country: String,
    pub server: u32,
    pub hostname: String,
}

#[derive(Debug, Clone)]
pub enum ConnectOption {
    Country(String),
    Server(String),
    CountryCode(String),
    City(String),
    Group(String),
    CountryCity(String, String),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone, strum::Display, strum::EnumString)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Technology {
    OpenVpn,
    NordLynx,
}

#[derive(Debug, Copy, Clone, strum::Display, strum::EnumString)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Debug, Copy, Clone)]
pub struct Transfer {
    pub received: Byte,
    pub sent: Byte,
}

pub fn account() -> CliResult<Option<Account>> {
    let (command, output, stdout) = command(["nordvpn", "account"])?;

    if stdout.contains("You are not logged in.") {
        return Ok(None);
    } else if !output.status.success() {
        return Err(CliError::FailedCommand(command));
    }

    let captures = match re::ACCOUNT.captures(&stdout) {
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
                    captures.name("expires_year")?.as_str(),
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
    let (command, output, stdout) = command(["nordvpn", "cities", country])?;

    if !output.status.success() {
        return Err(CliError::FailedCommand(command));
    }

    let cities = match re::parse_list(&stdout) {
        Some(cities) => cities,
        None => return Err(CliError::RegexError(RegexError::Cities, command)),
    };

    Ok(cities)
}

pub fn connect(option: Option<&ConnectOption>) -> CliResult<Connected> {
    let mut run = vec!["nordvpn", "connect"];

    if let Some(option) = option {
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
    }

    let (command, output, stdout) = command(run)?;

    if !output.status.success() {
        return Err(CliError::FailedCommand(command));
    }

    let captures = match re::CONNECT.captures(&stdout) {
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
    let (command, output, stdout) = command(["nordvpn", "countries"])?;

    if !output.status.success() {
        return Err(CliError::FailedCommand(command));
    }

    let countries = match re::parse_list(&stdout) {
        Some(countries) => countries,
        None => return Err(CliError::RegexError(RegexError::Countries, command)),
    };

    Ok(countries)
}

pub fn disconnect() -> CliResult<bool> {
    let (command, output, stdout) = command(["nordvpn", "disconnect"])?;

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
    let (command, output, stdout) = command(["nordvpn", "groups"])?;

    if !output.status.success() {
        return Err(CliError::FailedCommand(command));
    }

    let groups = match re::parse_list(&stdout) {
        Some(groups) => groups,
        None => return Err(CliError::RegexError(RegexError::Groups, command)),
    };

    Ok(groups)
}

pub fn login() -> CliResult<Option<String>> {
    let (command, output, stdout) = command(["nordvpn", "login"])?;

    if stdout.contains("You are already logged in.") {
        return Ok(None);
    } else if !output.status.success() {
        return Err(CliError::FailedCommand(command));
    }

    let captures = match re::LOGIN.captures(&stdout) {
        Some(captures) => captures,
        None => return Err(CliError::RegexError(RegexError::Login, command)),
    };

    let url = captures.name("url").unwrap().as_str().to_owned();

    Ok(Some(url))
}

pub fn logout() -> CliResult<bool> {
    let (command, output, stdout) = command(["nordvpn", "logout"])?;

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

#[deprecated(note = "please use `Settings` methods instead")]
pub fn set<S, T, V>(setting: S, values: V) -> CliResult<()>
where
    S: AsRef<str>,
    T: AsRef<str>,
    V: IntoIterator<Item = T>,
{
    let values = values.into_iter().collect::<Vec<_>>();
    let (command, output, stdout) = command(
        ["nordvpn", "set"]
            .into_iter()
            .chain(values.iter().map(AsRef::as_ref)),
    )?;

    if stdout.contains("The command you entered is not valid.") {
        return Err(CliError::InvalidSettingValue(
            setting.as_ref().to_owned(),
            values
                .iter()
                .map(|value| value.as_ref().to_owned())
                .collect(),
        ));
    } else if re::INVALID_SETTING.is_match(&stdout) {
        return Err(CliError::InvalidSettingName(setting.as_ref().to_owned()));
    } else if !output.status.success() {
        return Err(CliError::BadOutput(command));
    }

    Ok(())
}

pub fn settings() -> CliResult<()> {
    todo!();
}

pub fn status() -> CliResult<Option<Status>> {
    let (command, output, stdout) = command(["nordvpn", "status"])?;

    if stdout.contains("Disconnected") {
        return Ok(None);
    } else if !output.status.success() {
        return Err(CliError::FailedCommand(command));
    }

    let captures = match re::STATUS.captures(&stdout) {
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
    let (command, output, stdout) = command(["nordvpn", "version"])?;

    if !output.status.success() {
        return Err(CliError::FailedCommand(command));
    }

    let captures = match re::VERSION.captures(&stdout) {
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

#[derive(Debug, Clone)]
pub struct Settings {
    pub technology: Technology,
    pub protocol: Protocol,
    pub firewall: bool,
    pub killswitch: bool,
    pub cybersec: bool,
    pub obfuscate: Option<bool>,
    pub notify: bool,
    pub autoconnect: bool,
    pub ipv6: bool,
    pub dns: Option<Vec<IpAddr>>,
}

#[allow(deprecated)]
impl Settings {
    pub fn update(&mut self) -> CliResult<()> {
        self.set_technology(self.technology)?;
        self.set_protocol(self.protocol)?;
        self.set_firewall(self.firewall)?;
        self.set_killswitch(self.killswitch)?;
        self.set_cybersec(self.cybersec)?;
        self.set_obfuscate(self.obfuscate)?;
        self.set_notify(self.notify)?;
        self.set_autoconnect(self.autoconnect)?;
        self.set_ipv6(self.ipv6)?;
        self.set_dns(self.dns.clone())?;

        Ok(())
    }

    pub fn set_technology(&mut self, technology: Technology) -> CliResult<&mut Self> {
        set("technology", [technology.to_string().as_str()])?;
        self.technology = technology;
        Ok(self)
    }

    pub fn set_protocol(&mut self, protocol: Protocol) -> CliResult<&mut Self> {
        set("protocol", [protocol.to_string().as_str()])?;
        self.protocol = protocol;
        Ok(self)
    }

    pub fn set_firewall(&mut self, enabled: bool) -> CliResult<&mut Self> {
        set("firewall", [enabled.to_string().as_str()])?;
        self.firewall = enabled;
        Ok(self)
    }

    pub fn set_killswitch(&mut self, enabled: bool) -> CliResult<&mut Self> {
        set("killswitch", [enabled.to_string().as_str()])?;
        self.killswitch = enabled;
        Ok(self)
    }

    pub fn set_cybersec(&mut self, enabled: bool) -> CliResult<&mut Self> {
        set("cybersec", [enabled.to_string().as_str()])?;
        self.cybersec = enabled;
        Ok(self)
    }

    pub fn set_obfuscate(&mut self, enabled: Option<bool>) -> CliResult<&mut Self> {
        if let Some(enabled) = enabled {
            set("obfuscate", [enabled.to_string().as_str()])?;
        } else {
            set("obfuscate", ["false"])?;
        }
        self.obfuscate = enabled;
        Ok(self)
    }

    pub fn set_notify(&mut self, enabled: bool) -> CliResult<&mut Self> {
        set("notify", [enabled.to_string().as_str()])?;
        self.notify = enabled;
        Ok(self)
    }

    pub fn set_autoconnect(&mut self, enabled: bool) -> CliResult<&mut Self> {
        set("autoconnect", [enabled.to_string().as_str()])?;
        self.autoconnect = enabled;
        Ok(self)
    }

    pub fn set_ipv6(&mut self, enabled: bool) -> CliResult<&mut Self> {
        set("ipv6", [enabled.to_string().as_str()])?;
        self.ipv6 = enabled;
        Ok(self)
    }

    pub fn set_dns<V>(&mut self, addresses: Option<V>) -> CliResult<&mut Self>
    where
        V: IntoIterator<Item = IpAddr>,
    {
        if let Some(addresses) = addresses {
            let addresses = addresses.into_iter().collect::<Vec<_>>();
            set(
                "dns",
                addresses
                    .clone()
                    .into_iter()
                    .map(|address| address.to_string()),
            )?;
            self.dns = Some(addresses);
        } else {
            set("dns", ["false"])?;
            self.dns = None;
        }
        Ok(self)
    }
}

fn command<S, I>(run: I) -> CliResult<(Command, Output, String)>
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
{
    let mut run = run.into_iter();
    let mut command = Command::new(run.next().unwrap());

    command.args(run);

    let output = command.output()?;
    let stdout = String::from_utf8(output.stdout.clone())?;

    Ok((command, output, stdout))
}
