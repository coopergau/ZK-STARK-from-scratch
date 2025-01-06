extern crate ff;

mod trace;
mod finite_field;
mod utils;
mod prover;
mod polynomials;

use crate::finite_field::field_params::Fp;
use crate::ff::PrimeField;
use crate::trace::create_trace::mimc_output;
use crate::prover::proof;
use crate::polynomials::poly;

const MIMC_ROUNDS: u32 = 127;
const G_DOMAIN_SIZE: u64 = 128;
const L_DOMAIN_SIZE: u64 = 4096;

fn main() {
    // Private input to the proof
    let private_input = Fp::from(5);
    let public_input = mimc_output(&private_input, MIMC_ROUNDS);
    //proof::generate_proof(private_input, public_input);

    let coeffs = vec![Fp::from(5), Fp::from(1)];
    let p = poly::Polynomial::new(coeffs);
    
    let coeffs2 = vec![Fp::from(4), Fp::from(3)];
    let q = poly::Polynomial::new(coeffs2);
    
    let sum = p.mul(&q);
    println!("{:?}", sum);
   
}
