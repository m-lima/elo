use super::super::model;
use crate::{mailbox, smtp};

#[derive(Debug)]
pub struct Player<'a> {
    handler: &'a mut super::Handler,
}

impl<'a> Player<'a> {
    pub fn new(handler: &'a mut super::Handler) -> Self {
        Self { handler }
    }

    pub async fn handle(self, request: model::Player) -> Result<model::Response, model::Error> {
        let players = self.handler.store.players();

        match request {
            model::Player::Id => Ok(model::Response::Id(self.handler.user_id)),
            model::Player::List => players
                .list()
                .await
                .map_err(model::Error::Store)
                .map(model::Response::Players),
            model::Player::Rename(name) => players
                .rename(self.handler.user_id, &name)
                .await
                .map_err(model::Error::Store)
                .and_then(|r| r.ok_or(model::Error::NotFound))
                .map(model::Response::Id),
            model::Player::Invite(model::Invite { name, email }) => {
                let mailbox =
                    mailbox::Mailbox::new(name, email).map_err(model::Error::InvalidEmail)?;

                let id = players
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
