use super::message;

#[derive(Debug)]
pub struct User<'a> {
    control: &'a crate::Control,
}

impl<'a> User<'a> {
    pub fn new(control: &'a crate::Control) -> Self {
        Self { control }
    }

    pub async fn handle(self, user: message::User) -> Result<message::Response, message::Error> {
        let users = self.control.store.users();

        match user {
            message::User::Info => users
                .by_id(self.control.user_id)
                .await
                .map_err(message::Error::Store)
                .and_then(|r| r.ok_or(message::Error::NotFound))
                .map(message::Response::User),
            message::User::List => users
                .list()
                .await
                .map_err(message::Error::Store)
                .map(message::Response::Users),
            message::User::Get(email) => users
                .by_email(&email)
                .await
                .map_err(message::Error::Store)
                .and_then(|r| r.ok_or(message::Error::NotFound))
                .map(message::Response::User),
            message::User::Invite(email) => {
                let id = users
                    .invite(self.control.user_id, &email)
                    .await
                    .map_err(message::Error::Store)
                    .map(message::Response::Id)?;

                self.control
                    .broadcaster
                    .send(message::Push::Invited(email.clone()));
                tokio::spawn(crate::smtp::Payload::Invite(email).send());

                Ok(id)
            }
        }
    }
}
