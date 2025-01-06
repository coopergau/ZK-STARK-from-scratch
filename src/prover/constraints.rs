use crate::finite_field::field_params::Fp;
use crate::ff::PrimeField;

pub fn calculate_constraint_polys(proof_i: Fp, proof_o: Fp) {
    let c_1 = calculate_constraint_poly1(proof_i);
}

pub fn calculate_constraint_poly1(proof_i: Fp) {
    let a = Fp::from(3);
    let b = Fp::from(5);
    let c = Fp::from(7);

    // Define a polynomial: P(x) = 3 + 5x + 7x^2
    let coeffs = vec![a, b, c];
//    println!("{:?}", poly);
}