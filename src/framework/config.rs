use std::str::FromStr;
use config::{Config, Environment};
use rsa::pkcs8::der::zeroize::Zeroizing;
use serde::{Deserialize, Deserializer};
use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct ZeroizedString(Zeroizing<String>);

impl<'de> Deserialize<'de> for ZeroizedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(ZeroizedString(Zeroizing::new(s)))
    }
}

impl FromStr for ZeroizedString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ZeroizedString(Zeroizing::new(s.to_string())))
    }
}

impl std::ops::Deref for ZeroizedString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct OxidizeConfigEnvironment {
    pub mongodb_host: String,
    pub mongodb_port:u16,
    pub default_port:u16,
    pub mongodb_database_name: String,
    pub mongodb_root_username: String,
    pub mongodb_root_pwd: String,
    pub mongo_test_user: String,
    pub mongo_test_password:String,
    pub default_email_verification_key_length:usize,
    pub email_sender_from:String,
    pub email_reply_to:String,
    pub smtp_user:String,
    pub smtp_password:ZeroizedString,
    pub smtp_host:String,
    pub run_mode:String,
}

/// Creates a valid oxidizeConfig 
/// ```
/// use oxidize::framework::config::OxidizeConfig;
/// let config = OxidizeConfig::new();
/// config.is_ok();
/// ```

#[derive(Debug, Deserialize, Clone)]
pub struct OxidizeConfig {
    pub env: OxidizeConfigEnvironment,
}

impl OxidizeConfig{
    pub fn new() -> Result<Self, config::ConfigError> {
        dotenv().ok();
        let config = Config::builder()
            .add_source(Environment::default())
            .build()?
            .try_deserialize()?;
        Ok(Self { env: config })
    }
}

#[cfg(test)]
mod tests{
    use dotenv::var;

    use super::*;

    #[test]
    fn test_load_config() {
        let config = OxidizeConfig::new().expect("Failed to load configuration");
        // Verify that the configuration was loaded correctly
        assert_eq!(config.env.mongodb_host, var("mongodb_host").expect("No MongoDB host found in ENV FILE"));
        assert_eq!(config.env.mongodb_port, var("mongodb_port")
            .expect("No MongoDB port found in ENV FILE").parse::<u16>()
            .expect("mongodb Port is not a number"));
        assert_eq!(config.env.default_port, var("default_port")
            .expect("No default port found in ENV FILE").parse::<u16>()
            .expect("default Port is not a number"));
        assert_eq!(config.env.mongodb_database_name, var("mongodb_database_name").expect("No MongoDB db name found in ENV FILE"));
        assert_eq!(config.env.mongodb_root_username, var("mongodb_root_username").expect("No MongoDB root user name found in ENV FILE"));
        assert_eq!(config.env.mongodb_root_pwd, var("mongodb_root_pwd").expect("No MongoDB root pwd found in ENV FILE"));
        assert_eq!(config.env.mongo_test_user, var("mongo_test_user").expect("No mongodb user name found in ENV FILE"));
        assert_eq!(config.env.mongo_test_password, var("mongo_test_password").expect("No user password for mongodb found in ENV FILE"));
        assert_eq!(config.env.default_email_verification_key_length, var("default_email_verification_key_length")
            .expect("No default email verification key length set in ENV file").parse::<usize>()
            .expect("Email verification key length is not a number"));
        assert_eq!(config.env.smtp_password.to_string(), var("smtp_password").expect("No smtp password found in ENV FILE"));
        assert_eq!(config.env.run_mode.to_string(), var("run_mode").expect("No  run mode found in ENV FILE"));
    }
}
