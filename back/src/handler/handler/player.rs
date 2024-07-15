use super::super::{access, model};
use crate::smtp;

#[derive(Debug)]
pub struct Player<'a, A, S>
where
    A: super::Access,
    S: smtp::Smtp,
{
    handler: &'a mut super::Handler<A, S>,
}

impl<'a, A, S> Player<'a, A, S>
where
    A: super::Access,
    S: smtp::Smtp,
{
    pub fn new(handler: &'a mut super::Handler<A, S>) -> Self {
        Self { handler }
    }
}

impl<'a, S> Player<'a, access::Regular, S>
where
    S: smtp::Smtp,
{
    pub async fn handle(
        self,
        request: model::request::Player,
    ) -> Result<model::Response, model::Error> {
        let players = self.handler.store.players();

        match request {
            model::request::Player::Id => Ok(model::Response::User {
                id: self.handler.user.id(),
                pending: None,
            }),
            model::request::Player::List => players
                .list()
                .await
                .map_err(model::Error::Store)
                .map(|r| model::Response::Players(r.into_iter().map(Into::into).collect())),
            model::request::Player::Rename(name) => {
                let player = players
                    .rename(self.handler.user.id(), &name)
                    .await
                    .map_err(model::Error::Store)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Player(model::push::Player::Renamed {
                        player: player.id,
                        old: self.handler.user.name().clone(),
                        new: player.name.clone(),
                    }));

                self.handler.user.update_name(player.name);

                Ok(model::Response::Done)
            }
        }
    }
}

impl<'a, S> Player<'a, access::Pending, S>
where
    S: smtp::Smtp,
{
    // allow(clippy::unused_async): To match the expected signature
    #[allow(clippy::unused_async)]
    pub async fn handle(
        self,
        request: model::request::Player,
    ) -> Result<model::Response, model::Error> {
        match request {
            model::request::Player::Id => Ok(model::Response::User {
                id: self.handler.user.id(),
                pending: Some(true),
            }),
            _ => Err(model::Error::Forbidden),
        }
    }
}
