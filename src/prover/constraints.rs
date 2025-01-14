use crate::finite_field::field_params::Fp;
use crate::ff::{PrimeField, Field};
use crate::polynomials::poly::Polynomial;
use crate::trace::create_trace::MIMC_CONSTANT;

use rand::rngs::OsRng;

pub fn calculate_constraint_polys(proof_i: Fp, proof_o: Fp, f_poly: Polynomial<Fp>, subgroup_generator: Fp) {
    let c_1 = calculate_constraint_poly1(proof_i, &f_poly);
    let c_2 = calculate_constraint_poly2(&f_poly, subgroup_generator);
    println!("{:?}", c_2);
}

// c_1(x) = f(x) - I.
pub fn calculate_constraint_poly1(proof_i: Fp, f_poly: &Polynomial<Fp>) -> Polynomial<Fp> {
    let input_vec = vec![proof_i];
    let input_poly = Polynomial::new(&input_vec);
    let c_1 = f_poly.sub(&input_poly);

    c_1
}

// c_2(x) = f(gx) - (f(x) + k)^3.
pub fn calculate_constraint_poly2(f_poly: &Polynomial<Fp>, g: Fp) -> () {
   // Create g(x) = 1 + gx + g^2x^2 + g^3x^3 ...
   let mut g_vec = vec![Fp::ONE]; 
   for i in 1..f_poly.len() {
        g_vec.push(g.pow(&[i as u64]));
   }

   // Compute f(gx)
   let f_vec = &f_poly.coefficients;
   let fg_vec: Vec<Fp> = f_vec.iter().zip(g_vec.iter()).map(|(a, b)| *a * b).collect();
   let fg_poly = Polynomial::new(&fg_vec);

   // Compute (f(x) + k)^3
   let mut k_vec: Vec<Fp> = vec![*MIMC_CONSTANT];
   let k_poly = Polynomial::new(&k_vec);
   
   let f_plus_k_poly = f_poly.add(&k_poly);
   let f_k_cubed_poly = f_plus_k_poly.mul(&f_plus_k_poly).mul(&f_plus_k_poly);

}