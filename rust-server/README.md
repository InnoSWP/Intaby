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

The resulting json is tagged with the type field to indicate the state, and contains other fields next to the tag.

For example, if the game is in lobby with players Jake and Olyvia, then the returned json will look like:
```json
{
  "type": "Lobby",
  "players": [
    "Jake",
    "Olyvia"
  ]
}
```

Or if the game is currently in progress, the returned json will look similar to:
```json
{
  "type":  "InProgress",
  "current_question": {
      "answers": [],
      "question_type": "Poll",
      "quiz_id": 0,
      "text": "What is your favourite encoding format?",
      "time": 60
  },
  "current_question_id": 0,
  "time_left": 60
}
```

## Give an answer

> PUT /games/\<code>

> data = { user_id, question_id, answers: [String] }
