pub use fixed::types::{I1F15, I1F15 as Angle};
use proc_macro2::TokenStream;
use quote::quote;

pub fn angle_from_degrees(value: f64) -> TokenStream {
    let normalised_angle = normalised_angle_from_degrees(value);
    let as_fixed = fixed_normalised_angle_with_smallest_error_for(normalised_angle).to_string();
    quote! { ::notray_engine::raycasting::Angle::lit(#as_fixed) }
}

pub fn normalised_angle_from_degrees(value: f64) -> f64 {
    let normalised_degrees = (value / 360.0).abs().fract();
    if normalised_degrees < 0.5 {
        normalised_degrees / 0.5
    } else {
        (normalised_degrees - 0.5) / -0.5
    }
}

pub fn fixed_normalised_angle_with_smallest_error_for(value: f64) -> Angle {
    assert!(value >= -1.0 && value <= 1.0, "Normalised Angles must be in the range [-1, 1]");
    let epsilon = 1.0 / 65536.0;
    smallest_error_for(
        value,
        Angle::from_bits(((value - epsilon) * 32768.0).min(32767.4).max(-32767.4) as i16),
        Angle::from_bits((value * 32768.0).min(32767.4).max(-32767.4) as i16),
        Angle::from_bits(((value + epsilon) * 32768.0).min(32767.4).max(-32767.4) as i16))
}

fn smallest_error_for<T>(value: f64, a: T, b: T, c: T) -> T where T: Copy + Into<f64> {
    let as_float: (f64, f64, f64) = (a.into(), b.into(), c.into());
    let error = (as_float.0 - value, as_float.1 - value, as_float.2 - value);
    let error = (error.0 * error.0, error.1 * error.1, error.2 * error.2);

    if error.0 < error.1 {
        if error.0 < error.2 {
            a
        } else {
            c
        }
    } else {
        if error.1 < error.2 {
            b
        } else {
            c
        }
    }
}

pub fn fixed_normalised_i1f15_with_smallest_error_for(value: f64) -> I1F15 {
    assert!(value > -1.0 && value < 1.0, "Normalised I1F15 must be in the range (-1, 1)");
    let epsilon = 1.0 / 65536.0;
    smallest_error_for(
        value,
        I1F15::from_bits(((value - epsilon) * 32768.0).min(32767.4).max(-32767.4) as i16),
        I1F15::from_bits((value * 32768.0).min(32767.4).max(-32767.4) as i16),
        I1F15::from_bits(((value + epsilon) * 32768.0).min(32767.4).max(-32767.4) as i16))
}
