use std::sync::Arc;

use crate::{framework::config::OxidizeConfig, modules::mongo::service::MongoOracle};

use super::service::MailOracle;

#[tokio::test]
async fn test_ranodom_string_generator() {
    let config = Arc::new(OxidizeConfig::new().expect("Error while getting config"));
    let mongo = Arc::new(MongoOracle::new(config.clone()).await);
    let mail = MailOracle::new(config, mongo);
    let length = 32;
    let result = mail.generate_random_url_safe_string(length);

    // URL-safe Base64 alphabet
    let url_safe_base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

    // Check that all characters in the result are URL-safe Base64 characters
    for c in result.chars() {
        assert!(url_safe_base64_chars.contains(c), "Character {} is not URL-safe", c);
    }

    // Optional: Check the length of the generated string
    // This is tricky because URL_SAFE_NO_PAD encoding length depends on the input length
    // Base64 encoding increases the length by ~4/3, so we can check for some reasonable bounds
    let expected_min_length = (length * 4 / 3) as usize;
    assert!(result.len() >= expected_min_length, "Encoded string is too short");
}