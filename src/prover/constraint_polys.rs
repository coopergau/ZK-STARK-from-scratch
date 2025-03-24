use crate::finite_field::field_params::Fp;
use crate::ff::{PrimeField, Field};
use crate::polynomials::poly::Polynomial;
use crate::trace::create_trace::MIMC_CONSTANT;
use crate::utils::{generators, fft};

pub fn calculate_constraint_polys(
    proof_i: &Fp, 
    proof_o: &Fp, 
    f_poly: &Polynomial<Fp>, 
    subgroup_generator: &Fp
) -> (Polynomial<Fp>, Polynomial<Fp>, Polynomial<Fp>) {
    let c_1 = calculate_basic_constraint_poly(proof_i, f_poly);
    let c_2 = calculate_constraint_poly2(f_poly, &subgroup_generator);
    let c_3 = calculate_basic_constraint_poly(proof_o, f_poly);
    
    (c_1, c_2, c_3)
}

// Creates the constraint polynomial of the form: c(x) = f(x) - constant, for a given constant.
// Used to create c_1(x) = f(x) - I and c_3(x) = f(x) - O, for the mimc input I and output O.
pub fn calculate_basic_constraint_poly(constant: &Fp, f_poly: &Polynomial<Fp>) -> Polynomial<Fp> {
    let constant_vec = vec![*constant];
    let constant_poly = Polynomial::new(&constant_vec);
    let constraint_poly = f_poly.sub(&constant_poly);

    constraint_poly
}

// c_2(x) = f(gx) - (f(x) + k)^3.
// Creates the polynomial by calulating the evaluations of c_2 over the non extended subgroup domain and then uses fft to get coefficients.
pub fn calculate_constraint_poly2(f_poly: &Polynomial<Fp>, g: &Fp) -> Polynomial<Fp> {
    // Create g(x) = 1 + gx + g^2x^2 + g^3x^3 ...
    let mut g_vec = vec![Fp::ONE]; 
    for i in 1..f_poly.len() {
            g_vec.push(g.pow(&[i as u64]));
    }
    // Create f(gx)
    let f_vec = &f_poly.coefficients;
    let fg_vec: Vec<Fp> = f_vec.iter().zip(g_vec.iter()).map(|(a, b)| *a * b).collect();
    let fg_poly = Polynomial::new(&fg_vec);

    // Get roots of unity for fft.
    let f_degree = f_poly.len() - 1;
    let f_length = f_degree.next_power_of_two();
    let f_length_fp = Fp::from(f_length as u64);
    let f_root_of_unity = generators::get_generator(f_length_fp);
    let f_inverse_root_of_unity = f_root_of_unity.invert().unwrap();
    
    // Compute f(x) and f(gx) evaluations.
    let f_evals = fft::evaluate_poly(&f_poly.coefficients, f_root_of_unity);
    let fg_evals = fft::evaluate_poly(&fg_poly.coefficients, f_root_of_unity);

    // Compute c_2(x) evals.
    let cube_exponent: u64 = 3;
    let mut c_evals: Vec<Fp> = f_evals.iter().zip(fg_evals.iter()).map(|(a, b)| *b - (*a + *MIMC_CONSTANT).pow(&[cube_exponent])).collect();
    
    /* Cubing the polynomial f(x) will produce a new polynomial of degree 3 * deg(f(x)).
    So we need to find the smallest power of 2 that is greater than or equal to 3 * deg(f(x)). */
    let new_degree = f_degree * 3;
    let min_length = new_degree - 1;
    let length = min_length.next_power_of_two();
    let length_fp = Fp::from(length as u64);
    let new_root_of_unity = generators::get_generator(length_fp);
    let inverse_new_root_of_unity = new_root_of_unity.invert().unwrap();

    c_evals.resize(length, Fp::ZERO);
    let c_coeffs = fft::interpolate_poly(&c_evals, inverse_new_root_of_unity);

    Polynomial::new(&c_coeffs)
    }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MIMC_ROUNDS, G_DOMAIN_SIZE, L_DOMAIN_SIZE};
    use crate::trace::create_trace::mimc_output;
    use crate::trace::create_trace;

    #[test]
    // Tests that c_1(g^0) == 0 and c_2(g^127) == 0
    fn test_c_1_and_c_3_are_zero_on_correct_trace_elements() {
    // Running almost full proof up to the generation of constraint polys
    
    let mimc_input = Fp::from(5);
    let input_copy = mimc_input.clone();
    let mimc_output = mimc_output(&mimc_input, MIMC_ROUNDS);
    
    // Generate trace.
    let trace = create_trace::mimc_trace(input_copy, MIMC_ROUNDS);
    
    // Interpolate the polynomial f that maps each element of the domain G to a corresponding trace value.
    let g_order = Fp::from(G_DOMAIN_SIZE);
    let g_generator = generators::get_generator(g_order);
    let g_generator_inverse = g_generator.invert().unwrap();
    let mut f_poly_coeffs = fft::interpolate_poly(&trace, g_generator_inverse);
    
    // Skip - Low degree extension (LDE) - Evaluate the polynomial over the larger domain L.
    
    // Skip - Commit to the LDE of the polynomial f.
    
    // Compute the constraint polynomials c_1, c_2, and c_3.
    let f_poly = Polynomial::new(&f_poly_coeffs);
    let (c_1, c_2, c_3) = calculate_constraint_polys(&mimc_input, &mimc_output, &f_poly, &g_generator);
    
    assert!(c_1.evaluate(&Fp::ONE) == Fp::ZERO); 
    assert!(c_3.evaluate(&g_generator.pow(&[127])) == Fp::ZERO);
    
    for i in 1..126 {
        assert!(c_2.evaluate(&g_generator.pow(&[i as u64])) == Fp::ZERO);
    }
    }

}

