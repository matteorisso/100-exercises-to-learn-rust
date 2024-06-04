// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;
// use std::ops::Deref;
use std::cmp::PartialEq;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}

// impl Deref for SaturatingU16 {
//     type Target = u16;

//     fn deref(&self) -> &u16 {
//         &self.value
//     }
// }

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {
            value: value.into(),
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 { value: *value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        let val_u16: u16 = (*value).into();
        SaturatingU16 { value: val_u16 }
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(rhs),
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> SaturatingU16 {
        let rhs_u16: u16 = rhs.value;
        SaturatingU16 {
            value: self.value.saturating_add(rhs_u16),
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(*rhs),
        }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
