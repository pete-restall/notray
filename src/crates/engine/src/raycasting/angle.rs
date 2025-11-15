use core::cmp::PartialEq;
use core::ops::{Add, AddAssign, Div, Neg};

use fixed::types::I1F15;

pub use notray_procmacro::angle_from_degrees;

use crate::HasFixedPoint;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Angle(I1F15);

impl HasFixedPoint for Angle {
    type FixedPoint = I1F15;
}

impl Angle {
    pub const QUADRANT0_MIN: Angle = Angle::from_raw(0x0001_u16 as i16);
    pub const QUADRANT0_MAX: Angle = Angle::from_raw(0x3fff_u16 as i16);

    pub const QUADRANT1_MIN: Angle = Angle::from_raw(0x4001_u16 as i16);
    pub const QUADRANT1_MAX: Angle = Angle::from_raw(0x7fff_u16 as i16);

    pub const QUADRANT2_MIN: Angle = Angle::from_raw(0x8001_u16 as i16);
    pub const QUADRANT2_MAX: Angle = Angle::from_raw(0xbfff_u16 as i16);

    pub const QUADRANT3_MIN: Angle = Angle::from_raw(0xc001_u16 as i16);
    pub const QUADRANT3_MAX: Angle = Angle::from_raw(0xffff_u16 as i16);

    pub const QUADRANT_AXIS_0_1: Angle = Angle::from_raw(0x4000_u16 as i16);
    pub const QUADRANT_AXIS_1_2: Angle = Angle::from_raw(0x8000_u16 as i16);
    pub const QUADRANT_AXIS_2_3: Angle = Angle::from_raw(0xc000_u16 as i16);
    pub const QUADRANT_AXIS_3_0: Angle = Angle::from_raw(0x0000_u16 as i16);

    pub const fn default() -> Self {
        Self::lit("0")
    }

    pub const fn lit(value: &'static str) -> Self {
        Self(<Self as HasFixedPoint>::FixedPoint::lit(value))
    }

    pub const fn from_raw(value: i16) -> Self {
        Self(<Self as HasFixedPoint>::FixedPoint::from_bits(value))
    }

    pub const fn from(value: <Self as HasFixedPoint>::FixedPoint) -> Self {
        Self(value)
    }

    pub const fn to_fixed_point(self) -> <Self as HasFixedPoint>::FixedPoint { self.0 }

    pub fn is_quadrant_axis_0_and_1(self) -> bool { self.0 == Self::QUADRANT_AXIS_0_1.0 }
    pub fn is_quadrant_axis_1_and_2(self) -> bool { self.0 == Self::QUADRANT_AXIS_1_2.0 }
    pub fn is_quadrant_axis_2_and_3(self) -> bool { self.0 == Self::QUADRANT_AXIS_2_3.0 }
    pub fn is_quadrant_axis_3_and_0(self) -> bool { self.0 == Self::QUADRANT_AXIS_3_0.0 }

    pub fn is_within_quadrant_0_or_1(self) -> bool { self.0 >= Self::QUADRANT0_MIN.0 && self.0 <= Self::QUADRANT1_MAX.0 }
    pub fn is_within_quadrant_1_or_2(self) -> bool { self.0 >= Self::QUADRANT1_MIN.0 || self.0 <= Self::QUADRANT2_MAX.0 }
    pub fn is_within_quadrant_2_or_3(self) -> bool { self.0 >= Self::QUADRANT2_MIN.0 && self.0 <= Self::QUADRANT3_MAX.0 }
    pub fn is_within_quadrant_3_or_0(self) -> bool { self.0 >= Self::QUADRANT3_MIN.0 || self.0 <= Self::QUADRANT0_MAX.0 }
}

impl PartialEq<Angle> for Angle {
    fn eq(&self, rhs: &Angle) -> bool { self.0 == rhs.0 }
}

impl Add<Angle> for Angle {
    type Output = Angle;

    fn add(self, rhs: Angle) -> Self::Output {
        self + rhs.0
    }
}

impl AddAssign<Angle> for Angle {
    fn add_assign(&mut self, rhs: Angle) { (*self).0 = (*self + rhs).0; }
}

impl Add<<Angle as HasFixedPoint>::FixedPoint> for Angle {
    type Output = Angle;

    fn add(self, rhs: <Angle as HasFixedPoint>::FixedPoint) -> Self::Output {
        Self(self.0.wrapping_add(rhs))
    }
}

impl Neg for Angle {
    type Output = Angle;

    fn neg(self) -> Self::Output {
        Self::Output::from(-self.0)
    }
}

macro_rules! repeat {
    ($macro_op:ident! for [ $arg:ident ]) => {
        $macro_op! { $arg }
    };

    ($macro_op:ident! for [ $head:ident, $($tail:ident),+ ]) => {
        repeat! { $macro_op! for [ $head ] }
        repeat! { $macro_op! for [ $($tail),+ ] }
    };
}

macro_rules! forward_div {
    ($divisor:ident) => {
        impl Div<$divisor> for Angle {
            type Output = Angle;

            fn div(self, rhs: $divisor) -> Self::Output { Self(<<Angle as HasFixedPoint>::FixedPoint as Div<$divisor>>::div(self.0, rhs)) }
        }
    };
}

repeat! { forward_div! for [i16] }

macro_rules! forward_forced_div {
    ($divisor:ident) => {
        impl Div<$divisor> for Angle {
            type Output = Angle;

            fn div(self, rhs: $divisor) -> Self::Output { self / (rhs as i16) }
        }
    };
}

repeat! { forward_forced_div! for [u16, i32] }
