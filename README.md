# bitsy-interpreter

Rust interpreter for the scripting language used for dialogs in [Bitsy](https://bitsy.org/) game engine.

It's far from 100% compatibility. A lot of complex nodes won't be parsed. The goal was to keep the implementation simple and the AST flat. This might change in the future implementations and we'll hopefully have a proper parser for all quirks and gimmicks of bitsy.

## Installation

```bash
cargo add bitsy-script
```

## Usage

```rust
use bitsy_script::*;
let dialog = "hello {wvy}world{/wvy}!{br}";
let mut state = State::default();
let tokenizer = Tokenizer::new(dialog);
let interpreter = Interpreter {
    tokens: tokenizer,
    state: &mut state,
};
let words: Vec<_> = interpreter.collect();
```
