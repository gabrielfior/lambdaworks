use crate::field::fields::pasta_field_helpers::{to_bits_le, to_bytes_be, to_bytes_le};
use crate::{
    field::{
        element::FieldElement,
        fields::montgomery_backed_prime_fields::{IsModulus, MontgomeryBackendPrimeField},
    },
    unsigned_integer::element::U256,
};

pub type VestaMontgomeryBackendPrimeField<T> = MontgomeryBackendPrimeField<T, 4>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MontgomeryConfigVesta255PrimeField;
impl IsModulus<U256> for MontgomeryConfigVesta255PrimeField {
    const MODULUS: U256 = U256::from_hex_unchecked(
        "0x40000000000000000000000000000000224698fc0994a8dd8c46eb2100000001",
    );
}

pub type Vesta255PrimeField = VestaMontgomeryBackendPrimeField<MontgomeryConfigVesta255PrimeField>;

impl FieldElement<Vesta255PrimeField> {
    pub fn to_bytes_le(&self) -> [u8; 32] {
        to_bytes_le(&self)
    }

    pub fn to_bits_le(&self) -> [bool; 256] {
        to_bits_le(&self)
    }

    pub fn to_bytes_be(&self) -> [u8; 32] {
        to_bytes_be(&self)
    }
}
