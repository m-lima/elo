mod user;

use super::message;

pub fn user(control: &mut super::Control) -> user::User {
    user::User::new(control)
}
