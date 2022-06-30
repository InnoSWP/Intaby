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
        .map(|_| {
            let code = games.create_game(0, quiz.clone());
            assert_eq!(code.len(), 4);
            code
        })
        .collect::<Vec<_>>();
    codes.sort();
    codes.dedup();
    assert_eq!(CODE_GENS, codes.len())
}

#[test]
fn test_serialization() {
    test_lobby_serialization(vec![]);
    test_lobby_serialization(vec!["Jake", "Olyvia", "Guy"]);
    test_lobby_serialization(vec!["Jake", "Olyvia", "Guy", "Michael", "Olyvia"]);

    test_progress_serialization();

    test_finished_serialization();
}

fn test_lobby_serialization(mut players: Vec<&str>) {
    let quiz = QuizConfig {
        name: "Cool quiz".to_owned(),
        questions: vec![],
    };
    let user_id = 0;
    let mut game = Game {
        creator_id: user_id,
        players: HashMap::new(),
        quiz_config: quiz,
        state: GameState::Lobby,
    };
    for (user_id, name) in players.iter().enumerate() {
        let user_id = user_id as UserId;
        game.players.insert(
            user_id,
            Player {
                user_id,
                name: name.to_string(),
            },
        );
    }
    let mut state = game.to_serializable();
    assert!(
        match &mut state {
            SerGame::Lobby { players: names } => {
                players.sort();
                names.sort();
                assert_eq!(players, *names);
                true
            }
            _ => false,
        },
        "expected lobby, but found: {state:?}"
    );
}

fn test_progress_serialization() {
    let question = Question {
        answers: vec![],
        question_type: QuestionType::Poll,
        quiz_id: 0,
        text: "Polling".to_owned(),
        time: 60,
    };
    let quiz = QuizConfig {
        name: "Cool quiz".to_owned(),
        questions: vec![question.clone()],
    };
    let user_id = 0;
    let game = Game {
        creator_id: user_id,
        players: HashMap::new(),
        quiz_config: quiz,
        state: GameState::InProgress {
            current_question: 0,
            current_answers: HashMap::new(),
            start_time: Instant::now(),
        },
    };
    let mut state = game.to_serializable();
    assert!(
        match &mut state {
            SerGame::InProgress {
                current_question,
                current_question_id,
                time_left,
            } => {
                assert_eq!(*current_question_id, 0);
                assert_eq!(*current_question, question);
                assert_eq!(time_left.round() as u64, question.time);
                true
            }
            _ => false,
        },
        "expected game in progress, but found: {state:?}"
    );
}

fn test_finished_serialization() {
    let quiz = QuizConfig {
        name: "Cool quiz".to_owned(),
        questions: vec![],
    };
    let user_id = 0;
    let game = Game {
        creator_id: user_id,
        players: HashMap::new(),
        quiz_config: quiz,
        state: GameState::Finished,
    };
    let mut state = game.to_serializable();
    assert!(
        match &mut state {
            SerGame::Finished => true,
            _ => false,
        },
        "expected a finished game, but found: {state:?}"
    );
}
