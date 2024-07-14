use crate::core_logic::{self, BotMode, BotPlayer, Game, Move, PlayerInput};

fn render_ui() {
    let player = Player{};
    let bot_player = BotPlayer::new(BotMode::Easy);

    let game = Game::new(Box<player>, Box<bot_player>, on_round, on_game_finished);
}

struct Player {}

impl PlayerInput for Player {
    fn get_move(&self) -> Move {
        todo!()
    }
}
