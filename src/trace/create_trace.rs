use super::get_mimc_constants::MIMC_CONSTANTS;
use crate::finite_field::field_params::Fp;
use crate::ff::Field;

const CUBED_EXPONENT: [u64; 1] = [3];

pub fn mimc(input: Fp, rounds: u32) -> Vec<Fp> {
    let mut hash_value = input;
    let mut trace = vec![input];
    for i in 0..rounds as usize {
        hash_value = (hash_value + MIMC_CONSTANTS[i]).pow(CUBED_EXPONENT);
        trace.push(hash_value);
    }
    trace
}
