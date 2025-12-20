use crate::*;

#[test]
fn test_tokenizer_text() {
    check("hello", vec![w("hello")]);
    check("  hello", vec![w("  hello")]);
    check("  hello ", vec![w("  hello ")]);
    check("hello world", vec![w("hello "), w("world")]);
    check("hello world!!!", vec![w("hello "), w("world!!!")]);
    check("    ", vec![w("    ")]);
}

#[test]
fn test_tokenizer_tags() {
    use Tag::*;
    use Token::*;
    check("{br}", vec![OpenTag(Br)]);
    check("{ br}", vec![OpenTag(Br)]);
    check("{ br }", vec![OpenTag(Br)]);

    check("\n", vec![OpenTag(Br)]);
    check("{pg}", vec![OpenTag(Pg)]);

    check("{/pg}", vec![CloseTag(Pg)]);
    check("{/pg }", vec![CloseTag(Pg)]);
    check("{/ pg }", vec![CloseTag(Pg)]);
    check("{ / pg }", vec![CloseTag(Pg)]);

    check("{br}{br}", vec![OpenTag(Br), OpenTag(Br)]);
    check(
        "oh{br}hi{br}mark",
        vec![w("oh"), OpenTag(Br), w("hi"), OpenTag(Br), w("mark")],
    );

    check(
        "{blegh}",
        vec![OpenTag(Unknown("blegh".to_string(), "".to_string()))],
    );
    check(
        "{ blegh }",
        vec![OpenTag(Unknown("blegh".to_string(), "".to_string()))],
    );
    check(
        "{ blegh args }",
        vec![OpenTag(Unknown("blegh".to_string(), "args".to_string()))],
    );
    check(
        "{ blegh    args }",
        vec![OpenTag(Unknown("blegh".to_string(), "args".to_string()))],
    );
    check("{clr1}", vec![OpenTag(Eff(TextEffect::Color(1)))]);
    check("{clr 1}", vec![OpenTag(Eff(TextEffect::Color(2)))]);

    let expr = Expr::SimpleExpr(SimpleExpr::Var("hi".to_string()));
    check("{say hi}", vec![OpenTag(Say(expr.clone()))]);
    check("{ say  hi }", vec![OpenTag(Say(expr))]);
    let expr = Expr::SimpleExpr(SimpleExpr::Item("cat".to_string()));
    check(r#"{say {item "cat"}}"#, vec![OpenTag(Say(expr))]);

    check(
        r#"{exit "hi,3,4"}"#,
        vec![OpenTag(Exit("hi".to_string(), 3, 4))],
    );
    check(
        r#"{exit "hi",3,4}"#,
        vec![OpenTag(Exit("hi".to_string(), 3, 4))],
    );
    check(
        r#"{exit "hi", 3, 4}"#,
        vec![OpenTag(Exit("hi".to_string(), 3, 4))],
    );
}

#[test]
fn test_tokenizer_assignment() {
    use Tag::*;
    use Token::*;
    let val = Val::S("hello world!".to_string());
    let expr = Expr::SimpleExpr(SimpleExpr::Val(val));
    check(
        "{ a = hello world! }",
        vec![OpenTag(Set("a".to_string(), expr))],
    );

    let expr = Expr::SimpleExpr(SimpleExpr::Val(Val::I(14)));
    check("{a = 14}", vec![OpenTag(Set("a".to_string(), expr))]);

    let expr = Expr::SimpleExpr(SimpleExpr::Val(Val::I(-14)));
    check("{a = -14}", vec![OpenTag(Set("a".to_string(), expr))]);

    #[allow(clippy::approx_constant)]
    let expr = Expr::SimpleExpr(SimpleExpr::Val(Val::F(3.14)));
    check("{a = 3.14}", vec![OpenTag(Set("a".to_string(), expr))]);

    let expr = Expr::SimpleExpr(SimpleExpr::Val(Val::I(1)));
    check("{a = true}", vec![OpenTag(Set("a".to_string(), expr))]);

    let expr = Expr::SimpleExpr(SimpleExpr::Val(Val::S("hi".to_string())));
    check(r#"{a = "hi"}"#, vec![OpenTag(Set("a".to_string(), expr))]);

    let expr = Expr::SimpleExpr(SimpleExpr::Var("hi".to_string()));
    check(r#"{a = hi}"#, vec![OpenTag(Set("a".to_string(), expr))]);

    let expr = Expr::SimpleExpr(SimpleExpr::Var("hi_mark".to_string()));
    check(
        r#"{a = hi_mark}"#,
        vec![OpenTag(Set("a".to_string(), expr))],
    );
}

fn w(w: &str) -> Token {
    Token::Word(w.to_string())
}

fn check(given: &str, expected: Vec<Token>) {
    let tokenizer = Tokenizer::new(given);
    let actual: Vec<_> = tokenizer.collect();
    assert_eq!(actual, expected);
}
