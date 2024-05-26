mod user;

use super::message;

pub fn user(control: &super::Control) -> user::User {
    user::User::new(control)
}
