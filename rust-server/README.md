# Building

Install rust using rustup [here](https://www.rust-lang.org/tools/install)

Build the project using

```rust
cargo build --release
```

The executable file will be generated at `target/release/`

Alternatively, you can run the server locally with mock handlers using

```rust
cargo run -- --mock
```

# API

## Create game

> POST /games/\<quiz_id>

> data = \<user_id>

Returns: `String` - a 4-letter code of the new game

## Start game

> PUT /games/\<code>/state

> data = "InProgress"

## Join game

> POST /games/\<code>

> data = { user_id, name }

## Get game state

> GET /games/\<code>

Returns the state of the game, which has the type

```rust
enum Game {
    Lobby {
        players: Vec<String>,
    },
    InProgress {
        current_question: Question,
        current_question_id: QuestionId,
        time_left: f64,
    },
    Finished,
}
```

For example, if game was in lobby with players Jake and Olyvia, then the returned json would look like:
```json
{
  "Lobby": {
    "players": [
      "Jake",
      "Olyvia"
    ]
  }
}
```

## Give an answer

> PUT /games/\<code>

> data = { user_id, question_id, answers: [String] }
