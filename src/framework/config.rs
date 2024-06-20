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
}
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
        assert_eq!(config.env.mongodb_host, var("MONGODB_HOST").expect("No MongoDB host found in ENV FILE"));
        assert_eq!(config.env.mongodb_port, var("MONGODB_PORT")
            .expect("No MongoDB port found in ENV FILE").parse::<u16>()
            .expect("mongodb Port is not a number"));
        assert_eq!(config.env.default_port, var("DEFAULT_PORT")
            .expect("No default port found in ENV FILE").parse::<u16>()
            .expect("default Port is not a number"));
        assert_eq!(config.env.mongodb_database_name, var("MONGODB_DATABASE_NAME").expect("No MongoDB db name found in ENV FILE"));
        assert_eq!(config.env.mongodb_root_username, var("MONGODB_ROOT_USERNAME").expect("No MongoDB root user name found in ENV FILE"));
        assert_eq!(config.env.mongodb_root_pwd, var("MONGODB_ROOT_PWD").expect("No MongoDB root pwd found in ENV FILE"));
        assert_eq!(config.env.mongo_test_user, var("MONGO_TEST_USER").expect("No mongodb user name found in ENV FILE"));
        assert_eq!(config.env.mongo_test_password, var("MONGO_TEST_PASSWORD").expect("No user password for mongodb found in ENV FILE"));
    }
}
