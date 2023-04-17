use core::fmt;
use core::fmt::Display;
use std::ops;

use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MoneroAmount {
    piconero: u64,
}

impl Serialize for MoneroAmount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MoneroAmount", 1)?;
        state.serialize_field("piconero", &self.piconero)?;
        state.end()
    }
}
impl MoneroAmount {
    pub fn from_xmr(xmr_amount: f64) -> Self {
        let piconero = (xmr_amount * f64::powf(10.0, 12.0)) as u64;
        Self { piconero }
    }

    pub fn from_piconero(piconero: u64) -> Self {
        Self { piconero }
    }

    #[allow(non_snake_case)]
    pub fn as_XMR(&self) -> f64 {
        (self.piconero as f64) / (u64::pow(10, 12) as f64)
    }

    pub fn as_piconero(&self) -> u64 {
        self.piconero
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        self.piconero.to_le_bytes()
    }
}

impl Display for MoneroAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Monero Amount: {} XMR, {} piconero",
            self.as_XMR(),
            self.as_piconero()
        )?;
        Ok(())
    }
}

impl ops::Add<Self> for MoneroAmount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            piconero: self.piconero + rhs.piconero,
        }
    }
}

impl ops::AddAssign for MoneroAmount {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            piconero: self.piconero + other.piconero,
        }
    }
}

impl ops::Sub for MoneroAmount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            piconero: self.piconero - rhs.piconero,
        }
    }
}

impl ops::Mul for MoneroAmount {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            piconero: self.piconero * rhs.piconero,
        }
    }
}

impl ops::Mul<f64> for MoneroAmount {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            piconero: ((self.piconero as f64) * rhs) as u64,
        }
    }
}
