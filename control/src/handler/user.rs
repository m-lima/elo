use super::super::message;

#[derive(Debug)]
pub struct User<'a> {
    control: &'a crate::Control,
}

impl<'a> User<'a> {
    pub fn new(control: &'a crate::Control) -> Self {
        Self { control }
    }

    pub async fn handle(self, user: message::User) -> Result<message::Response, message::Error> {
        match user {
            message::User::Info => {
                match self.control.store.users().by_id(self.control.user_id).await {
                    Ok(Some(user)) => Ok(message::Response::User(user)),
                    Ok(None) => Err(message::Error::NotFound),
                    Err(error) => Err(message::Error::Store(error)),
                }
            }
            message::User::List => self
                .control
                .store
                .users()
                .list()
                .await
                .map(message::Response::Users)
                .map_err(message::Error::Store),
            message::User::Get(email) => match self.control.store.users().by_email(&email).await {
                Ok(Some(user)) => Ok(message::Response::User(user)),
                Ok(None) => Err(message::Error::NotFound),
                Err(error) => Err(message::Error::Store(error)),
            },
        }
    }
}
