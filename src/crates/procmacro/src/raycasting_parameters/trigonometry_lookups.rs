use proc_macro2::TokenStream;
use quote::quote;

use crate::fixed_point_functions::*;
use super::ArgumentTokens;

pub fn generate_trigonometry_lookups(args: &ArgumentTokens) -> TokenStream {
    let sine_lookup_size_degrees: u16 = args.sine_lookup_size_degrees
        .base10_parse()
        .expect("Sine lookup table's size must be a literal number of degrees");

    assert!(
        sine_lookup_size_degrees == 90 || sine_lookup_size_degrees == 360,
        "Sine lookup table's size must be either a quarter or a full rotation, ie. 90 or 360 degrees");

    let sine_lookup_msbs: u8 = args.sine_lookup_msbs
        .base10_parse()
        .expect("Sine lookup table's Most Significant Bits must be a literal number of bits");

    assert!(
        matches! { (sine_lookup_size_degrees, sine_lookup_msbs), (90, 6..=14) | (360, 8..=16) },
        "Sine lookup table's Most Significant Bits must be in the range [6, 14] for a single quadrant or [8, 16] for a full table");

    let sine_lookup_size_entries: usize = 1 << sine_lookup_msbs;
    let sine_lookup_step_degrees = sine_lookup_size_degrees as f64 / sine_lookup_size_entries as f64;
    let mut sine_lookup_fixed = Vec::<I1F15>::with_capacity(sine_lookup_size_entries);
    let mut sine_lookup_bits = Vec::<i16>::with_capacity(sine_lookup_size_entries);
    for i in 0..sine_lookup_size_entries {
        let angle_degrees = i as f64 * sine_lookup_step_degrees;
        let sine = angle_degrees.to_radians().sin();
        let sine = fixed_normalised_i1f15_with_smallest_error_for(
            if sine <= -1.0 { -0.999999 } else if sine >= 1.0 { 0.999999 } else { sine });

        sine_lookup_fixed.push(sine);
        sine_lookup_bits.push(sine.to_bits());
    }

    let type_ident = &args.type_ident;
    let angle_ident = quote! { ::notray_engine::raycasting::Angle };
    let i1f15_ident = quote! { ::fixed::types::I1F15 };

    let sine_lookup_code = if sine_lookup_size_degrees == 90 {
        quote! {
            static LOOKUP: [#i1f15_ident; #sine_lookup_size_entries as usize] = [
                #(#i1f15_ident::from_bits(#sine_lookup_bits)),*
            ];

            const QUADRANT_MASK: u16 = 0xc000;
            const HALF_INDEX: u16 = (1 << (14 - #sine_lookup_msbs)) / 2;
            let angle = angle.to_fixed_point().to_bits() as u16;
            let index = ((angle & !QUADRANT_MASK) >> (14 - #sine_lookup_msbs)) as usize;
            if angle & 0x4000 != 0 {
                let index = LOOKUP.len() - index - 1;
                let sine = LOOKUP[index];
                if angle & HALF_INDEX != 0 && index != 0 {
                    let next_sine = LOOKUP[index - 1];
                    let half_diff = (next_sine - sine).unbounded_shr(1);
                    let sine = sine + half_diff;
                    if angle & 0x8000 != 0 { -sine } else { sine }
                } else {
                    if angle & 0x8000 != 0 { -sine } else { sine }
                }
            } else {
                    let sine = LOOKUP[index];
                    if angle & HALF_INDEX != 0 && index != LOOKUP.len() - 1 {
                        let next_sine = LOOKUP[index + 1];
                        let half_diff = (next_sine - sine).unbounded_shr(1);
                        let sine = sine + half_diff;
                        if angle & 0x8000 != 0 { -sine } else { sine }
                    } else {
                        if angle & 0x8000 != 0 { -sine } else { sine }
                    }
            }
        }
    } else {
        quote! {
            static LOOKUP: [#i1f15_ident; #sine_lookup_size_entries as usize] = [
                #(#i1f15_ident::from_bits(#sine_lookup_bits)),*
            ];

            const HALF_INDEX: u16 = (1 << (16 - #sine_lookup_msbs)) / 2;
            let angle = angle.to_fixed_point().to_bits() as u16;
            let index = (angle >> (16 - #sine_lookup_msbs)) as usize;
            let sine = LOOKUP[index];

            if angle & HALF_INDEX != 0 {
                let next_index = if index == LOOKUP.len() - 1 { 0 } else { index + 1 };
                let next_sine = LOOKUP[next_index];
                let half_diff = (next_sine - sine).unbounded_shr(1);
                sine + half_diff
            } else {
                sine
            }
        }
    };

    quote! {
        impl ::notray_engine::raycasting::Trigonometry for #type_ident {
            fn cosine(angle: #angle_ident) -> #i1f15_ident {
                Self::sine(angle + #angle_ident::from_raw(0x4000))
            }

            fn sine(angle: #angle_ident) -> #i1f15_ident {
                #sine_lookup_code
            }
        }
    }
}
