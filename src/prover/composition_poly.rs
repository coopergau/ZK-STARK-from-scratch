use crate::finite_field::field_params::Fp;
use crate::ff::{PrimeField, Field};
use crate::polynomials::poly::Polynomial;

use rand::rngs::OsRng; // remove this once root hash is used for randomness


pub fn calculate_composition_poly(c_1: &Polynomial<Fp>, c_2: &Polynomial<Fp>, g: &Fp) -> Polynomial<Fp> {
    let p_1 = calculate_p_1(c_1, g);
    let last_subgroup_element = g.pow(&[127 as u64]);
    let x_degree = 1;
    let p_2 = calculate_basic_composition_poly(c_2, &last_subgroup_element, x_degree); // divisor is x - g^127

    // Use the root hash of the commitment to f to get three random nums.
    let a_as_vec = vec![Fp::random(OsRng)];
    let b_as_vec = vec![Fp::random(OsRng)];

    let a_poly = Polynomial::new(&a_as_vec);
    let b_poly = Polynomial::new(&b_as_vec);

    let scaled_p_1 = p_1.mul(&a_poly);
    let scaled_p_2 = p_2.mul(&b_poly);

    let composition_poly = scaled_p_1.add(&scaled_p_2);

    composition_poly
}

// Create the p_1 poly: p_1(x) = c_1(x) / [(x^128 - 1) / (x - g^127)]
pub fn calculate_p_1(constraint_poly: &Polynomial<Fp>, g: &Fp) -> Polynomial<Fp> {
    // (x^128 - 1)
    let mut root_vec = vec![Fp::ZERO - Fp::ONE];
    root_vec.resize(128, Fp::ZERO);
    root_vec.push(Fp::ONE);
    let mut root_poly = Polynomial::new(&root_vec);
    
    //  (x - g^127)
    let mut g_127_root_vec = vec![Fp::ZERO - Fp::ONE];
    g_127_root_vec.resize(1, Fp::ZERO);
    g_127_root_vec.push(Fp::ONE);
    let g_127_root_poly = Polynomial::new(&g_127_root_vec);
    
    let (root_poly, r2) = root_poly.div(&g_127_root_poly);
    let (composition_poly, remainder) = constraint_poly.div(&root_poly);
    
    // If r != 0, error - maybe just allow this so we can create false proofs that will get rejected by the verifier.
    composition_poly
} 

// Creates a basic quotient polynomial by dividing a given polynomial by one root (x^x_degree - constant) for a given constant.
// For p_2(x) = c_2(x) / (x - g^127), the constants is g^127 and the x_degree is 1.
pub fn calculate_basic_composition_poly(constraint_poly: &Polynomial<Fp>, constant: &Fp, x_degree: usize) -> Polynomial<Fp> {
    let mut root_vec = vec![Fp::ZERO - constant];
    if x_degree != 1 {
        root_vec.resize(x_degree, Fp::ZERO);
    }
    root_vec.push(Fp::ONE);
    let root_poly = Polynomial::new(&root_vec);
    let (composition_poly, remainder) = constraint_poly.div(&root_poly);

    // If r != 0, error - maybe just allow this so we can create false proofs that will get rejected by the verifier.
    composition_poly
} 

