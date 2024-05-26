mod user;

pub fn user(control: &super::Control) -> user::User {
    user::User::new(control)
}
