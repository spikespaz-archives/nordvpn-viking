mod cli;
mod re;

pub use cli::*;
pub use re::RegexError;

#[cfg(test)]
mod tests {
    use semver::Version;

    #[test]
    fn test_nordvpn() {
        let version = super::version().unwrap();
        println!("Version: {}", version);
        assert!(version >= Version::new(3, 12, 0));

        let account = super::account().unwrap();
        println!("Account: {:#?}", account);

        let countries = super::countries().unwrap();
        println!("Countries: {:?}", countries);

        for country in countries {
            let cities = super::cities(&country).unwrap();
            println!("Cities in {}: {:?}", country, cities);
        }

        let groups = super::groups().unwrap();
        println!("Groups: {:?}", groups);

        let status = super::status().unwrap();
        println!("Status: {:#?}", status);

        let connect = super::connect(None).unwrap();
        println!("Connect response: {:#?}", connect);

        let status = super::status().unwrap();
        println!("Status: {:#?}", status);

        let disconnect = super::disconnect().unwrap();
        println!("Disconnected: {}", disconnect);

        let status = super::status().unwrap();
        println!("Status: {:#?}", status);
    }
}
