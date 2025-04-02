use crate::finite_field::field_params::{Fp, FpRepr};
use crate::ff::{PrimeField, Field};
use crate::utils::{generators, fft};
use crate::polynomials::poly::Polynomial;
use sha2::{Sha256, Digest};
use num_primes::BigUint;
use std::default::Default;

#[derive(Default)]
pub struct MerkleTree {
    levels: Vec<Vec<Fp>>
}

impl MerkleTree {
    pub fn default() -> Self {
        MerkleTree { levels: vec![] }
    }

    // Used for commiting to a polynomial over a specific domain.
    // If we just had to compute the merkle root then we could use recursion, but we need 
    // to save every hash in order to create the merkle proof later so we will use iteration.
    pub fn merkle_commit(polynomial: &Polynomial<Fp>, omega: Fp, domain_size: usize) -> Self {
        if domain_size % 2 == 1 {
            panic!("Domain for merkle tree must be even")
        }

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
        
        // Convert poly into evaluations over the specified domain and hash each eval
        let poly_evals = fft::evaluate_poly(&coeffs, omega);
        let mut hashed_poly_evals: Vec<Fp> = Vec::new();
        for eval in poly_evals.iter() {
            let output = Self::hash(&[*eval], &modulus);
            hashed_poly_evals.push(output);
        }

        // Create the merkle tree
        let mut level = hashed_poly_evals;
        let mut levels = vec![level.clone()];
        while level.len() > 1 {
            let next_level: Vec<Fp> = level.chunks(2).map(|chunk| Self::hash(&chunk, &modulus)).collect();
            levels.push(next_level.clone());
            level = next_level;
        }
        
        Self { levels }
    }
    

    // Sha256 hash. Input is an Fp. Output is a bigint converted into an Fp.
    pub fn hash(input: &[Fp], modulus: &BigUint) -> Fp {
        let mut hasher = Sha256::new();
        for element in input {
            hasher.update(element.to_repr());
        }
        let hash_output = hasher.finalize();
        let big_int_output = BigUint::from_bytes_le(&hash_output);
        let reduced_output = big_int_output % modulus;
        let mut output_bytes = reduced_output.to_bytes_le();
        while output_bytes.len() < 32 {
            output_bytes.push(0);
        } 
        let bytes_array: [u8; 32] = output_bytes.try_into().expect("Failed to convert hash output vec into bytes array");
        let output = Fp::from_repr(FpRepr(bytes_array)).unwrap();
        
        output
    }
}