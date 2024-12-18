extern crate ff;

mod trace;
mod finite_field;
mod utils;

use crate::finite_field::field_params::Fp;
use crate::ff::PrimeField;
use crate::utils::fft;

const MIMC_ROUNDS: u32 = 127;

fn main() {
    // Private input to the proof
    // let private_input = Fp::from_u128(5);
    let g: i32 = 4;
    let inverse_g: i32 = 13;
    let poly_coeffs: Vec<i32> = vec![5, 1, 13, 16];
    let poly_evaluations: Vec<i32> = vec![1, 0, 1, 1];
    let bruh = fft::interpolate_poly(poly_evaluations, g);
    println!("{:?}", bruh);
    
    let yuh = -8 % 17;
    println!("{:?}", yuh);
    
    
    
}
