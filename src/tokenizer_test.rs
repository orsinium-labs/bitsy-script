use crate::tokenizer::*;

#[test]
fn test_tokenizer() {
    use Tag::*;
    use Token::*;
    check("hello", vec![w("hello")]);
    check("  hello", vec![w("  hello")]);
    check("  hello ", vec![w("  hello ")]);
    check("hello world", vec![w("hello "), w("world")]);
    check("hello world!!!", vec![w("hello "), w("world!!!")]);
    check("    ", vec![w("    ")]);

    check("{br}", vec![OpenTag(Br)]);
    check("{ br}", vec![OpenTag(Br)]);
    check("{ br }", vec![OpenTag(Br)]);

    check("\n", vec![OpenTag(Br)]);
    check("{pg}", vec![OpenTag(Pg)]);

    check("{/pg}", vec![CloseTag(Pg)]);
    check("{/pg }", vec![CloseTag(Pg)]);
    check("{/ pg }", vec![CloseTag(Pg)]);
    check("{ / pg }", vec![CloseTag(Pg)]);
}

fn w(w: &str) -> Token {
    Token::Word(w.to_string())
}

fn check(given: &str, expected: Vec<Token>) {
    let tokenizer = Tokenizer::new(given);
    let actual: Vec<_> = tokenizer.collect();
    assert_eq!(actual, expected);
}
