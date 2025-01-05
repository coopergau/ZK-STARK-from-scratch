extern crate ff;

mod trace;
mod finite_field;
mod utils;
mod prover;

use crate::finite_field::field_params::Fp;
use crate::ff::PrimeField;
use crate::trace::create_trace::mimc_output;
use crate::prover::proof;

const MIMC_ROUNDS: u32 = 127;

fn main() {
    // Private input to the proof
    let private_input = Fp::from_u128(5);
    let public_input = mimc_output(&private_input, MIMC_ROUNDS);
    proof::generate_proof(private_input, public_input);

    
    
   
}
