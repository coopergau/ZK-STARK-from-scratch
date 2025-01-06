use crate::ff::PrimeField;

#[derive(Debug, Clone)]
pub struct Polynomial<F: PrimeField> {
    coefficients: Vec<F>,
}

impl<F: PrimeField> Polynomial<F> {
    pub fn new(coefficients: Vec<F>) -> Polynomial<F> {
        Self { coefficients }
    }

    pub fn evaluate(&self, x: F) -> F {
        // Horner's method
        let mut result = F::ZERO;
        for &coeff in self.coefficients.iter().rev() {
            result = result * x + coeff;
        }
        result
    }

    //pub fn add(&self, other_poly:) {}
}
