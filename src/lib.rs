use napi::bindgen_prelude::*;
use napi_derive::napi;
use num_bigint::RandBigInt;
use num_bigint::{BigInt, Sign};
use num_traits::Zero;
use rand::thread_rng;

#[napi]
pub struct SRPInteger {
  value: BigInt,
  hex_length: Option<usize>,
}

#[napi]
impl SRPInteger {
  #[napi(constructor)]
  pub fn new(hex: String) -> Self {
    let hex_length = hex.len();
    let value = BigInt::parse_bytes(hex.as_bytes(), 16).unwrap_or_else(|| BigInt::zero());
    Self {
      value,
      hex_length: Some(hex_length),
    }
  }

  #[napi]
  pub fn add(&self, other: &SRPInteger) -> SRPInteger {
    SRPInteger {
      value: &self.value + &other.value,
      hex_length: self.hex_length.or(other.hex_length),
    }
  }

  #[napi]
  pub fn subtract(&self, other: &SRPInteger) -> SRPInteger {
    SRPInteger {
      value: &self.value - &other.value,
      hex_length: self.hex_length.or(other.hex_length),
    }
  }

  #[napi]
  pub fn multiply(&self, other: &SRPInteger) -> SRPInteger {
    SRPInteger {
      value: &self.value * &other.value,
      hex_length: self.hex_length.or(other.hex_length),
    }
  }

  #[napi]
  pub fn mod_pow(&self, exponent: &SRPInteger, modulus: &SRPInteger) -> SRPInteger {
    SRPInteger {
      value: self.value.modpow(&exponent.value, &modulus.value),
      hex_length: modulus.hex_length,
    }
  }

  #[napi]
  pub fn modulo(&self, modulus: &SRPInteger) -> SRPInteger {
    let mut result = &self.value % &modulus.value;
    if result.sign() == Sign::Minus {
      result += &modulus.value;
    }
    SRPInteger {
      value: result,
      hex_length: modulus.hex_length,
    }
  }

  #[napi]
  pub fn equals(&self, other: &SRPInteger) -> bool {
    self.value == other.value
  }

  #[napi]
  pub fn xor(&self, other: &SRPInteger) -> SRPInteger {
    SRPInteger {
      value: &self.value ^ &other.value,
      hex_length: self.hex_length.or(other.hex_length),
    }
  }

  #[napi]
  pub fn to_hex(&self) -> Result<String> {
    if let Some(len) = self.hex_length {
      let hex_str = self.value.to_str_radix(16);
      if self.value.sign() == Sign::Minus {
        return Err(Error::from_reason(
          "Negative values cannot be represented in hex",
        ));
      }
      Ok(format!("{:0>width$}", hex_str, width = len))
    } else {
      Err(Error::from_reason(
        "This SRPInteger has no specified length",
      ))
    }
  }

  #[napi(factory)]
  pub fn from_hex(input: String) -> SRPInteger {
    SRPInteger::new(input)
  }

  #[napi(factory)]
  pub fn random_integer(bytes: u32) -> SRPInteger {
    let mut rng = thread_rng();
    let rand_bigint = rng.gen_bigint((bytes * 8) as u64);
    SRPInteger {
      value: rand_bigint,
      hex_length: Some((bytes * 2) as usize),
    }
  }

  #[napi(getter)]
  pub fn inspect(&self) -> String {
    let hex = self.value.to_str_radix(16);
    format!(
      "<SRPInteger {}>",
      if hex.len() > 16 {
        format!("{}...", &hex[0..16])
      } else {
        hex
      }
    )
  }

  #[napi(factory)]
  pub fn zero() -> SRPInteger {
    SRPInteger {
      value: BigInt::zero(),
      hex_length: None,
    }
  }
}
