use snakeyidentifiers_macro::snakecas;

#[test]
fn upper() {
    struct UpperCamelCase;
    const UPPER: &str = snakecas!(UpperCamelCase);
    assert_eq!(UPPER, "upper_camel_case");
}
#[test]
fn lower() {
    struct lowerCamelCase;
    const LOWER: &str = snakecas!(lowerCamelCase);
    assert_eq!(LOWER, "lower_camel_case");
}
#[test]
fn snake() {
    struct snake_case;
    const SNAKE: &str = snakecas!(snake_case);
    assert_eq!(SNAKE, "snake_case");
}
