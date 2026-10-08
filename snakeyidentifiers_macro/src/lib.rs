use quote::quote;
use syn::{Ident, parse_macro_input};

fn as_snake_case(input: &str) -> String {
    // God have mercy on you if you try to use non-ascii identifiers.
    assert!(
        input.is_ascii(),
        "only ascii is allowed, i don wanna think about how utf8 considers case"
    );

    let mut output = String::with_capacity(input.len());
    for (i, char) in input.chars().enumerate() {
        if char.is_uppercase() && i > 0 {
            output.push('_');
        }
        output.push(char.to_ascii_lowercase());
    }
    output
}

#[proc_macro]
pub fn snakeycas(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ident: Ident = parse_macro_input!(input as Ident);
    let ident = ident
        .span()
        .source_text()
        .expect("Must correspond to somewhere in source");

    let new_name: String = as_snake_case(&ident);

    quote! { #new_name }.into()
}
