use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn def_trace(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Ident);
    let name = input.to_string();
    let macro_name = format!("{}_trace", name.to_lowercase());
    let macro_ident = Ident::new(&macro_name, input.span());

    let expanded = quote! {
        #[macro_export]
        macro_rules! #macro_ident {
            ($($arg:tt)*) => {
                #[cfg(feature = #name)]
                {
                    log::trace!(target: #name, $($arg)*);
                }
            };
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn def_log(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as Ident);
    let name = input.to_string();
    let macro_name = format!("{}_log", name.to_lowercase());
    let macro_ident = Ident::new(&macro_name, input.span());

    let expanded = quote! {
        #[macro_export]
        macro_rules! #macro_ident {
            ($($arg:tt)*) => {
                #[cfg(feature = #name)]
                {
                    log::debug!(target: #name, file_name = file!(), line_number = line!(); $($arg)*);
                }
            };
        }
    };

    TokenStream::from(expanded)
}

