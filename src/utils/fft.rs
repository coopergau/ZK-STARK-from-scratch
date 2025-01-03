use crate::finite_field::field_params::Fp;
use crate::ff::PrimeField;
use crate::ff::Field;

/* The Fast Fourier Transform (FFT) and inverse-FFT can be used to efficiently evaluate and interpolate polynomials over the domain of a cyclic subgroup.
The FFT is used for evaluation and the IFFT is used to interpolation. Functions are generic to support testing over different prime fields. */

// FFT
pub fn evaluate_poly<F: PrimeField>(coeffs: Vec<F>, omega: F) -> Vec<F> {
    let length = coeffs.len();
    if length == 1 {
        return coeffs;
    }

    // Split the polynomial into even and odd parts 
    let even_poly = coeffs.iter().enumerate()
    .filter(|(i, _)| i % 2 == 0).map(|(_, v)| v.clone())
    .collect();
    let odd_poly = coeffs.iter().enumerate()
    .filter(|(i, _)| i % 2 != 0).map(|(_, v)| v.clone())
    .collect();
    
    // Recursive calls over the new domain of half the length of the current domain
    let new_omega = omega.pow(&[2]);
    let y_even = evaluate_poly(even_poly, new_omega);
    let y_odd = evaluate_poly(odd_poly, new_omega);

    // Combine even and odd terms using the FFT butterfly operation.
    let mut y: Vec<F> = vec![Field::ZERO; length];
    let mut current_omega = F::from_u128(1);
    for i in 0..length/2 {
        let odd_term = current_omega * y_odd[i];
        y[i] = y_even[i] + odd_term;
        y[i + length/2] = y_even[i] - odd_term;
        current_omega = current_omega * omega;
    }
    return y;
}

// IFFT
pub fn interpolate_poly<F: PrimeField>(evals: Vec<F>, inverse_omega: F) -> Vec<F> {
    let length = evals.len();
    if length == 1 {
        return evals;
    }

    // Split the polynomial into even and odd parts 
    let even_poly = evals.iter().enumerate()
    .filter(|(i, _)| i % 2 == 0).map(|(_, v)| v.clone())
    .collect();
    let odd_poly = evals.iter().enumerate()
    .filter(|(i, _)| i % 2 != 0).map(|(_, v)| v.clone())
    .collect();
    
    // Recursive calls over the new domain of half the length of the current domain.
    /* We call evaluate_poly() because interpolate_poly() normalizes the vector which we only want to do at the end of the recursion.
       Even though inerpolate_poly() uses the inverse of omega, it takes that as a function argument and uses it the same way evaluate_poly() does,
       so the two funcitons are essentially the same but interpolate_poly has normalization. */
    let new_omega = inverse_omega.pow(&[2]);
    let y_even = evaluate_poly(even_poly, new_omega);
    let y_odd = evaluate_poly(odd_poly, new_omega);

    // Combine even and odd terms using the FFT butterfly operation.
    let mut y: Vec<F> = vec![Field::ZERO; length];
    let mut current_omega = F::from_u128(1);
    for i in 0..length/2 {
        let odd_term = current_omega * y_odd[i];
        y[i] = y_even[i] + odd_term;
        y[i + length/2] = y_even[i] - odd_term;
        current_omega = current_omega * inverse_omega;
    }

    // Normalize the vector
    let inverse_length = F::from_u128(length as u128).invert().unwrap();
    y.iter_mut().for_each(|x| *x = *x * inverse_length);

    return y;
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    /* A smaller field will be used for testing to ensure the functions produce expected polynomials. 
    The field modulus is 17, generator is 7. This allows for the use of the cyclic subgroup {1, 4, 13, 16} with generator 4. */

    #[derive(PrimeField)]
    #[PrimeFieldModulus = "17"]
    #[PrimeFieldGenerator = "7"]
    #[PrimeFieldReprEndianness = "little"]
    pub struct Fp([u64; 1]);
   
    pub static G: Lazy<Fp> = Lazy::new(|| Fp::from_u128(4));
    pub static G_INVERSE: Lazy<Fp> = Lazy::new(|| Fp::invert(&G).unwrap());

    #[test]
    // The polynomial 5 + x + 13x^2 + 16x^3 has evaluations f(1)=1, f(4)=0, f(13)=1, and f(16)=1.
    fn test_evaluate_poly_basic() {
        let poly_coeffs: Vec<Fp> = vec![Fp::from_u128(5), Fp::from_u128(1), Fp::from_u128(13), Fp::from_u128(16)];
        let expected_evaluations: Vec<Fp> = vec![Fp::from_u128(1), Fp::from_u128(0), Fp::from_u128(1), Fp::from_u128(1)];
        let actual_evaluations = evaluate_poly(poly_coeffs, *G);
        assert_eq!(actual_evaluations, expected_evaluations);
    }
    
    #[test]
    fn test_interpolate_poly_basic() {
        // The evaluations f(1)=1, f(4)=0, f(13)=1, and f(16)=1 should return the coefficients of 5 + x + 13x^2 + 16x^3. 
        let poly_evaluations: Vec<Fp> = vec![Fp::from_u128(1), Fp::from_u128(0), Fp::from_u128(1), Fp::from_u128(1)];
        let expected_coeffs: Vec<Fp> = vec![Fp::from_u128(5), Fp::from_u128(1), Fp::from_u128(13), Fp::from_u128(16)];
        let actual_coeffs = interpolate_poly(poly_evaluations, *G_INVERSE);
        assert_eq!(actual_coeffs, expected_coeffs);
    }
}