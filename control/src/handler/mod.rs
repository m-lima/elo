mod user;

pub fn user(control: &mut super::Control) -> user::User {
    user::User::new(control)
}
