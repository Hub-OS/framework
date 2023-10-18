use super::InputEvent;
use math::*;

pub enum GameWindowEvent {
    Resumed,
    CloseRequested,
    Resized(UVec2),
    Moved(IVec2),
    InputEvent(InputEvent),
}

impl std::convert::From<InputEvent> for GameWindowEvent {
    fn from(event: InputEvent) -> Self {
        GameWindowEvent::InputEvent(event)
    }
}
