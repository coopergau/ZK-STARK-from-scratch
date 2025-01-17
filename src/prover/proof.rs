use crate::finite_field::field_params::Fp;
use crate::ff::{PrimeField, Field};
use crate::{MIMC_ROUNDS, G_DOMAIN_SIZE, L_DOMAIN_SIZE};
use crate::trace::create_trace;
use crate::utils::{generators, fft};
use crate::polynomials::poly::Polynomial;
use super::constraints::calculate_constraint_polys;
use super::composition_poly::calculate_composition_poly;


pub fn generate_proof(mimc_input: Fp, mimc_output: Fp) {
    // Generate trace.
    let trace = create_trace::mimc_trace(mimc_input, MIMC_ROUNDS);
    
    // Interpolate the polynomial f that maps each element of the domain G to a corresponding trace value.
    let g_order = Fp::from(G_DOMAIN_SIZE);
    let g_generator = generators::get_generator(g_order);
    let g_generator_inverse = g_generator.invert().unwrap();
    let mut f_poly_coeffs = fft::interpolate_poly(&trace, g_generator_inverse);
    
    // Low degree extension (LDE) - Evaluate the polynomial over the larger domain L.
    let l_order = Fp::from(L_DOMAIN_SIZE);
    let l_generator = generators::get_generator(l_order);
    f_poly_coeffs.resize(L_DOMAIN_SIZE as usize, Field::ZERO); // Add padding in order to extend the evaluation domain.
    let f_evals_over_extended_domain = fft::evaluate_poly(&f_poly_coeffs, l_generator);
    
    // Commit to the LDE of the polynomial f.
    
    // Compute the constraint polynomials c_1, c_2, and c_3.
    let f_poly = Polynomial::new(&f_poly_coeffs);
    let (c_1, c_2, c_3) = calculate_constraint_polys(&mimc_input, &mimc_output, f_poly, &g_generator);
    
    // Compute the composition polynomial p. 
    let composition_poly = calculate_composition_poly(&c_1, &c_2, &c_3, &g_generator); // Update whether these should be references or not

    // Commit to the polynomial p.
}