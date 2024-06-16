use super::super::{access, model};

#[derive(Debug)]
pub struct Game<'a, A>
where
    A: super::Access,
{
    handler: &'a mut super::Handler<A>,
}

impl<'a, A> Game<'a, A>
where
    A: super::Access,
{
    pub fn new(handler: &'a mut super::Handler<A>) -> Self {
        Self { handler }
    }
}

impl<'a> Game<'a, access::Regular> {
    pub async fn handle(self, request: model::Game) -> Result<model::Response, model::Error> {
        let games = self.handler.store.games();

        match request {
            model::Game::List => games
                .list()
                .await
                .map_err(model::Error::Store)
                .map(model::Response::Games),
        }
    }
}

impl<'a> Game<'a, access::Pending> {
    // allow(clippy::unused_async): To match the expected signature
    #[allow(clippy::unused_async)]
    pub async fn handle(self, _: model::Game) -> Result<model::Response, model::Error> {
        Err(model::Error::Forbidden)
    }
}
