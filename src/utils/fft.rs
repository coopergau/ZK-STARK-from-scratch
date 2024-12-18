use crate::finite_field::field_params::Fp;
use crate::ff::PrimeField;

/* The Fast Fourier Transform (FFT) and inverse-FFT can be used to efficiently evaluate and interpolate polynomials over the domain of a cyclic subgroup.
The FFT is used for evaluation and the IFFT is used to interpolation. */

// FFT
pub fn evaluate_poly(coeffs: Vec<i32>, omega: i32) -> Vec<i32> {
    let modulo: i32 = 17;
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
    
    let new_omega = omega.pow(2) % modulo;
    let y_even = evaluate_poly(even_poly, new_omega);
    let y_odd = evaluate_poly(odd_poly, new_omega);

    let mut y: Vec<i32> = vec![0; length];
    let mut current_omega = 1;
    for i in 0..length/2 {
        let odd_term = (current_omega * y_odd[i]) % modulo;
        y[i] = (y_even[i] + odd_term) % modulo;
        y[i + length/2] = (y_even[i] - odd_term) % modulo;
        current_omega = (current_omega * omega) % modulo;
    }
    return y;
}

// IFFT // Does not work FFt works
pub fn interpolate_poly(evaluations: Vec<i32>, omega_inverse: i32) -> Vec<i32> {
    let modulo: i32 = 17;
    let length = evaluations.len();
    if length == 1 {
        return evaluations;
    }

    let even_poly = evaluations.iter().enumerate()
    .filter(|(i, _)| i % 2 == 0).map(|(_, v)| v.clone())
    .collect();
    let odd_poly = evaluations.iter().enumerate()
    .filter(|(i, _)| i % 2 != 0).map(|(_, v)| v.clone())
    .collect();
    
    let new_omega_inverse = omega_inverse.pow(2) % modulo;
    let y_even: Vec<i32> = interpolate_poly(even_poly, new_omega_inverse);
    let y_odd: Vec<i32> = interpolate_poly(odd_poly, new_omega_inverse);

    let mut y: Vec<i32> = vec![0; length];
    let mut current_omega = 1;
    for i in 0..length/2 {
        let odd_term = (current_omega * y_odd[i]) % modulo;
        y[i] = (y_even[i] + odd_term) % modulo;
        y[i + length/2] = (y_even[i] - odd_term) % modulo;
        current_omega = (current_omega * omega_inverse) % modulo;
    }

    // Normalize the vector
    let length_inv = mod_inverse(length as i32, modulo);
    y.iter_mut().for_each(|x| *x = (*x * length_inv) % modulo);

    return y;
}

pub fn mod_inverse(a: i32, modulo: i32) -> i32 {
    let mut m0 = modulo;
    let mut x0 = 0;
    let mut x1 = 1;
    let mut a = a % modulo;

    if modulo == 1 {
        return 0;
    }

    while a > 1 {
        let q = a / m0;
        let t = m0;

        m0 = a % m0;
        a = t;
        let t = x0;

        x0 = x1 - q * x0;
        x1 = t;
    }

    if x1 < 0 {
        x1 += modulo;
    }

    x1
}