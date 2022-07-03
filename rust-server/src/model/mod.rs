pub use rocket::serde;
pub use rocket::serde::json::serde_json;
use rocket::{
    serde::{Deserialize, Serialize},
    time::Instant,
};
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests;

pub type UserId = u64;
pub type QuizId = u64;
pub type QuestionId = u64;
pub type Time = u64;

#[derive(Debug)]
pub struct Games {
    map: HashMap<GameCode, Game>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct QuizConfig {
    pub name: String,
    pub questions: Vec<Question<Answer>>,
    pub user_id: UserId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub enum QuestionType {
    Poll,
    Quiz,
    Multiquiz,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct Question<T> {
    pub answers: Vec<T>,
    pub question_type: QuestionType,
    pub quiz_id: QuizId,
    pub text: String,
    pub time: Time,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct Answer {
    pub correct_answer: bool,
    pub question_id: QuestionId,
    pub text: String,
}

/// Represents different states of a game
#[derive(Debug)]
pub enum GameState {
    /// Initial state of the game
    Lobby,
    InProgress {
        current_question: usize,
        start_time: Instant,
    },
    Finished {
        stats: Option<(Vec<LeaderboardEntry>, HashMap<PlayerName, PlayerStats>)>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum StateTarget {
    InProgress,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct StateUpdate {
    pub user_id: UserId,
    pub state: StateTarget,
}

pub type GameCode = String;
pub type PlayerName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct GameAnswer {
    player_name: PlayerName,
    question_id: QuestionId,
    answers: Vec<String>,
}

#[derive(Debug)]
pub struct Game {
    creator_id: UserId,
    players: HashSet<PlayerName>,
    quiz_config: QuizConfig,
    state: GameState,
    answers: HashMap<PlayerName, Vec<GameAnswer>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct QuestionStats {
    pub question_id: QuestionId,
    pub player_answers: Vec<String>,
    pub is_fully_correct: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct PlayerStats {
    pub all_answers: Vec<GameAnswer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde", deny_unknown_fields)]
pub struct LeaderboardEntry {
    player: PlayerName,
    score: f64,
    max_score: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", tag = "type", deny_unknown_fields)]
pub enum SerGame {
    Lobby {
        players: Vec<PlayerName>,
    },
    InProgress {
        current_question: Question<String>,
        current_question_id: QuestionId,
        /// Time left for the current question in seconds
        time_left: f64,
    },
    Finished {
        quiz: QuizConfig,
        leaderboard: Vec<LeaderboardEntry>,
        statistics: HashMap<PlayerName, PlayerStats>,
    },
}

impl Games {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn create_game(&mut self, owner_id: UserId, quiz_config: QuizConfig) -> GameCode {
        let code = loop {
            let code = game_code_generator();
            if !self.map.contains_key(&code) {
                break code;
            }
        }; // TODO: avoid infinite loop
        let game = Game::new(owner_id, quiz_config);
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
    pub fn new(creator_id: UserId, quiz_config: QuizConfig) -> Self {
        Self {
            players: Default::default(),
            state: GameState::Lobby,
            answers: Default::default(),
            quiz_config,
            creator_id,
        }
    }

    pub fn player_join(&mut self, name: PlayerName) {
        // Players can only join before the start
        if !matches!(self.state, GameState::Lobby) {
            return;
        }

        // TODO: check collisions
        self.players.insert(name);
    }

    pub fn change_state(&mut self, update: StateUpdate) {
        // Only the creator can change the state of the game
        if self.creator_id != update.user_id {
            return;
        }

        match update.state {
            StateTarget::InProgress => self.start_game(),
        }
    }

    fn start_game(&mut self) {
        match self.state {
            GameState::Lobby => {
                self.state = GameState::InProgress {
                    current_question: 0,
                    start_time: Instant::now(), // TODO: delay start
                };
            }
            _ => {}
        }
    }

    pub fn player_answer(&mut self, answer: GameAnswer) {
        self.update();
        if !self.players.contains(&answer.player_name) {
            return;
        }

        let accepted = match &mut self.state {
            GameState::InProgress { .. } => true,
            GameState::Finished { stats } => {
                *stats = None;
                true
            }
            _ => false,
        };
        if accepted {
            self.answers
                .entry(answer.player_name.clone())
                .or_default()
                .push(answer);
        }
    }

    fn update(&mut self) {
        match &mut self.state {
            GameState::Lobby => {}
            GameState::InProgress {
                current_question,
                start_time,
            } => {
                let elapsed = start_time.elapsed().as_seconds_f64().floor() as Time;
                let time_limit = self
                    .quiz_config
                    .questions
                    .get(*current_question)
                    .expect("Current question index is illegal")
                    .time;
                if elapsed >= time_limit {
                    if *current_question + 1 >= self.quiz_config.questions.len() {
                        self.state = GameState::Finished { stats: None };
                    } else {
                        // Next question
                        *start_time = Instant::now();
                        *current_question += 1;
                    }
                }
            }
            GameState::Finished { .. } => {
                // TODO: drop the game after some time
            }
        }
    }

    pub fn to_serializable(&mut self) -> SerGame {
        self.update();
        match &mut self.state {
            GameState::Lobby => SerGame::Lobby {
                players: self.players.iter().cloned().collect(),
            },
            GameState::InProgress {
                current_question,
                start_time,
                ..
            } => {
                let question = self
                    .quiz_config
                    .questions
                    .get(*current_question)
                    .expect("Failed to get current question")
                    .clone();
                SerGame::InProgress {
                    time_left: question.time as f64 - start_time.elapsed().as_seconds_f64(),
                    current_question: Question {
                        answers: question
                            .answers
                            .into_iter()
                            .map(|answer| answer.text)
                            .collect(),
                        question_type: question.question_type,
                        quiz_id: question.quiz_id,
                        text: question.text,
                        time: question.time,
                    },
                    current_question_id: *current_question as QuestionId,
                }
            }
            GameState::Finished { stats } => {
                if stats.is_none() {
                    *stats = Some(gen_statistics(&self.answers, &self.quiz_config));
                }
                let (leaderboard, statistics) = stats.as_ref().unwrap().clone();
                SerGame::Finished {
                    quiz: self.quiz_config.clone(),
                    statistics,
                    leaderboard,
                }
            }
        }
    }
}

pub fn gen_statistics(
    answers: &HashMap<PlayerName, Vec<GameAnswer>>,
    quiz_config: &QuizConfig,
) -> (Vec<LeaderboardEntry>, HashMap<PlayerName, PlayerStats>) {
    let mut statistics: HashMap<PlayerName, PlayerStats> = HashMap::new();
    for (player, answers) in answers {
        let player = player.clone();
        let stats = statistics.entry(player).or_default();
        for answer in answers {
            let mut answer = answer.clone();
            answer.answers.sort();
            answer.answers.dedup();
            stats.all_answers.push(answer);
        }
    }
    let mut board = statistics
        .iter()
        .map(|(player, stats)| {
            let score = stats
                .all_answers
                .iter()
                .map(|stats| {
                    let question = quiz_config
                        .questions
                        .get(stats.question_id as usize)
                        .expect("Invalid question id in stats");
                    match question.question_type {
                        QuestionType::Poll => 1.0,
                        QuestionType::Quiz => {
                            if stats
                                .answers
                                .last()
                                .filter(|answer| question.is_answer_correct(answer))
                                .is_some()
                            {
                                1.0
                            } else {
                                0.0
                            }
                        }
                        QuestionType::Multiquiz => {
                            let correct_size = question
                                .answers
                                .iter()
                                .filter(|answer| answer.correct_answer)
                                .count();

                            let mut correct_answers = 0;
                            let mut incorrect_answers = 0;
                            for answer in &stats.answers {
                                if question.is_answer_correct(answer) {
                                    correct_answers += 1;
                                } else {
                                    incorrect_answers += 1;
                                }
                            }

                            1.0 - (correct_size - correct_answers + incorrect_answers) as f64
                                / question.answers.len() as f64
                        }
                    }
                })
                .sum();
            LeaderboardEntry {
                player: player.to_string(),
                score,
                max_score: quiz_config.questions.len(),
            }
        })
        .collect::<Vec<_>>();
    board.sort_by(|b, a| a.score.partial_cmp(&b.score).unwrap());

    (board, statistics)
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

impl Question<Answer> {
    fn is_answer_correct(&self, player_answer: &str) -> bool {
        self.answers
            .iter()
            .filter(|answer| answer.correct_answer)
            .any(|correct| correct.text.eq(player_answer))
    }
}
