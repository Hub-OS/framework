use crate::common::GameIO;

pub trait GameService {
    /// Called every tick before all updates
    fn pre_update(&mut self, _game_io: &mut GameIO) {}

    /// Called every tick after all updates
    fn post_update(&mut self, _game_io: &mut GameIO) {}
}
