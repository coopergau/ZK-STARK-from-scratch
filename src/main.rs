#[macro_use]
extern crate ff;

mod trace;
mod finite_field;
mod utils;

fn main() {
    //trace::mimc(1, 127);
    //utils::mimc_constants::generate_random_mimc_constants(127);
    finite_field::find_field_params::find_primitive_element();
}
