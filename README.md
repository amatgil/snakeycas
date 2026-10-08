# Snakeycas
Turns identifiers into `const`, `snake_case` string slices.

Behold, the one-and-only usage:
```rust
struct UpperCamelCase;
struct lowerCamelCase;
struct snake_case;

// Note the const-ness
const UPPER: &str = snakeycas!(UpperCamelCase);
assert_eq!(UPPER, "upper_camel_case");

const LOWER: &str = snakeycas!(lowerCamelCase);
assert_eq!(LOWER, "lower_camel_case");

const NOOP: &str = snakeycas!(snake_case);
assert_eq!(NOOP, "snake_case");
}
```

This probably has one (maybe even two!) uses overall.
