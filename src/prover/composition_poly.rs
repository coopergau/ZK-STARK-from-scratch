use crate::finite_field::field_params::Fp;
use crate::ff::{PrimeField, Field};
use crate::polynomials::poly::Polynomial;

use rand::rngs::OsRng; // remove this once root hash is used for randomness


pub fn calculate_composition_poly(c_1: &Polynomial<Fp>, c_2: &Polynomial<Fp>, c_3: &Polynomial<Fp>, g: &Fp) -> Polynomial<Fp> {
    let p_1 = calculate_basic_composition_poly(c_1, &Fp::ONE, 1); // divisor is x - g^0
    let p_2 = calculate_basic_composition_poly(c_2, &Fp::ONE, 128); // divisor is x^128 - 1
    let last_subgroup_element = g.pow(&[127 as u64]);
    let p_3 = calculate_basic_composition_poly(c_3, &last_subgroup_element, 1); // divisor is x - g^127

    // Use the root hash of the commitment to f to get three random nums.
    let a_as_vec = vec![Fp::random(OsRng)];
    let b_as_vec = vec![Fp::random(OsRng)];
    let c_as_vec = vec![Fp::random(OsRng)];

    let a_poly = Polynomial::new(&a_as_vec);
    let b_poly = Polynomial::new(&b_as_vec);
    let c_poly = Polynomial::new(&c_as_vec);

    let scaled_p_1 = p_1.mul(&a_poly);
    let scaled_p_2 = p_2.mul(&b_poly);
    let scaled_p_3 = p_3.mul(&c_poly);

    let composition_poly = scaled_p_1.add(&scaled_p_2).add(&scaled_p_3);

    composition_poly
}

// Creates a basic quotient polynomial by dividing a given polynomial by one root (x^x_degree - constant) for a given constant.
// For p_1(x) = c_1(x) / (x - g^0) and p_3(x) = c_3(x) / (x - g^127), the constants are g^0 and g^127 and the x_degree is 1.
// For p_2(x) = c_2(x) / x^128 - 1, the constant is 1 and the x_degree is 128.
pub fn calculate_basic_composition_poly(constraint_poly: &Polynomial<Fp>, constant: &Fp, x_degree: usize) -> Polynomial<Fp> {
    let mut root_vec = vec![Fp::ZERO - constant];
    if x_degree != 1 {
        root_vec.resize(x_degree, Fp::ZERO);
    }
    root_vec.push(Fp::ONE);
    let root_poly = Polynomial::new(&root_vec);
    let (composition_poly, remainder) = constraint_poly.div(&root_poly);
    
    // If r != 0, error - maybe jsut allow this so we can create false proofs that will get rejected by the verifier.
    composition_poly
} 
