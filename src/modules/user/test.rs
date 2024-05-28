use super::dto::User;

#[test]
fn test_new_user() {
    let email = String::from("troin@hotmail.com");
    let password = String::from("Thisisapassword");
    let description = String::from("description");
    let role: u8 = 3;

    // Use string slices here
    let email_slice = email.as_str();
    let password_slice = password.as_str();
    
    let user = User {
        email: email_slice.to_string(),
        password: password_slice.to_string(),
        description,
        role,
        _id: None,
    };

    assert!(user.email == email_slice);
    assert!(user.password == password_slice);
    assert!(user._id == None);
}