extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

//TODO: Change macro to not time, and only create the method tracer
//We should have another macro which is responsible for timing the function.
//So that we can give the function attributes which look like
//#[trace]
//#[time]
//fn foo(){}
//
//Or if we didn't want to time the function (for whatever reason)
//we could just do
//#[trace]
//fn foo() {}
//We could just use the logger timestamps,
//but I want to be extremly precise for hot call functions which run very fast.
#[proc_macro_attribute]
pub fn time(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_args = &input.sig.inputs;
    let fn_block = &input.block;
    let fn_output = &input.sig.output;
    let fn_visibility = &input.vis;

    let expanded = match cfg!(feature = "Time") {
        true => quote! {
            #fn_visibility fn #fn_name(#fn_args) #fn_output {
                let tracer = MethodTracer::new(stringify!(#fn_name), line!());
                let result = (|| #fn_block)();
                tracer.dispose();
                result
            }
        },
        false => quote! {
        #fn_visibility fn #fn_name(#fn_args) #fn_output {
            #fn_block
        }
        },
    };

    TokenStream::from(expanded)
}

