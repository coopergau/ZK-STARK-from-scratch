use crate::finite_field::field_params::Fp;
use crate::ff::{PrimeField, Field};
use crate::{MIMC_ROUNDS, G_DOMAIN_SIZE, L_DOMAIN_SIZE};
use crate::trace::create_trace;
use crate::utils::{generators, fft};


pub fn generate_proof(mimc_input: Fp, mimc_output: Fp) {
    // Generate trace.
    let trace = create_trace::mimc_trace(mimc_input, MIMC_ROUNDS);
    
    // Interpolate the polynomial f that maps each element of the domain G to a corresponding trace value.
    let g_order = Fp::from_u128(G_DOMAIN_SIZE);
    let g_generator = generators::get_generator(g_order);
    let g_generator_inverse = g_generator.invert().unwrap();
    let mut f_poly = fft::interpolate_poly(trace, g_generator_inverse);
    
    // Evaluate the polynomial over the domain L.
    let l_order = Fp::from_u128(L_DOMAIN_SIZE);
    let l_generator = generators::get_generator(l_order);
    f_poly.resize(L_DOMAIN_SIZE as usize, Field::ZERO); // Add padding in order to extend the domain.
    let l_evaluations = fft::evaluate_poly(f_poly, l_generator);
    

}