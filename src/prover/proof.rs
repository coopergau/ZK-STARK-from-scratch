use crate::finite_field::field_params::Fp;
use crate::ff::PrimeField;
use crate::MIMC_ROUNDS;
use crate::trace::create_trace;
use crate::utils::generators;


pub fn generate_proof(mimc_input: Fp, mimc_output: Fp) {
    // Generate trace.
    let trace = create_trace::mimc_trace(mimc_input, MIMC_ROUNDS);
    
    // Interpolate the polynomial f that maps each element of the domain G to a corresponding trace value.
    let g_order = Fp::from_u128(128);
    let g_generator = generators::get_generator(g_order);
    println!("{:?}", g_generator);
}