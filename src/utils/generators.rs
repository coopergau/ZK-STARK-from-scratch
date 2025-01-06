use crate::finite_field::field_params::Fp;
use crate::ff::{PrimeField, Field};

// Returns a subgroup generator of the form 7^{(p-1) / group_order}.
pub fn get_generator<F: PrimeField>(group_order: F) -> F {
    let generator_exponent_repr = (-F::ONE * group_order.invert().unwrap()).to_repr();
    let generator_exponent_bytes = generator_exponent_repr.as_ref();
    let generator_exponent = bytes_to_u64(generator_exponent_bytes);
    let generator = F::MULTIPLICATIVE_GENERATOR.pow(generator_exponent);

    generator
}

// Converts an array of bytes to an array of u64s. Necessary for the pow() function used in get_generator().
pub fn bytes_to_u64(bytes: &[u8]) -> [u64; 4] {
    let mut result = [0u64; 4];
    let chunks = bytes.chunks(8);
    
    let mut index = 0;
    for chunk in chunks {
        let mut array = [0u8; 8];
        for (i, &byte) in chunk.iter().enumerate() {
            array[i] = byte;
        }
        
        result[index] = u64::from_le_bytes(array); 
        index += 1;
    }

    result
}