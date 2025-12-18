use crate::tokenizer::*;

#[test]
fn test_tokenizer() {
    use Tag::*;
    use Token::*;
    check("hello", vec![w("hello")]);
    check("{br}", vec![OpenTag(Br)]);
}

fn w(w: &str) -> Token {
    Token::Word(w.to_string())
}

fn check(given: &str, expected: Vec<Token>) {
    let tokenizer = Tokenizer::new(given);
    let actual: Vec<_> = tokenizer.collect();
    assert_eq!(actual, expected);
}
