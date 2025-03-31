use crate::finite_field::field_params::{Fp, FpRepr};
use crate::ff::{PrimeField, Field};
use crate::utils::{generators, fft};
use crate::polynomials::poly::Polynomial;
use sha2::{Sha256, Digest};
use num_primes::BigUint;


// Used for commiting to a polynomial over a specific domain.
pub fn merkle_commit(polynomial: &Polynomial<Fp>, omega: Fp, domain_size: usize) {
    // Pad the coeffs with zeros so the vec is the same length as the domain
    let mut coeffs = polynomial.coefficients.clone();
    if domain_size > coeffs.len() {
        coeffs.resize(domain_size, Fp::ZERO);
    } else if coeffs.len() > domain_size {
        panic!("Polynomial degree should not be larger than commitment domain");
    }

    // Get field modulus for reducing hash outputs
    let modulus_str = &Fp::MODULUS[2..];
    let modulus = BigUint::parse_bytes(modulus_str.as_bytes(), 16).expect("Failed to parse modulus");
    
    let poly_evals = fft::evaluate_poly(&coeffs, omega);
    let mut hashed_poly_evals: Vec<Fp> = Vec::new();
    for eval in poly_evals.iter() {
        let mut hasher = Sha256::new();
        hasher.update(&eval.to_repr());
        let hash_output = hasher.finalize();
        let big_int_output = BigUint::from_bytes_le(&hash_output);
        let reduced_output = big_int_output % &modulus;
        let mut output_bytes = reduced_output.to_bytes_le();
        while output_bytes.len() < 32 {
            output_bytes.push(0);
        } 
        let bytes_array: [u8; 32] = output_bytes.try_into().expect("Failed to convert hash output vec into bytes array");
        let output = Fp::from_repr(FpRepr(bytes_array)).unwrap();
        
        println!("{:?}", output);
    } 
}