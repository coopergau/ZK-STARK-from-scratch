use crate::finite_field::field_params::Fp;
use crate::ff::{PrimeField, Field};
use crate::utils::{generators, fft};

// Divide function for PrimeFields, used in polynomial division.
pub fn field_divide<F: PrimeField>(dividend: &F, divisor: &F) -> F {
    let inverse = F::invert(&divisor).unwrap();
    *dividend * inverse
}

#[derive(Debug, Clone)]
pub struct Polynomial<F> {
    pub coefficients: Vec<F>,
}

impl<F: PrimeField> Polynomial<F> {
    
    pub fn new(coefficients: &Vec<F>) -> Polynomial<F> {
        if *coefficients.last().unwrap() == F::ZERO {
            let mut new_coeffs = coefficients.clone();
            while let Some(&last) = new_coeffs.last() {
                if last == F::ZERO && new_coeffs.len() > 1 {
                    new_coeffs.pop();
                } else {
                    break;
                }
            }
            return Self { coefficients: new_coeffs };
        }
        return Self { coefficients: coefficients.to_vec() };
    }

    pub fn len(&self) -> usize {
        self.coefficients.len()
    }

    pub fn evaluate(&self, x: &F) -> F {
        // Horner's method
        let mut result = F::ZERO;
        for &coeff in self.coefficients.iter().rev() {
            result = result * x + coeff;
        }
        result
    }

    pub fn add(&self, other_poly: &Polynomial<F>) -> Polynomial<F> {
        let max_length = std::cmp::max(self.len(), other_poly.len());
        let mut new_coeffs: Vec<F> = Vec::new();
        for i in 0..max_length {
            let coeff1 = if i < self.len() {self.coefficients[i]} else { F::ZERO };
            let coeff2 = if i < other_poly.len() {other_poly.coefficients[i]} else { F::ZERO };
            new_coeffs.push(coeff1 + coeff2);
        }

        Polynomial::new(&new_coeffs)
    }

    pub fn sub(&self, other_poly: &Polynomial<F>) -> Polynomial<F> {
        let max_length = std::cmp::max(self.len(), other_poly.len());
        let mut new_coeffs: Vec<F> = Vec::new();
        for i in 0..max_length {
            let coeff1 = if i < self.len() {self.coefficients[i]} else { F::ZERO };
            let coeff2 = if i < other_poly.len() {other_poly.coefficients[i]} else { F::ZERO };
            new_coeffs.push(coeff1 - coeff2);
        }
        
        Polynomial::new(&new_coeffs)
    }

    /* Polynomial multiplication using the FFT.
       1. Converts both polynomials into evaluation form.
       2. Multiplies the evaluations that are from the same x value to get one evaluation vector.
       3. Converts that vector back into coefficient form. */
    pub fn mul(&self, other_poly: &Polynomial<F>) -> Polynomial<F> {
        /* Multiplying two polynomials a(x) and b(x) will produce a new polynomial c(x) where deg(c) = deg(a) + deg(b).
           So we need to find the smallest power of 2 that is greater than or equal to deg(self) * deg(other_poly), call it n. */
        let new_length = self.len() + other_poly.len() - 1;
        let length = new_length.next_power_of_two();
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
        let evals1 = fft::evaluate_poly(&coeffs1, root_of_untiy);
        let evals2 = fft::evaluate_poly(&coeffs2, root_of_untiy);

        // Multiply the evaluations.
        let mut product: Vec<F> = Vec::new();
        for i in 0..length {
            product.push(evals1[i] * evals2[i]);
        }

        // Convert back into coefficient form and remove any trailing zeros.
        let mut product_poly = fft::interpolate_poly(&product, inverse_root_of_unity);
        if let Some(index) = product_poly.iter().rposition(|&x| x != F::ZERO) {
            product_poly.truncate(index + 1);
        } else {
            product_poly = vec![F::ZERO];
        }

        Polynomial::new(&product_poly)
    }

