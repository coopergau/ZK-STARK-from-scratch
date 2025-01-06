use crate::finite_field::field_params::Fp;
use crate::ff::{PrimeField, Field};
use crate::{MIMC_ROUNDS, G_DOMAIN_SIZE, L_DOMAIN_SIZE};
use crate::trace::create_trace;
use crate::utils::{generators, fft};
use super::constraints::calculate_constraint_polys;


pub fn generate_proof(mimc_input: Fp, mimc_output: Fp) {
    // Generate trace.
    let trace = create_trace::mimc_trace(mimc_input, MIMC_ROUNDS);
    
    // Interpolate the polynomial f that maps each element of the domain G to a corresponding trace value.
    let g_order = Fp::from(G_DOMAIN_SIZE);
    let g_generator = generators::get_generator(g_order);
    let g_generator_inverse = g_generator.invert().unwrap();
    let mut f_poly_coeffs = fft::interpolate_poly(trace, g_generator_inverse);
    
    // Low degree extension (LDE) - Evaluate the polynomial over the larger domain L.
    let l_order = Fp::from(L_DOMAIN_SIZE);
    let l_generator = generators::get_generator(l_order);
    f_poly_coeffs.resize(L_DOMAIN_SIZE as usize, Field::ZERO); // Add padding in order to extend the evaluation domain.
    let l_evaluations = fft::evaluate_poly(f_poly_coeffs, l_generator);
    println!("{:?}", l_evaluations);    
    // Commit to the LDE of the polynomial f.

    // Compute the constraint polynomials c_i. These should all equal zero over the domain G.
    calculate_constraint_polys(mimc_input, mimc_output);

    // Compute the composition polynomial p.

    // Commit to the polynomial p.
}