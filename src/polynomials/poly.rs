use crate::finite_field::field_params::Fp;
use crate::ff::{PrimeField, Field};
use crate::utils::{generators, fft};

#[derive(Debug, Clone)]
pub struct Polynomial<F: PrimeField> {
    coefficients: Vec<F>,
}

impl<F: PrimeField> Polynomial<F> {
    
    pub fn new(coefficients: Vec<F>) -> Polynomial<F> {
        Self { coefficients }
    }

    pub fn len(&self) -> usize {
        self.coefficients.len()
    }

    pub fn evaluate(&self, x: F) -> F {
        // Horner's method
        let mut result = F::ZERO;
        for &coeff in self.coefficients.iter().rev() {
            result = result * x + coeff;
        }
        result
    }

    pub fn add(&self, other_poly: Polynomial<F>) -> Polynomial<F> {
        let max_length = std::cmp::max(self.len(), other_poly.len());
        let mut new_coeffs: Vec<F> = Vec::new();
        for i in 0..max_length {
            let coeff1 = if i < self.len() {self.coefficients[i]} else { F::ZERO };
            let coeff2 = if i < other_poly.len() {other_poly.coefficients[i]} else { F::ZERO };
            new_coeffs.push(coeff1 + coeff2);
        }
        Polynomial::new(new_coeffs)
    }

    pub fn sub(&self, other_poly: Polynomial<F>) -> Polynomial<F> {
        let max_length = std::cmp::max(self.len(), other_poly.len());
        let mut new_coeffs: Vec<F> = Vec::new();
        for i in 0..max_length {
            let coeff1 = if i < self.len() {self.coefficients[i]} else { F::ZERO };
            let coeff2 = if i < other_poly.len() {other_poly.coefficients[i]} else { F::ZERO };
            new_coeffs.push(coeff1 - coeff2);
        }
        Polynomial::new(new_coeffs)
    }

    /* Polynomial multiplication using the FFT.
       1. Converts both polynomials into evaluation form.
       2. Multiplies the evaluations that are from the same x value to get one evaluation vector.
       3. Converts that vector back into coefficient form. */
    pub fn mul(&self, other_poly: Polynomial<F>) -> Polynomial<F> {
        /* Multiplying two polynomials a(x) and b(x) will produce a new polynomial c(x) where deg(c) = deg(a) * deg(b).
           So we need to find the smallest power of 2 that is greater than or equal to deg(self) * deg(other_poly), call it n. */
        let new_degree = self.len() * other_poly.len();
        let length = new_degree.next_power_of_two();
        let length_fp = F::from(length as u64);

        // Pad the coefficient vectors to be the size of n.
        let mut coeffs1 = self.coefficients.clone();
        let mut coeffs2 = other_poly.coefficients.clone();
        coeffs1.resize(length, F::ZERO);
        coeffs2.resize(length, F::ZERO);

        // The FFT and IFFT require the nth primitive root of unity.
        let root_of_untiy = generators::get_generator(length_fp);
        let inverse_root_of_unity = root_of_untiy.invert().unwrap();

        // Convert each polynomial into evaluation form.
        let evals1 = fft::evaluate_poly(coeffs1, root_of_untiy);
        let evals2 = fft::evaluate_poly(coeffs2, root_of_untiy);

        // Multiply the evaluations.
        let mut product: Vec<F> = Vec::new();
        for i in 0..length {
            product.push(evals1[i] * evals2[i]);
        }

        // Convert back into coefficient form and remove any trailing zeros.
        let mut product_poly = fft::interpolate_poly(product, inverse_root_of_unity);
        if let Some(index) = product_poly.iter().rposition(|&x| x != F::ZERO) {
            product_poly.truncate(index + 1);
        } else {
            product_poly = vec![F::ZERO];
        }

        Polynomial::new(product_poly)
    }
}
