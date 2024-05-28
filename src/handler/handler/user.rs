use super::super::model;
use crate::smtp;

#[derive(Debug)]
pub struct User<'a> {
    handler: &'a mut super::Handler,
}

impl<'a> User<'a> {
    pub fn new(handler: &'a mut super::Handler) -> Self {
        Self { handler }
    }

    pub async fn handle(self, user: model::User) -> Result<model::Response, model::Error> {
        let users = self.handler.store.users();

        match user {
            model::User::Info => users
                .by_id(self.handler.user_id)
                .await
                .map_err(model::Error::Store)
                .and_then(|r| r.ok_or(model::Error::NotFound))
                .map(model::Response::User),
            model::User::Rename(name) => users
                .rename(self.handler.user_id, &name)
                .await
                .map_err(model::Error::Store)
                .and_then(|r| r.ok_or(model::Error::NotFound))
                .map(model::Response::Id),
            model::User::List => users
                .list()
                .await
                .map_err(model::Error::Store)
                .map(model::Response::Users),
            model::User::Get(email) => users
                .by_email(&email)
                .await
                .map_err(model::Error::Store)
                .and_then(|r| r.ok_or(model::Error::NotFound))
                .map(model::Response::User),
            model::User::Invite(model::Invite { name, email }) => {
                let mailbox =
                    smtp::Mailbox::new(name, email).map_err(model::Error::InvalidEmail)?;

                let id = users
                    .invite(self.handler.user_id, mailbox.name(), mailbox.email())
                    .await
                    .map_err(model::Error::Store)
                    .map(model::Response::Id)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Invited(model::Invite {
                        name: String::from(mailbox.name()),
                        email: String::from(mailbox.email()),
                    }));

                self.handler.smtp.send(smtp::Payload::Invite(mailbox)).await;

                Ok(id)
            }
        }
    }
}
