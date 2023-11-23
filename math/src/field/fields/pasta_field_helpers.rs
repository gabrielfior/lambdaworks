use crate::field::fields::montgomery_backed_prime_fields::MontgomeryBackendPrimeField;

pub fn to_bytes_le<T, const N: usize>(
    field_element: &MontgomeryBackendPrimeField<T, N>,
) -> [u8; 32] {
    let limbs = field_element.representative().limbs;
    let mut bytes: [u8; 32] = [0; 32];

    for i in (0..4).rev() {
        let limb_bytes = limbs[i].to_le_bytes();
        for j in 0..8 {
            // i = 3 ->
            bytes[(3 - i) * 8 + j] = limb_bytes[j]
        }
    }
    bytes
}

pub fn to_bits_le<T, const N: usize>(
    field_element: &MontgomeryBackendPrimeField<T, 4>,
) -> [bool; 256] {
    let limbs = field_element.representative().limbs;
    let mut bits = [false; 256];

    for i in (0..4).rev() {
        let limb_bytes = limbs[i].to_le_bytes();
        let limb_bytes_starting_index = (3 - i) * 8;
        for (j, byte) in limb_bytes.iter().enumerate() {
            let byte_index = (limb_bytes_starting_index + j) * 8;
            for k in 0..8 {
                let bit_index = byte_index + k;
                let bit_value = (byte >> k) & 1 == 1;
                bits[bit_index] = bit_value;
            }
        }
    }
    bits
}

pub fn to_bytes_be<T, const N: usize>(
    field_element: &MontgomeryBackendPrimeField<T, 4>,
) -> [u8; 32] {
    let limbs = field_element.representative().limbs;
    let mut bytes: [u8; 32] = [0; 32];

    for i in 0..4 {
        let limb_bytes = limbs[i].to_be_bytes();
        for j in 0..8 {
            bytes[i * 8 + j] = limb_bytes[j]
        }
    }
    bytes
}
