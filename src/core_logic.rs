use rand::{self, Rng};

pub struct Game {
    player1_input: Box<dyn PlayerInput>,
    player2_input: Box<dyn PlayerInput>,
    on_round_finished: fn(RoundResult),
    on_game_finished: fn(RoundResult),
}

impl Game {

    pub fn new(player1_input: Box<dyn PlayerInput>, player2_input: Box<dyn PlayerInput>, on_round: fn(RoundResult), on_game_finished: fn(RoundResult)) -> Self {
        Self {
            player1_input,
            player2_input,
            on_round_finished: on_round,
            on_game_finished,
        }
    }

    pub fn start_game(&self, rounds: u8) {

        let mut player1_points = 0;
        let mut player2_points = 0;

        for _ in 0..rounds {
            let player1_move = self.player1_input.get_move();
            let player2_move = self.player2_input.get_move();
            
            let round_result = determine_round_result(player1_move, player2_move);

            match round_result {
                RoundResult::Tie => (),
                RoundResult::Player1Won => player1_points += 1,
                RoundResult::Player2Won => player2_points += 1,
            }

            (self.on_round_finished)(round_result)
        }

        if player1_points > player2_points {
            (self.on_game_finished)(RoundResult::Player1Won)
        } else if player2_points > player1_points {
            (self.on_game_finished)(RoundResult::Player2Won)
        } else {
            (self.on_game_finished)(RoundResult::Tie)
        }
    }
}

fn determine_round_result(move1: Move, move2: Move) -> RoundResult {

    match move1 {
        Move::Rock => match move2 {
            Move::Rock => RoundResult::Tie,
            Move::Paper => RoundResult::Player2Won,
            Move::Scissors => RoundResult::Player1Won,
        },
        Move::Paper => match move2 {
            Move::Rock => RoundResult::Player1Won,
            Move::Paper => RoundResult::Tie,
            Move::Scissors => RoundResult::Player2Won,
        },
        Move::Scissors => match move2 {
            Move::Rock => RoundResult::Player2Won,
            Move::Paper => RoundResult::Player1Won,
            Move::Scissors => RoundResult::Tie,
        },
    }
}

pub enum RoundResult {
    Tie,
    Player1Won,
    Player2Won,
}

pub struct BotPlayer {
    mode: BotMode, 
    enemy_move: Move,
}

pub enum BotMode {
    Random,
    Easy,
    Medium,
    Hard
}

impl BotPlayer {
    pub fn new(mode: BotMode) -> Self {
        Self {
            mode,
            enemy_move: Move::Rock, // Default move
        }
    }

    fn move_random() -> Move {
        match rand::thread_rng().gen_range(0..3) {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            _ => panic!("Error")
        }
    }

    fn move_easy(enemy_move: Move) -> Move {
        Self::move_probability(enemy_move, 0.3)
    }

    fn move_medium(enemy_move: Move) -> Move {
        Self::move_probability(enemy_move, 0.5)
    }

    fn move_hard(enemy_move: Move) -> Move {
        Self::move_probability(enemy_move, 0.7)
    }

    fn move_probability(enemy_move: Move, winning_probability: f64) -> Move {
        let winner_move = Self::winner_move(enemy_move);

            if rand::thread_rng().gen::<f64>() < winning_probability {
            return winner_move;
        }

        Self::rand_move_except(winner_move)
    }

    fn rand_move_except(exclude_move: Move) -> Move {
        let moves = vec![Move::Rock, Move::Paper, Move::Scissors];

        let mut moves: Vec<Move> = moves.into_iter().filter(|pmove| *pmove != exclude_move).collect();

        let selection = rand::thread_rng().gen_range(0..2);

        std::mem::replace(&mut moves[selection], Move::Rock)
    }

    fn winner_move(enemy_move: Move) -> Move {
        match enemy_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}

impl PlayerInput for BotPlayer {
    fn get_move(&self) -> Move {
        match self.mode {
            BotMode::Random => BotPlayer::move_random(),
            BotMode::Easy => todo!(),
            BotMode::Medium => todo!(),
            BotMode::Hard => todo!(),
        }
    }
}

pub trait PlayerInput {
    fn get_move(&self) -> Move;
}

#[derive(PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}
