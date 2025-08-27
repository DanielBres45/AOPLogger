use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Error, Ident, Token};

// Custom struct to parse two comma-separated identifiers
struct TwoIdents {
    name: Ident,
    _comma: Token![,],
    const_var: Ident
}

impl Parse for TwoIdents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(TwoIdents {
            name: input.parse()?, //? auto returns error
            _comma: input.parse()?,
            const_var: input.parse()?
        })
    }
}

#[proc_macro]
pub fn def_log(input: TokenStream) -> TokenStream {
    // Parse the input tokens - now expecting "name, const_var" format
    let TwoIdents { name, const_var, .. } = parse_macro_input!(input as TwoIdents);
    
    let name_str = name.to_string();
    let debug_macro_name = format!("{}_log", &name_str.to_lowercase());
    let debug_macro_ident = Ident::new(&debug_macro_name, name.span());
    let trace_macro_name = format!("{}_trace", &name_str.to_lowercase());
    let trace_macro_ident = Ident::new(&trace_macro_name, name.span());
    let data_macro_name = format!("{}_data", &name_str.to_lowercase());
    let data_macro_ident = Ident::new(&data_macro_name, name.span());
    let expanded = quote! {
        
        #[macro_export]
        macro_rules! #debug_macro_ident {
        ($fmt:literal, || $closure:expr) => { //|| to match closures without arguments...
            if #const_var {
                log::debug!(target: #name_str, $fmt, $closure);
            }
        };
        
        ($fmt:literal, |$($param:ident),*| $closure:expr) => {
            if CAMERA_DEBUG {
                log::debug!(target: "camera", $fmt, $($param),* $closure);
            }
        };

        ($($arg:tt)*) => {
            if #const_var {
                log::debug!(target: #name_str, $($arg)*);
            }
        };
    }

        #[macro_export]
        macro_rules! #trace_macro_ident {
            (($arg:tt)*) => {
                if #const_var {
                    log::trace!(target: #name, $($arg)*);
                }
            };
        }

        // #[macro_export]
        // macro_rules! #data_macro_ident {
        //     ($arg:expr) => {
        //         #[cfg(all(feature = #name, feature = "serde"))]
        //         {
        //             let val = match serde_json::to_string($arg) {
        //                 Ok(v) => v,
        //                 Err(e_msg) => panic!("Could not serialize data: {}", e_msg)
        //             };

        //             log::debug!(target: #name, data = val);
        //         }
        //     };

        //     ($arg:expr, ($msg:tt)*) => {
        //         #[cfg(all(feature = #name, feature = "serde"))]
        //         {
        //             let val = match serde_json::to_string($arg) {
        //                 Ok(v) => v,
        //                 Err(e_msg) => panic!("Could not serialize data: {}", e_msg)
        //             };

        //             log::debug!(target: #name, data = val; $msg);
        //         }
        //     };
        // }
    };

    TokenStream::from(expanded)
}
