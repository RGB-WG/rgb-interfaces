// Collection of the standard RGB smart contract interface
//
// SPDX-License-Identifier: Apache-2.0
//
// Designed in 2019-2025 by RGB Consortium members & contributors
// Written in 2024-2025 by Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2019-2025 RGB Consortium members & contributors
// All rights under the above copyrights are reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
// or implied. See the License for the specific language governing permissions and limitations under
// the License.

use std::iter::Sum;

use strict_types::{StrictDeserialize, StrictSerialize, StrictVal};

use crate::LIB_NAME_RGB_CONTRACT;

#[derive(
    Wrapper, WrapperMut, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, From
)]
#[wrapper(Add, Sub, Mul, Div, Rem, Display, FromStr)]
#[wrapper_mut(AddAssign, SubAssign, MulAssign, DivAssign, RemAssign)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct Amount(
    #[from]
    #[from(u32)]
    #[from(u16)]
    #[from(u8)]
    u64,
);

impl StrictSerialize for Amount {}
impl StrictDeserialize for Amount {}

impl Amount {
    pub const ZERO: Self = Amount(0);

    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self {
        value.unwrap_uint::<u64>().into()
    }

    pub fn with_precision(amount: u64, precision: impl Into<Precision>) -> Self {
        precision.into().unchecked_convert(amount)
    }

    pub fn with_precision_checked(amount: u64, precision: impl Into<Precision>) -> Option<Self> {
        precision.into().checked_convert(amount)
    }

    pub fn value(self) -> u64 { self.0 }

    pub fn split(self, precision: impl Into<Precision>) -> (u64, u64) {
        let precision = precision.into();
        let int = self.floor(precision);
        let fract = self.rem(precision);
        (int, fract)
    }

    pub fn round(&self, precision: impl Into<Precision>) -> u64 {
        let precision = precision.into();
        let mul = precision.multiplier();
        if self.0 == 0 {
            return 0;
        }
        let inc = 2 * self.rem(precision) / mul;
        self.0 / mul + inc
    }

    pub fn ceil(&self, precision: impl Into<Precision>) -> u64 {
        let precision = precision.into();
        if self.0 == 0 {
            return 0;
        }
        let inc = if self.rem(precision) > 0 { 1 } else { 0 };
        self.0 / precision.multiplier() + inc
    }

    pub fn floor(&self, precision: impl Into<Precision>) -> u64 {
        if self.0 == 0 {
            return 0;
        }
        self.0 / precision.into().multiplier()
    }

    pub fn rem(&self, precision: impl Into<Precision>) -> u64 {
        self.0 % precision.into().multiplier()
    }

    pub fn saturating_add(&self, other: impl Into<Self>) -> Self {
        self.0.saturating_add(other.into().0).into()
    }
    pub fn saturating_sub(&self, other: impl Into<Self>) -> Self {
        self.0.saturating_sub(other.into().0).into()
    }

    pub fn saturating_add_assign(&mut self, other: impl Into<Self>) {
        *self = self.0.saturating_add(other.into().0).into();
    }
    pub fn saturating_sub_assign(&mut self, other: impl Into<Self>) {
        *self = self.0.saturating_sub(other.into().0).into();
    }

    #[must_use]
    pub fn checked_add(&self, other: impl Into<Self>) -> Option<Self> {
        self.0.checked_add(other.into().0).map(Self)
    }
    #[must_use]
    pub fn checked_sub(&self, other: impl Into<Self>) -> Option<Self> {
        self.0.checked_sub(other.into().0).map(Self)
    }

    #[must_use]
    pub fn checked_add_assign(&mut self, other: impl Into<Self>) -> Option<()> {
        *self = self.0.checked_add(other.into().0).map(Self)?;
        Some(())
    }
    #[must_use]
    pub fn checked_sub_assign(&mut self, other: impl Into<Self>) -> Option<()> {
        *self = self.0.checked_sub(other.into().0).map(Self)?;
        Some(())
    }
}

impl Sum<u64> for Amount {
    fn sum<I: Iterator<Item = u64>>(iter: I) -> Self {
        iter.fold(Amount::ZERO, |sum, value| sum.saturating_add(value))
    }
}

impl Sum for Amount {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Amount::ZERO, |sum, value| sum.saturating_add(value))
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
#[repr(u8)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT, tags = repr, into_u8, try_from_u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub enum Precision {
    Indivisible = 0,
    Deci = 1,
    Centi = 2,
    Milli = 3,
    DeciMilli = 4,
    CentiMilli = 5,
    Micro = 6,
    DeciMicro = 7,
    #[default]
    CentiMicro = 8,
    Nano = 9,
    DeciNano = 10,
    CentiNano = 11,
    Pico = 12,
    DeciPico = 13,
    CentiPico = 14,
    Femto = 15,
    DeciFemto = 16,
    CentiFemto = 17,
    Atto = 18,
}
impl StrictSerialize for Precision {}
impl StrictDeserialize for Precision {}

impl Precision {
    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self { value.unwrap_enum() }
    pub const fn decimals(self) -> u8 { self as u8 }

    pub const fn multiplier(self) -> u64 {
        match self {
            Precision::Indivisible => 1,
            Precision::Deci => 10,
            Precision::Centi => 100,
            Precision::Milli => 1000,
            Precision::DeciMilli => 10_000,
            Precision::CentiMilli => 100_000,
            Precision::Micro => 1_000_000,
            Precision::DeciMicro => 10_000_000,
            Precision::CentiMicro => 100_000_000,
            Precision::Nano => 1_000_000_000,
            Precision::DeciNano => 10_000_000_000,
            Precision::CentiNano => 100_000_000_000,
            Precision::Pico => 1_000_000_000_000,
            Precision::DeciPico => 10_000_000_000_000,
            Precision::CentiPico => 100_000_000_000_000,
            Precision::Femto => 1_000_000_000_000_000,
            Precision::DeciFemto => 10_000_000_000_000_000,
            Precision::CentiFemto => 100_000_000_000_000_000,
            Precision::Atto => 1_000_000_000_000_000_000,
        }
    }

    pub fn unchecked_convert(self, amount: impl Into<u64>) -> Amount {
        (amount.into() * self.multiplier()).into()
    }

    pub fn checked_convert(self, amount: impl Into<u64>) -> Option<Amount> {
        amount
            .into()
            .checked_mul(self.multiplier())
            .map(Amount::from)
    }
    pub fn saturating_convert(self, amount: impl Into<u64>) -> Amount {
        amount.into().saturating_mul(self.multiplier()).into()
    }
}

impl From<Precision> for u16 {
    fn from(value: Precision) -> Self { value as u8 as u16 }
}

impl From<Precision> for u32 {
    fn from(value: Precision) -> Self { value as u8 as u32 }
}

impl From<Precision> for u64 {
    fn from(value: Precision) -> Self { value as u8 as u64 }
}
