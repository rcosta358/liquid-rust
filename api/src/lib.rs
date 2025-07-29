use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, Expr, LitStr, Token};
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
    let RefineArgs { refinement, value, .. } = syn::parse_macro_input!(input as RefineArgs);
    let refinement_str = refinement.value();
    let refinement_ast = parse_expr(&refinement_str).expect("Failed to parse refinement expression");
    let unsatisfied = check_refinement(&refinement_ast, &value);
    let expanded = if unsatisfied {
        quote! { compile_error!("Value does not satisfy the refinement"); }
    } else {
        quote! { #value }
    };
    expanded.into()
}
