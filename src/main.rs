extern crate ff;

mod trace;
mod finite_field;
mod utils;

use crate::finite_field::field_params::Fp;
use crate::ff::PrimeField;
use crate::utils::fft;

use crate::ff::Field;

const MIMC_ROUNDS: u32 = 127;

fn main() {
    // Private input to the proof
    // let private_input = Fp::from_u128(5);
    let g = Fp::from_u128(4);
    let inverse_g = Fp::from_u128(13);
    let poly_coeffs: Vec<Fp> = vec![Fp::from_u128(5), Fp::from_u128(1), Fp::from_u128(13), Fp::from_u128(16)];
    let poly_evaluations: Vec<Fp> = vec![Fp::from_u128(1), Fp::from_u128(0), Fp::from_u128(1), Fp::from_u128(1)];
    let bruh = fft::evaluate_poly(poly_coeffs, g);
    println!("{:?}", bruh);
    
    
    //un comment get mimc constants 
    
}
