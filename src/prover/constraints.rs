use crate::finite_field::field_params::Fp;
use crate::ff::{PrimeField, Field};
use crate::polynomials::poly::Polynomial;
use crate::trace::create_trace::MIMC_CONSTANT;

pub fn calculate_constraint_polys(
    proof_i: &Fp, 
    proof_o: &Fp, 
    f_poly: Polynomial<Fp>, 
    subgroup_generator: &Fp
) -> (Polynomial<Fp>, Polynomial<Fp>, Polynomial<Fp>) {
    let c_1 = calculate_basic_constraint_poly(proof_i, &f_poly);
    let c_2 = calculate_constraint_poly2(&f_poly, &subgroup_generator);
    let c_3 = calculate_basic_constraint_poly(proof_o, &f_poly);
    
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
pub fn calculate_constraint_poly2(f_poly: &Polynomial<Fp>, g: &Fp) -> Polynomial<Fp> {
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

   // Compute f(gx) + (f(x) + k)^3
   let c_2 = fg_poly.add(&f_k_cubed_poly);

    c_2
}