    // Function divides self by the divisor and returns the tuple in the form (quotient, remainder)
    // This shit does not work, write your own
    pub fn div(&self, divisor_poly: &Polynomial<F>) -> (Polynomial<F>, Polynomial<F>) {
        let mut dividend_vec = self.coefficients.clone();
        let divisor = &divisor_poly.coefficients;
        let divisor_degree = divisor.len() - 1;
        
        // Can't divide by zero
        if divisor.is_empty() || divisor.iter().all(|&x| x == F::ZERO) {
            panic!("Cannot divide by zero.");
        }

        // Just scale the dividend if the divisor is a constant.
        if divisor.len() == 1 {
            let inverse_divisor = F::invert(&divisor[0]).unwrap();
            let scaled_dividend_vec = dividend_vec.iter().map(|&x| x * inverse_divisor).collect();
            let remainder = vec![F::ZERO];
            return (Polynomial::new(&scaled_dividend_vec), Polynomial::new(&remainder));
        }

        // The loop below calculates quotient terms starting with the highest degree.
        let mut quotient: Vec<F> = Vec::new();

        // While the degree of the dividend is at least the degree of the divisor.
        while dividend_vec.len() - 1 >= divisor_degree {
            // Ensure the loop exits if the dividend becomes empty.
            if dividend_vec.is_empty() {
                break;
            }

            let dividend_degree = dividend_vec.len() - 1;
            
            // Compute the new term in the quotient.
            let new_term = field_divide(&dividend_vec[dividend_degree], &divisor[divisor_degree]);
            quotient.insert(0, new_term);
            
            // Subtract (new_term * divisor) from the dividend.
            let mut subtract_vec: Vec<F> = divisor.iter().map(|&x| x * new_term).collect();
            subtract_vec.splice(0..0, vec![F::ZERO; dividend_degree - divisor_degree]);
            
            dividend_vec = dividend_vec.iter()
            .zip(subtract_vec.iter())
            .map(|(a, b)| *a - b)
            .collect();
            dividend_vec.pop();
        }
                
        // The remaining dividend is the remainder
        return (Polynomial::new(&quotient), Polynomial::new(&dividend_vec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;
    use rand::Rng;

    #[test]
    // Test that creating a new polyomial removes trailing zeros.
    fn test_remove_trailing_zeros() {
        let coeffs = vec![Fp::from(20), Fp::from(9), Fp::from(1), Fp::from(0), Fp::from(0), Fp::from(0)];
        let poly = Polynomial::new(&coeffs);
        let expected_coeffs =  vec![Fp::from(20), Fp::from(9), Fp::from(1)];

        assert_eq!(expected_coeffs, poly.coefficients);
    }

    #[test]
    // Test the polynomial add function works properly.
    fn test_add() {
        for _ in 0..50 {
            let length1 = OsRng.gen_range(1..=100);
            let length2 = OsRng.gen_range(1..=100);
            let mut poly1_vec: Vec<Fp> = (0..length1).map(|_| Fp::random(OsRng)).collect();
            let mut poly2_vec: Vec<Fp> = (0..length2).map(|_| Fp::random(OsRng)).collect();

            let poly1 = Polynomial::new(&poly1_vec);
            let poly2 = Polynomial::new(&poly2_vec);
            let poly_sum = poly1.add(&poly2);

            // Naive adding.
            // Add leading zeros so the vecs are the same length.
            if length1 > length2 {
                poly2_vec.resize(length1, Fp::ZERO);
            } else if length2 > length1 {
                poly1_vec.resize(length2, Fp::ZERO);
            }

            let mut naive_sum_vec: Vec<Fp> = Vec::new();
            for i in 0..poly1_vec.len() {
                naive_sum_vec.push(poly1_vec[i] + poly2_vec[i]);
            }

            assert_eq!(poly_sum.coefficients, naive_sum_vec);
        }
    }
    
    #[test]
    // Test the polynomial subtract function works properly.
    fn test_sub() {
        for _ in 0..50 {
            let length1 = OsRng.gen_range(1..=100);
            let length2 = OsRng.gen_range(1..=100);
            let mut poly1_vec: Vec<Fp> = (0..length1).map(|_| Fp::random(OsRng)).collect();
            let mut poly2_vec: Vec<Fp> = (0..length2).map(|_| Fp::random(OsRng)).collect();

            let poly1 = Polynomial::new(&poly1_vec);
            let poly2 = Polynomial::new(&poly2_vec);
            let poly_diff = poly1.sub(&poly2);

            // Naive subtracting.
            // Add leading zeros so the vecs are the same length.
            if length1 > length2 {
                poly2_vec.resize(length1, Fp::ZERO);
            } else if length2 > length1 {
                poly1_vec.resize(length2, Fp::ZERO);
            }

            let mut naive_diff_vec: Vec<Fp> = Vec::new();
            for i in 0..poly1_vec.len() {
                naive_diff_vec.push(poly1_vec[i] - poly2_vec[i]);
            }

            assert_eq!(poly_diff.coefficients, naive_diff_vec);
        }
    }

    #[test]
    /* Test that the mul function, which uses the FFT, gets the same product as multiplying each coefficient in one polynomial 
       by each coefficient in the other polynomial.
       Test runs for 100 random pairs of polynomials of length 1 to 100. */
    fn test_fft_mul() {
        for _ in 0..100 {
            let length1 = OsRng.gen_range(1..=100);
            let length2 = OsRng.gen_range(1..=100);
            let poly1_vec: Vec<Fp> = (0..length1).map(|_| Fp::random(OsRng)).collect();
            let poly2_vec: Vec<Fp> = (0..length2).map(|_| Fp::random(OsRng)).collect();
            
            // Naive multiplication
            let mut product_vec = vec![Fp::ZERO; length1 + length2 - 1];
            for i in 0..length1 {
                for j in 0..length2 {
                    product_vec[i + j] = product_vec[i + j] + (poly1_vec[i] * poly2_vec[j])
                }
            }
            // Remove ending zeros
            if let Some(index) = product_vec.iter().rposition(|&x| x != Fp::ZERO) {
                product_vec.truncate(index + 1);
            } else {
                product_vec = vec![Fp::ZERO];
            }
            
            // FFT muliplication
            let poly1 = Polynomial::new(&poly1_vec);
            let poly2 = Polynomial::new(&poly2_vec);
            let fft_product = poly1.mul(&poly2);
            
            assert_eq!(fft_product.coefficients, product_vec);
        }
    }

    #[test]
    /* Test takes two random Polynomials, divides one by the other, then multiplies the divisor 
       polynomial by the quotient and adds the remainder. This process should result in the original
       dividend polynomial. */
    fn tests_random_poly_division() {
        for _ in 0..50 {
            let length1 = OsRng.gen_range(1..=100);
            let length2 = OsRng.gen_range(1..=100);

            let mut dividend_vec: Vec<Fp> = Vec::new();
            let mut divisor_vec: Vec<Fp> = Vec::new();

            if length1 > length2 {
                dividend_vec = (0..length1).map(|_| Fp::random(OsRng)).collect();
                divisor_vec = (0..length2).map(|_| Fp::random(OsRng)).collect();
            } else {
                dividend_vec = (0..length2).map(|_| Fp::random(OsRng)).collect();
                divisor_vec = (0..length1).map(|_| Fp::random(OsRng)).collect();
            }

            let dividend = Polynomial::new(&dividend_vec);
            let divisor = Polynomial::new(&divisor_vec);

            let (quotient, remainder) = dividend.div(&divisor);
            
            let product = divisor.mul(&quotient);
            let reconstructed_dividend = product.add(&remainder);
        
            assert_eq!(dividend.coefficients, reconstructed_dividend.coefficients);
        }
    }

    #[test]
    /* Test takes two random Polynomials, multiplies them together, takes the product and divided it by
       one of the polynomials. This process should result in the other random polynomial and a remainder
       of zero. */
    fn tests_poly_division_with_zero_remainder() {
        for _ in 0..50 {
            let length1 = OsRng.gen_range(1..=100);
            let length2 = OsRng.gen_range(1..=100);

            let poly1_vec: Vec<Fp> = (0..length1).map(|_| Fp::random(OsRng)).collect();
            let poly2_vec: Vec<Fp> = (0..length2).map(|_| Fp::random(OsRng)).collect();

            let poly1 = Polynomial::new(&poly1_vec);
            let poly2 = Polynomial::new(&poly2_vec);
    
            let product = poly1.mul(&poly2);
            let (quotient, remainder) = product.div(&poly2);

            assert_eq!(quotient.coefficients, poly1.coefficients);
            assert_eq!(remainder.coefficients, vec![Fp::ZERO]);
           
        }

    }
}
