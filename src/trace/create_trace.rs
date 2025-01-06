use crate::finite_field::field_params::{Fp, FpRepr};
use crate::ff::{Field, PrimeField};
use once_cell::sync::Lazy;

const CUBE_EXPONENT: [u64; 1] = [3];

// Random field element
pub static MIMC_CONSTANT: Lazy<Fp> = Lazy::new(|| 
    Fp::from_repr(FpRepr([227, 190, 227, 225, 200, 156, 236, 3, 195, 252, 118, 37, 96, 137, 37, 7, 77, 135, 22, 156, 67, 213, 4, 3, 212, 248, 68, 119, 61, 185, 90, 0])).unwrap(),
);

// Function creates the trace of consisting of the input, every subsequent erm, and the output of the mimc hash function
pub fn mimc_trace(input: Fp, rounds: u32) -> Vec<Fp> {
    let mut hash_value = input;
    let mut trace = vec![input];
    for i in 0..rounds as usize {
        hash_value = (hash_value + *MIMC_CONSTANT).pow(CUBE_EXPONENT);
        trace.push(hash_value);
    }
    trace
}

/* This function is not used directly in proof generation.
   It is used to get the output that a user would use in their proof.
   To create a proof, the user has to publicly submit the output that their private input produces. */
#[allow(dead_code)]
pub fn mimc_output(input: &Fp, rounds: u32) -> Fp {
    let mut hash_value = input.clone();
    for i in 0..rounds as usize {
        hash_value = (hash_value + *MIMC_CONSTANT).pow(CUBE_EXPONENT);
    }
    hash_value
}