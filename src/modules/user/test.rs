use crate::framework::testing::Mock;
use super::dto::User;

#[test]
fn test_new_user() {
    let email = String::from("troin@hotmail.com");
    let password = String::from("Thisisapassword");
    let description = String::from("description");

    // Use string slices here
    let email_slice = email.as_str();
    let password_slice = password.as_str();
    
    let user = User {
        email: email_slice.to_string(),
        password: password_slice.to_string(),
        description,
        public_key: String::from("randompublickey"),
        _id: None,
    };

    assert!(user.email == email_slice);
    assert!(user.password == password_slice);
    assert!(user._id == None);
    assert!(user.public_key == "randompublickey");
}

#[test]
fn test_fake_user() {
    let user = User::mock();
    //Check that the user is valid
    assert!(user.email.contains("@"));
    assert!(user.password.len() >= 8);
    assert!(user.description.len() >= 10);
    assert!(user._id == None);
    assert!(user.public_key.len() > 0);
}