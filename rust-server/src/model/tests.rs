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
                players == *names
            }
            _ => false,
        },
        "expected lobby with players {players:?}, but found: {state:?}"
    );
}
