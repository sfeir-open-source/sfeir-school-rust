pub mod game;
pub mod cell;
pub mod game_over_overlay;
mod slot_if;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameStatus {
    Playing,
    New,
    Lost,
}