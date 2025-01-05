use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

#[proc_macro]
pub fn my_format(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as syn::ExprCall);

    // Extract the format string and the arguments
    let format_string = if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = &*input.args[0] {
        s.value()
    } else {
        panic!("Expected a string literal as the first argument to my_format!");
    };

    // Generate the formatted string
    let mut format_expr = quote! {
        format!(#format_string)
    };

    for (i, arg) in input.args.iter().enumerate().skip(1) {
        let index = syn::Index::from(i - 1);
        format_expr = quote! {
            #format_expr, #arg
        };
    }

    TokenStream::from(format_expr)
}