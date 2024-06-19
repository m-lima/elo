use super::super::{access, model};

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

impl<'a> Player<'a, access::Regular> {
    pub async fn handle(
        self,
        request: model::request::Player,
    ) -> Result<model::Response, model::Error> {
        let players = self.handler.store.players();

        match request {
            model::request::Player::Id => Ok(model::Response::Id(self.handler.user.id())),
            model::request::Player::List => players
                .list()
                .await
                .map_err(model::Error::Store)
                .map(|r| model::Response::Players(r.into_iter().map(Into::into).collect())),
            model::request::Player::Rename(name) => {
                players
                    .rename(self.handler.user.id(), &name)
                    .await
                    .map_err(model::Error::Store)
                    .and_then(|r| r.ok_or(model::Error::NotFound))
                    .map(model::Response::Id)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Player(model::push::Player::Renamed {
                        player: self.handler.user.id(),
                        name,
                    }));

                Ok(model::Response::Done)
            }
        }
    }
}

impl<'a> Player<'a, access::Pending> {
    // allow(clippy::unused_async): To match the expected signature
    #[allow(clippy::unused_async)]
    pub async fn handle(self, _: model::request::Player) -> Result<model::Response, model::Error> {
        Err(model::Error::Forbidden)
    }
}
