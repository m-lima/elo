use super::super::{access, model};
use crate::smtp;

#[derive(Debug)]
pub struct Game<'a, A, S>
where
    A: super::Access,
    S: smtp::Smtp,
{
    handler: &'a mut super::Handler<A, S>,
}

impl<'a, A, S> Game<'a, A, S>
where
    A: super::Access,
    S: smtp::Smtp,
{
    pub fn new(handler: &'a mut super::Handler<A, S>) -> Self {
        Self { handler }
    }
}

impl<'a, S> Game<'a, access::Regular, S>
where
    S: smtp::Smtp,
{
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
                player,
                opponent,
                score,
                opponent_score,
                challenge,
                millis,
            } => {
                let (game, updates) = games
                    .register(
                        (player, opponent),
                        (score, opponent_score),
                        challenge,
                        millis,
                        skillratings::elo::EloRating::new().rating,
                        super::rating_updater,
                    )
                    .await
                    .map_err(model::Error::Store)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Game(model::push::Game::Registered {
                        game,
                        updates: updates.into_iter().map(Into::into).collect(),
                    }));

                Ok(model::Response::Done)
            }
            model::request::Game::Update(game) => {
                let updates = games
                    .update(
                        &game,
                        skillratings::elo::EloRating::new().rating,
                        super::rating_updater,
                    )
                    .await
                    .map_err(model::Error::Store)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Game(model::push::Game::Updated {
                        game: game.id,
                        updates: updates.into_iter().map(Into::into).collect(),
                    }));

                Ok(model::Response::Done)
            }
            model::request::Game::History(game) => games
                .history(game)
                .await
                .map_err(model::Error::Store)
                .map(|r| model::Response::History(r.into_iter().map(Into::into).collect())),
        }
    }
}

impl<'a, S> Game<'a, access::Pending, S>
where
    S: smtp::Smtp,
{
    // allow(clippy::unused_async): To match the expected signature
    #[allow(clippy::unused_async)]
    pub async fn handle(self, _: model::request::Game) -> Result<model::Response, model::Error> {
        Err(model::Error::Forbidden)
    }
}
