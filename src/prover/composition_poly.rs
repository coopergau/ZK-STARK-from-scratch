use crate::finite_field::field_params::Fp;
use crate::ff::{PrimeField, Field};
use crate::polynomials::poly::Polynomial;

pub fn calculate_composition_poly(c_1: &Polynomial<Fp>, c_2: &Polynomial<Fp>, c_3: &Polynomial<Fp>, mimc_input: &Fp, mimc_output: &Fp, g: &Fp) {
    calculate_basic_composition_poly(c_1, &Fp::ONE); // constant is g^0

    

    let last_subgroup_element = g.pow(&[127 as u64]);
    calculate_basic_composition_poly(c_3, &last_subgroup_element); // constant is g^127
}

// Creates a basic quotient polynomial by dividing a given polynomial by one root (x - constant) for a given constant.
// Used to create p_1(x) = c_1(x) / (x - I) and p_3(x) = c_3(x) / (x - O), for the mimc input I and output O.
pub fn calculate_basic_composition_poly(constraint_poly: &Polynomial<Fp>, constant: &Fp) -> Polynomial<Fp> {
    let root_vec = vec![Fp::ZERO - constant, Fp::ONE];
    let root_poly = Polynomial::new(&root_vec);
    let (composition_poly, remainder) = constraint_poly.div(&root_poly);
    
    // If r != 0, error
    composition_poly
} 