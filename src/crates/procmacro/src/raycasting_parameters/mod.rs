use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitFloat, LitInt, Token};
use syn::parse::{Parse, ParseStream};

mod projection_plane_parameters;
use projection_plane_parameters::*;

mod trigonometry_lookups;
use trigonometry_lookups::*;

pub struct ArgumentTokens {
    type_ident: Ident,
    _delimiter_1: Token![,],
    canvas_width_pixels: LitInt,
    _delimiter_2: Token![,],
    canvas_height_pixels: LitInt,
    _delimiter_3: Token![,],
    field_of_view_degrees: LitFloat,
    _delimiter_4: Token![,],
    sine_lookup_msbs: LitInt,
    _delimiter_5: Token![,],
    sine_lookup_size_degrees: LitInt,
    no_extra_tokens: bool
}

impl Parse for ArgumentTokens {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            type_ident: input.parse()?,
            _delimiter_1: input.parse()?,
            canvas_width_pixels: input.parse()?,
            _delimiter_2: input.parse()?,
            canvas_height_pixels: input.parse()?,
            _delimiter_3: input.parse()?,
            field_of_view_degrees: input.parse()?,
            _delimiter_4: input.parse()?,
            sine_lookup_msbs: input.parse()?,
            _delimiter_5: input.parse()?,
            sine_lookup_size_degrees: input.parse()?,
            no_extra_tokens: input.is_empty()
        })
    }
}

pub fn raycasting_parameters(args: ArgumentTokens) -> TokenStream {
    assert!(args.no_extra_tokens, "Too many tokens passed to macro");

    let projection_plane_parameters = generate_projection_plane_parameters(&args);
    let trigonometry_lookups = generate_trigonometry_lookups(&args);

    let type_ident = &args.type_ident;
    let engine_parameters_ident = quote! { ::notray_engine::raycasting::EngineParameters };
    let world_absolute_coordinate_ident = quote! { ::notray_engine::raycasting::WorldAbsoluteCoordinate };

    quote! {
        pub struct #type_ident;

        impl #engine_parameters_ident for #type_ident {
            const MAX_RAY_DISTANCE: #world_absolute_coordinate_ident = #world_absolute_coordinate_ident::lit("64");
        }

        #projection_plane_parameters
        #trigonometry_lookups
    }
}
