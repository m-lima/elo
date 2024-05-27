use crate::message;

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
            message::User::Rename(name) => users
                .rename(self.control.user_id, &name)
                .await
                .map_err(message::Error::Store)
                .and_then(|r| r.ok_or(message::Error::NotFound))
                .map(message::Response::Id),
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
                let mailbox =
                    smtp::Mailbox::new(name, email).map_err(message::Error::InvalidEmail)?;

                let id = users
                    .invite(self.control.user_id, mailbox.name(), mailbox.email())
                    .await
                    .map_err(message::Error::Store)
                    .map(message::Response::Id)?;

                self.control
                    .broadcaster
                    .send(message::Push::Invited(message::Invite {
                        name: String::from(mailbox.name()),
                        email: String::from(mailbox.email()),
                    }));

                self.control.smtp.send(smtp::Payload::Invite(mailbox)).await;

                Ok(id)
            }
        }
    }
}
