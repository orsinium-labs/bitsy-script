use crate::*;

#[test]
fn test_interpreter() {
    let (words, _) = run("hello");
    let exp = vec![Word::Text("hello".to_string(), TextEffect::None)];
    assert_eq!(words, exp);

    let (words, _) = run("oh{br}hi");
    let exp = vec![
        Word::Text("oh".to_string(), TextEffect::None),
        Word::LineBreak,
        Word::Text("hi".to_string(), TextEffect::None),
    ];
    assert_eq!(words, exp);

    let (words, _) = run("{a = 14}{say a}");
    let exp = vec![Word::Text("14".to_string(), TextEffect::None)];
    assert_eq!(words, exp);
}

fn run(t: &str) -> (Vec<Word>, State) {
    let mut state = State::default();
    let tokenizer = Tokenizer::new(t);
    let interpreter = Interpreter {
        tokens: tokenizer,
        state: &mut state,
    };
    let words: Vec<_> = interpreter.collect();
    (words, state)
}
