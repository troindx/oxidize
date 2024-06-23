use config::{Config, Environment};
use serde::Deserialize;
use dotenv::dotenv;

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

    }
}
