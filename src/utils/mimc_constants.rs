use rand::rngs::OsRng;
use crate::finite_field::field_params::Fp;
use crate::ff::Field;

pub fn generate_random_mimc_constants(size: u32) {
    for _ in 1..=size {
        let random_field_element = Fp::random(OsRng);
        println!("{:?}", random_field_element);
    }
}