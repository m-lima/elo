use crate::message;
use crate::smtp;

#[derive(Debug)]
pub struct User<'a> {
    control: &'a mut crate::Control,
}

impl<'a> User<'a> {
    pub fn new(control: &'a mut crate::Control) -> Self {
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
            message::User::Invite(message::Invite { name, email }) => {
                let (user, domain) = {
                    let mut parts = email.split('@');
                    let Some(user) = parts.next() else {
                        return Err(message::Error::InvalidEmail);
                    };
                    let Some(domain) = parts.next() else {
                        return Err(message::Error::InvalidEmail);
                    };
                    if parts.next().is_some() {
                        return Err(message::Error::InvalidEmail);
                    }
                    (String::from(user), String::from(domain))
                };
                let id = users
                    .invite(self.control.user_id, &email)
                    .await
                    .map_err(message::Error::Store)
                    .map(message::Response::Id)?;

                self.control
                    .broadcaster
                    .send(message::Push::Invited(email.clone()));

                self.control
                    .smtp
                    .send(smtp::Payload::Invite { name, user, domain })
                    .await;

                Ok(id)
            }
        }
    }
}
