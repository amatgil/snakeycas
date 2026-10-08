#![allow(non_camel_case_types)] // lmao
#![allow(unused)] // this is just tests anyway
use snakeyidentifiers_macro::snakeycas;

// Many test means more green in cargo test :relieved:
#[test]
fn upper() {
    struct UpperCamelCase;
    const OUT: &str = snakeycas!(UpperCamelCase);
    assert_eq!(OUT, "upper_camel_case");
}
#[test]
fn lower() {
    struct lowerCamelCase;
    const OUT: &str = snakeycas!(lowerCamelCase);
    assert_eq!(OUT, "lower_camel_case");
}
#[test]
fn snake() {
    struct snake_case;
    const OUT: &str = snakeycas!(snake_case);
    assert_eq!(OUT, "snake_case");
}
#[test]
fn singleletter() {
    struct L;
    const OUT: &str = snakeycas!(L);
    assert_eq!(OUT, "l");
}
#[test]
fn manycaps() {
    struct WOOOO;
    const OUT: &str = snakeycas!(WOOOO);
    assert_eq!(OUT, "w_o_o_o_o");
}
