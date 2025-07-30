use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Expr, Item, LitStr, Token};
use checker::check_refinement;
use parser::parse_expr;

struct RefineArgs {
    refinement: LitStr,
    _comma: Token![,],
    value: Expr,
}

impl Parse for RefineArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(RefineArgs {
            refinement: input.parse()?,
            _comma: input.parse()?,
            value: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn refine(input: TokenStream) -> TokenStream {
    let RefineArgs { refinement, value, .. } = parse_macro_input!(input as RefineArgs);
    let refinement_str = refinement.value();
    if refinement_str.trim().is_empty() {
        return quote! {
            compile_error!("Refinement cannot be empty");
        }.into();
    }
    let refinement_ast = parse_expr(&refinement_str).expect("Failed to parse refinement expression");
    let result = check_refinement(&refinement_ast, &value);
    if let Err(e) = result {
        quote! { compile_error!(#e); }.into()
    } else {
        quote! { #value }.into()
    }
}

#[proc_macro_attribute]
pub fn refinement(attr: TokenStream, item: TokenStream) -> TokenStream {
    let refinement_lit = parse_macro_input!(attr as LitStr);
    let refinement_str = refinement_lit.value();
    match parse_macro_input!(item as Item) {
        Item::Const(mut item) => { // wrap the const’s initializer in a refine! macro call
            let orig = *item.expr;
            item.expr = Box::new(syn::parse_quote! {
                refine!(#refinement_str, #orig)
            });
            quote!(#item).into()
        }
        _ => quote! {
            compile_error!("The macro attribute `#[refinement(...)]` cannot be used here");
        }.into()
    }
}
