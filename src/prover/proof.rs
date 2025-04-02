use crate::finite_field::field_params::Fp;
use crate::ff::{PrimeField, Field};
use crate::{MIMC_ROUNDS, G_DOMAIN_SIZE, L_DOMAIN_SIZE};
use crate::trace::create_trace;
use crate::utils::{generators, fft};
use crate::polynomials::poly::Polynomial;
use super::merkle::MerkleTree;
use super::constraint_polys::calculate_constraint_polys;
use super::composition_poly::calculate_composition_poly;
use std::default::Default;


pub struct Proof {
    mimc_rounds: u32,
    initial_domain_size: u64,
    extended_domain_size: u64,
    mimc_output: Fp,
    trace_polynomial: Polynomial<Fp>,
    composition_polynomial: Polynomial<Fp>,
    trace_poly_merkle_tree: MerkleTree,
    composition_poly_merkle_tree: MerkleTree,
}

impl Proof {
    pub fn initialize_proof(mimc_rounds: u32, initial_domain_size: u64, extended_domain_size: u64) -> Self {
        Self {
            mimc_rounds,
            initial_domain_size,
            extended_domain_size,
            mimc_output: Default::default(),
            trace_polynomial: Default::default(),
            composition_polynomial: Default::default(),
            trace_poly_merkle_tree: Default::default(),
            composition_poly_merkle_tree: Default::default()
        }
    }

    pub fn generate_proof(&mut self, mimc_input: Fp) {
        // Compute the trace polynomial.
        let trace = create_trace::mimc_trace(mimc_input, self.mimc_rounds);
        self.mimc_output = trace.last().unwrap().clone();
        self.interpolate_trace(trace);
        
        // Commit to the trace polynomial over the extended domain. 
        self.trace_poly_merkle_tree = self.compute_merkle_tree(&self.trace_polynomial.clone());
        
        // Compute the composition polynomial
        self.compute_composition_polynomial();

        // Commit to the composition polynomial
        self.composition_poly_merkle_tree = self.compute_merkle_tree(&self.composition_polynomial.clone());
    }
    
    pub fn interpolate_trace(&mut self, trace: Vec<Fp>) {
        let g_order = Fp::from(self.initial_domain_size);
        let g_generator = generators::get_generator(g_order);
        let g_generator_inverse = g_generator.invert().unwrap();
        let mut trace_poly_coeffs = fft::interpolate_poly(&trace, g_generator_inverse);
        let trace_poly = Polynomial::new(&trace_poly_coeffs);
        self.trace_polynomial = trace_poly;
    }

    pub fn compute_composition_polynomial(&mut self) {
        // Compute the constraint polynomials c_1 and c_2.
        let g_order = Fp::from(self.initial_domain_size);
        let g_generator = generators::get_generator(g_order);
        let (c_1, c_2) = calculate_constraint_polys(&self.mimc_output, &self.trace_polynomial,&g_generator);

        self.composition_polynomial = calculate_composition_poly(&c_1, &c_2, &g_generator);
    }

    pub fn compute_merkle_tree(&mut self, polynomial: &Polynomial<Fp>) -> MerkleTree {
        let l_order = Fp::from(self.extended_domain_size);
        let l_generator = generators::get_generator(l_order);
        let merkle_tree = MerkleTree::merkle_commit(polynomial, l_generator, self.extended_domain_size as usize);
        merkle_tree
    }
}

//
//// Generate trace.
//let trace = create_trace::mimc_trace(mimc_input, MIMC_ROUNDS);
//
//// Interpolate the polynomial f that maps each element of the domain G to a corresponding trace value.
//let g_order = Fp::from(G_DOMAIN_SIZE);
//let g_generator = generators::get_generator(g_order);
//let g_generator_inverse = g_generator.invert().unwrap();
//let mut f_poly_coeffs = fft::interpolate_poly(&trace, g_generator_inverse);
//let f_poly = Polynomial::new(&f_poly_coeffs);
//
//// Low degree extension (LDE) - Evaluate the polynomial over the larger domain L.
//let l_size = Fp::from(L_DOMAIN_SIZE);
//let l_generator = generators::get_generator(l_size);
//f_poly_coeffs.resize(L_DOMAIN_SIZE as usize, Field::ZERO); // Add padding in order to extend the evaluation domain.
//let f_evals_over_extended_domain = fft::evaluate_poly(&f_poly_coeffs, l_generator);
//
//// Commit to the LDE of the polynomial f.
//let f_poly_merkle_tree = MerkleTree::merkle_commit(&f_poly, l_generator, L_DOMAIN_SIZE as usize);
//
//// Compute the constraint polynomials c_1 and c_2.
//let (c_1, c_2) = calculate_constraint_polys(&mimc_input, &mimc_output, &f_poly, &g_generator);
//
//// Compute the composition polynomial p. 
//let composition_poly = calculate_composition_poly(&c_1, &c_2, &g_generator); // Update whether these should be references or not
//
//// Commit to the polynomial p.
//let composition_poly_merkle_tree = MerkleTree::merkle_commit(&composition_poly, l_generator, L_DOMAIN_SIZE as usize);
//
//// FRI
