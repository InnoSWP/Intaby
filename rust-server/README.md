# Building

Install rust using rustup [here](https://www.rust-lang.org/tools/install)

Build the project using

```rust
cargo build --release
```

The executable file will be generated at `target/release/`

# API

 - Create game
> POST /games/\<quiz_id>

> data = \<user_id>

Returns: `String` - a 4-letter code of the new game

 - Join game
> POST /games/\<code>

> data = { user_id, name }

 - Get game state
> GET /games/\<code>

Returns: `TODO` - state of the game

 - Give an answer
> PUT /games/\<code>

> data = { user_id, question_id, answers: [String] }
