use proc_macro::TokenStream;
use syn::{parse_macro_input, LitFloat};

mod fixed_point_functions;
mod raycasting_parameters;

#[proc_macro]
pub fn angle_from_degrees(items: TokenStream) -> TokenStream {
    let value = parse_macro_input!(items as LitFloat).base10_parse().unwrap();
    fixed_point_functions::angle_from_degrees(value).into()
}

#[proc_macro]
pub fn _raycasting_parameters(items: TokenStream) -> TokenStream {
    let args = parse_macro_input!(items as raycasting_parameters::ArgumentTokens);
    raycasting_parameters::raycasting_parameters(args).into()
}
