use crate::finite_field::field_params::Fp;
use crate::ff::PrimeField;
use crate::ff::Field;

/* The Fast Fourier Transform (FFT) and inverse-FFT can be used to efficiently evaluate and interpolate polynomials over the domain of a cyclic subgroup.
The FFT is used for evaluation and the IFFT is used to interpolation. */

// FFT
pub fn evaluate_poly(coeffs: Vec<Fp>, omega: Fp) -> Vec<Fp> {
    let length = coeffs.len();
    if length == 1 {
        return coeffs;
    }

    let even_poly = coeffs.iter().enumerate()
    .filter(|(i, _)| i % 2 == 0).map(|(_, v)| v.clone())
    .collect();
    let odd_poly = coeffs.iter().enumerate()
    .filter(|(i, _)| i % 2 != 0).map(|(_, v)| v.clone())
    .collect();
    
    let new_omega = omega.pow(&[2]);
    let y_even = evaluate_poly(even_poly, new_omega);
    let y_odd = evaluate_poly(odd_poly, new_omega);

    let mut y: Vec<Fp> = vec![Field::ZERO; length];
    let mut current_omega = Fp::from_u128(1);
    for i in 0..length/2 {
        let odd_term = current_omega * y_odd[i];
        y[i] = y_even[i] + odd_term;
        y[i + length/2] = y_even[i] - odd_term;
        current_omega = current_omega * omega;
    }
    return y;
}

// IFFT
pub fn interpolate_poly(coeffs: Vec<Fp>, inverse_omega: Fp) -> Vec<Fp> {
    let length = coeffs.len();
    if length == 1 {
        return coeffs;
    }

    let even_poly = coeffs.iter().enumerate()
    .filter(|(i, _)| i % 2 == 0).map(|(_, v)| v.clone())
    .collect();
    let odd_poly = coeffs.iter().enumerate()
    .filter(|(i, _)| i % 2 != 0).map(|(_, v)| v.clone())
    .collect();
    
    let new_omega = inverse_omega.pow(&[2]);
    let y_even = evaluate_poly(even_poly, new_omega);
    let y_odd = evaluate_poly(odd_poly, new_omega);

    let mut y: Vec<Fp> = vec![Field::ZERO; length];
    let mut current_omega = Fp::from_u128(1);
    for i in 0..length/2 {
        let odd_term = current_omega * y_odd[i];
        y[i] = y_even[i] + odd_term;
        y[i + length/2] = y_even[i] - odd_term;
        current_omega = current_omega * inverse_omega;
    }

    // Normalize the vector
    let inverse_length = Fp::from_u128(length as u128).invert().unwrap();
    y.iter_mut().for_each(|x| *x = *x * inverse_length);

    return y;
}
