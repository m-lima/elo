use super::super::model;

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
            model::Player::Rename(name) => {
                players
                    .rename(self.handler.user_id, &name)
                    .await
                    .map_err(model::Error::Store)
                    .and_then(|r| r.ok_or(model::Error::NotFound))
                    .map(model::Response::Id)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Renamed(model::Renamed {
                        player: self.handler.user_id,
                        name,
                    }));

                Ok(model::Response::Renamed)
            }
        }
    }
}
