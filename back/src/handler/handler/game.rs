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
    pub async fn handle(
        self,
        request: model::request::Game,
    ) -> Result<model::Response, model::Error> {
        let games = self.handler.store.games();

        match request {
            model::request::Game::List => games
                .list()
                .await
                .map_err(model::Error::Store)
                .map(|r| model::Response::Games(r.into_iter().map(Into::into).collect())),
            model::request::Game::Register {
                opponent,
                score,
                opponent_score,
            } => {
                let game = games
                    .register(
                        self.handler.user.id(),
                        opponent,
                        score,
                        opponent_score,
                        |one, two| {
                            let ratings = skillratings::elo::elo(
                                &skillratings::elo::EloRating { rating: one },
                                &skillratings::elo::EloRating { rating: two },
                                if score > opponent_score {
                                    &skillratings::Outcomes::WIN
                                } else {
                                    &skillratings::Outcomes::LOSS
                                },
                                &skillratings::elo::EloConfig::new(),
                            );
                            (ratings.0.rating, ratings.1.rating)
                        },
                    )
                    .await
                    .map_err(model::Error::Store)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Game(model::push::Game::Registered(game)));

                Ok(model::Response::Done)
            }
            model::request::Game::Accept(id) => {
                let (id, player_one, player_two) = games
                    .accept(self.handler.user.id(), id)
                    .await
                    .map_err(model::Error::Store)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Game(model::push::Game::Accepted {
                        id,
                        player_one,
                        player_two,
                    }));

                Ok(model::Response::Done)
            }
            model::request::Game::Cancel(id) => {
                let id = games
                    .cancel(self.handler.user.id(), id)
                    .await
                    .map_err(model::Error::Store)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Game(model::push::Game::Unregistered(id)));

                Ok(model::Response::Done)
            }
        }
    }
}

impl<'a> Game<'a, access::Pending> {
    // allow(clippy::unused_async): To match the expected signature
    #[allow(clippy::unused_async)]
    pub async fn handle(self, _: model::request::Game) -> Result<model::Response, model::Error> {
        Err(model::Error::Forbidden)
    }
}
