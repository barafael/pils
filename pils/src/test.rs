#[cfg(test)]
use super::*;

#[test]
fn process_simple_sexpr() {
    assert_eq!(process_str("* 2 (+ 4 5) (/ 10 2) (-2) (- 1 2 3)"), "1080");
}

#[test]
fn process_simple_qexpr() {
    assert_eq!(process_str("{ * 1 2 3 }"), "{ * 1 2 3 }");
}

#[test]
fn process_join() {
    assert_eq!(
        process_str("join { { 1 2 3 } { 4 ( 5 6 ) } }"),
        "{ 1 2 3 4 ( 5 6 ) }" // disagrees with variables.c, wtf?
    );
}

#[test]
fn example_11a() {
    assert_eq!(process_str("+"), "<function>");
}

#[test]
fn example_11b() {
    assert_eq!(process_str("eval (head {5 10 11 15})"), "5");
}

#[test]
fn example_11d() {
    assert_eq!(process_str("(eval (head {+ - + - * /})) 10 20"), "30");
}

#[test]
fn example_11e() {
    assert_eq!(process_str("hello"), "Error: unbound symbol");
}

#[test]
fn process_eval() {
    assert_eq!(process_str("eval { tail ( list 1 2 3 4 ) }"), "{ 2 3 4 }");
}

#[test]
fn process_def() {
    let _ = process_str("def {x} 100");
    let _ = process_str("def {y} 200");
    assert_eq!(process_str("+ x y"), "300".to_string());
    let _ = process_str("def {a b} 5 6");
    assert_eq!(process_str("+ a b"), "11".to_string());
    let _ = process_str("def {arglist} {a b x y}");
    let _ = process_str("def arglist 1 2 3 4");
    assert_eq!(process_str("list a b x y"), "{ 1 2 3 4 }".to_string());
}

#[test]
fn displays() {
    let line = "eval { tail ( list 1 2 3 4 ) }";
    let mut pairs = Pils::parse(Rule::Pils, line).unwrap();
    let pair = pairs.next().unwrap();

    let val = Value::from_pair(pair).unwrap().unwrap();

    let result = format!("{val}");
    assert_eq!(format!("( {line} )"), result);
}
