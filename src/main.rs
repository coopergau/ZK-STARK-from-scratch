#[macro_use]
extern crate ff;

mod trace;
mod finite_field;
mod utils;

fn main() {
    //trace::mimc(1, 127);
    utils::get_mimc_constants::generate_random_mimc_constants(127);
}
