use fixed::traits::ToFixed;
use fixed::types::{U0F16, U11F21, U16F0, U2F30, U32F0};
use proc_macro2::TokenStream;
use quote::quote;

use crate::fixed_point_functions::fixed_normalised_i1f15_with_smallest_error_for;
use super::ArgumentTokens;

pub fn generate_projection_plane_parameters(args: &ArgumentTokens) -> TokenStream {
    let canvas_width_pixels: u16 = args.canvas_width_pixels
        .base10_parse()
        .expect("Canvas Width must be a literal number of pixels");

    assert!(canvas_width_pixels >= 128 && canvas_width_pixels <= 1024, "Canvas Width must be in the range [128, 1024] pixels");

    let canvas_height_pixels: u16 = args.canvas_height_pixels
        .base10_parse()
        .expect("Canvas Height must be a literal number of pixels");

    assert!(canvas_height_pixels >= 64 && canvas_height_pixels <= 1024, "Canvas Height must be in the range [64, 1024] pixels");

    let field_of_view_degrees: f64 = args.field_of_view_degrees
        .base10_parse()
        .expect("Field-of-View must be a literal number of degrees");

    assert!(field_of_view_degrees >= 10.0 && field_of_view_degrees <= 90.0, "Field of view must be in the range [10, 90]");

    let projection_plane_vector_y = fixed_normalised_i1f15_with_smallest_error_for(0.5 * field_of_view_degrees.to_radians().tan());
    let projection_plane_vector_y_bits = projection_plane_vector_y.to_bits();

    let canvas_column_normalising_factor: U0F16 =
        (U2F30::from_num(2) / canvas_width_pixels as u32)
        .checked_to_fixed()
        .expect("Overflow when calculating the column normalising factor");

    let canvas_column_normalising_factor_bits = canvas_column_normalising_factor.to_bits();

    let aspect_ratio_for_wall_height: U11F21 =
        U32F0::from_num(canvas_height_pixels as u32 * canvas_height_pixels as u32)
        .checked_div(U16F0::from_num(canvas_width_pixels).into())
        .expect("Overflow when calculating the aspect ratio for wall heights (1)")
        .checked_to_fixed()
        .expect("Overflow when calculating the aspect ratio for wall heights (2)");

    let aspect_ratio_for_wall_height_bits = aspect_ratio_for_wall_height.to_bits();

    let type_ident = &args.type_ident;
    let i1f15_ident = quote! { ::fixed::types::I1F15 };
    let u0f16_ident = quote! { ::fixed::types::U0F16 };
    let u11f21_ident = quote! { ::fixed::types::U11F21 };

    quote! {
        impl ::notray_engine::raycasting::ProjectionPlaneParameters for #type_ident {
            const CANVAS_WIDTH_PIXELS: u16 = #canvas_width_pixels;
            const CANVAS_HEIGHT_PIXELS: u16 = #canvas_height_pixels;
            const CANVAS_COLUMN_NORMALISING_FACTOR: #u0f16_ident = #u0f16_ident::from_bits(#canvas_column_normalising_factor_bits);

            const PROJECTION_PLANE_VECTOR_Y: #i1f15_ident = #i1f15_ident::from_bits(#projection_plane_vector_y_bits);
            const ASPECT_RATIO_FOR_WALL_HEIGHT: #u11f21_ident = #u11f21_ident::from_bits(#aspect_ratio_for_wall_height_bits);
        }
    }
}
