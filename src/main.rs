extern crate ff;

mod trace;
mod finite_field;

use crate::finite_field::field_params::Fp;
use crate::ff::PrimeField;

const MIMC_ROUNDS: u32 = 127;

fn main() {
    // Private input to the proof
    let private_input = Fp::from_u128(5);
    //let trace = trace::create_trace::mimc(private_input, MIMC_ROUNDS);
    //println!("{:?}", trace);
    finite_field::find_field_params::find_primitive_element();
    finite_field::find_field_params::find_subgroup_generator();
}
