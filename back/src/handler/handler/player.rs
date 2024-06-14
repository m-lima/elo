use super::super::model;
use crate::types;

#[derive(Debug)]
pub struct Player<'a, A>
where
    A: super::Access,
{
    handler: &'a mut super::Handler<A>,
}

impl<'a, A> Player<'a, A>
where
    A: super::Access,
{
    pub fn new(handler: &'a mut super::Handler<A>) -> Self {
        Self { handler }
    }
}

impl<'a> Player<'a, types::ExistingUser> {
    pub async fn handle(self, request: model::Player) -> Result<model::Response, model::Error> {
        let players = self.handler.store.players();

        match request {
            model::Player::Id => Ok(model::Response::Id(self.handler.user.id)),
            model::Player::List => players
                .list()
                .await
                .map_err(model::Error::Store)
                .map(model::Response::Players),
            model::Player::Rename(name) => {
                players
                    .rename(self.handler.user.id, &name)
                    .await
                    .map_err(model::Error::Store)
                    .and_then(|r| r.ok_or(model::Error::NotFound))
                    .map(model::Response::Id)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Renamed(model::Renamed {
                        player: self.handler.user.id,
                        name,
                    }));

                Ok(model::Response::Renamed)
            }
        }
    }
}

impl<'a> Player<'a, types::PendingUser> {
    // allow(clippy::unused_async): To match the expected signature
    #[allow(clippy::unused_async)]
    pub async fn handle(self, _: model::Player) -> Result<model::Response, model::Error> {
        Err(model::Error::Forbidden)
    }
}
