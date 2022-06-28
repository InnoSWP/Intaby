pub use rocket::serde;
pub use rocket::serde::json::serde_json;
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type UserId = u64;
pub type QuizId = u64;
pub type QuestionId = u64;
pub type Time = u64;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct Games {
    map: HashMap<GameCode, Game>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct QuizConfig {
    pub name: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub enum QuestionType {
    Poll,
    Quiz,
    Multiquiz,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct Question {
    pub answers: Vec<Answer>,
    pub quiestion_type: QuestionType,
    pub quiz_id: QuizId,
    pub text: String,
    pub time: Time,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct Answer {
    pub correct_answer: bool,
    pub question_id: QuestionId,
    pub text: String,
}

/// Represents different states of a game
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub enum GameState {
    /// Initial state of the game
    Lobby,
    InProgress,
    Finished,
}

pub type GameCode = String;
pub type PlayerName = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Player {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct Game {
    players: HashMap<PlayerName, Player>,
    quiz_config: QuizConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct GameAnswer {}

impl Games {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn create_game(&mut self, quiz_config: QuizConfig) -> GameCode {
        let code = loop {
            let code = game_code_generator();
            if !self.map.contains_key(&code) {
                break code;
            }
        }; // TODO: avoid infinite loop
        let game = Game::new(quiz_config);
        let res = self.map.insert(code.clone(), game);
        assert!(res.is_none());
        code
    }

    pub fn get_game(&self, code: &GameCode) -> Option<&Game> {
        self.map.get(code)
    }

    pub fn get_game_mut(&mut self, code: &GameCode) -> Option<&mut Game> {
        self.map.get_mut(code)
    }
}

impl Game {
    pub fn new(quiz_config: QuizConfig) -> Self {
        Self {
            players: HashMap::new(),
            quiz_config,
        }
    }

    pub fn player_join(&mut self, player_name: PlayerName) {
        self.players.entry(player_name).or_insert_with(|| Player {});
    }
}

pub fn game_code_generator() -> GameCode {
    use rand::Rng;
    const LEN: usize = 4;

    let mut rng = rand::thread_rng();
    let mut code = String::new();
    for _ in 0..LEN {
        let symbol = rng.gen_range('A'..='Z');
        code.push(symbol);
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_create() {
        let mut games = Games::new();
        const CODE_GENS: usize = 1000;
        let quiz = QuizConfig {
            name: "Cool quiz".to_owned(),
            questions: vec![],
        };
        let mut codes = (0..CODE_GENS)
            .map(|_| games.create_game(quiz.clone()))
            .collect::<Vec<_>>();
        codes.sort();
        codes.dedup();
        assert_eq!(CODE_GENS, codes.len())
    }
}
