use num_primes::BigUint;
use num_primes::Verification;
use num_traits::identities::One;
use num_traits::ops::checked::{CheckedMul, CheckedAdd, CheckedDiv, CheckedSub};
use num_traits::Pow;
use std::ops::Rem;

// The functions in this file were used to find the finite field params but are not needed for the proof protocol so they all have #[allow(dead_code)].

/* This function is used to find the following properties of a finite field for the STARK:
1. Field modulus, p
2. Field generator (needed for ff crate)
3. Generator of cyclic subgroup of order 128 (because the trace has 128 elements) 

We want the subgroup to have order 128. This happens if p - 1 divides 128, where p is the field modulus.
This means we can test for p values using the equation p = (k)128 + 1, for some natural number k.

There is currently a bug with the is_prime function where it sometimes returns the wrong answer, so the function was used to find candidates that were then verified to be prime.
*/
#[allow(dead_code)]
pub fn find_prime_field() {
    let subgroup_order = BigUint::parse_bytes(b"128", 10).unwrap(); // Might want to change to MIMC_ROUNDS
    let two = BigUint::from(2u32);
    let two_hundred_fifty = BigUint::from(240u32);
    let mut k = two.pow(two_hundred_fifty); // Start k at 2^250 
    
    // Run until a k is found so that (k)128 + 1 is prime:
    let mut counter = 0;
    loop {
        let p = subgroup_order.checked_mul(&k).unwrap().checked_add(&BigUint::one()).unwrap();
        let is_prime = Verification::is_prime(&p);
        if is_prime {
            println!("p: {:?}", p.to_str_radix(10));
            println!("Is prime? {:?}", is_prime);
            println!("k: {:?}", counter);
            break;
        }
        k = k.checked_add(&BigUint::one()).unwrap();
        counter += 1;
    }
}

/* find_prime_field returned 226156424291633194186662080095093570025917938800079226639565593765455339521 after 64 iterations so it is equal to (64 + 2^240)(128) + 1

Once the field modulus has been found, we need to find a primitive element (generator) of the field by using the following theorem:
The element a is a primitive element in Z_p if and only if  a^{(p-1)/q} != 1 mod p  for each prime factor q of (p-1). 
So we need to factor the p-1 and test elements of Z_p.

We know that p-1 = (64 + 2^240)(128) = 2^6(1 + 2^234)(2^7). Using an online factoring calculator like wolfram for (1 + 2^234) we get the prime factors of:
2, 5, 13, 37, 53, 109, 157, 313, 1249, 1613, 3121, 7489, 21061, 21841, 348661, 1112388285061, 370244405487013669
*/

/* Function find_primitive_element() finds a primitive element of F_p by checking if each natural number is a primitive element, starting at 2, and stopping when one is found. 
Right now the limit is set to only test up to the number 10, which worked because 3 and 7 are both primitive elements.
7 is a very commonly used generator so that will be used in field_params.rs. */

/* To perform polynomial encoding of the trace, a subgroup of F_p of order equal to the length of the trace (128) is required.
To find a generator of this subgroup we can take our field generator g and compute g' = g^{(p-1)/128} mod p.
g' will have the following properties:
- g'^128 == 1 mod p, by Fermatt's little theorem
- g'^i != 1 mod p, for 1 <= i <= 127
- g'^i != g'^j, for i != j
So essentially, g' will generate a cyclic subgroup of F_p with 128 unique elements.
*/


#[allow(dead_code)]
pub fn find_primitive_element() {
    let p = BigUint::parse_bytes(b"226156424291633194186662080095093570025917938800079226639565593765455339521", 10).unwrap();
    let p_minus_one = p.checked_sub(&&BigUint::one()).unwrap();
    let prime_factors = [BigUint::from(2u64), BigUint::from(5u64), BigUint::from(13u64), BigUint::from(37u64), BigUint::from(53u64), 
                                    BigUint::from(109u64), BigUint::from(157u64), BigUint::from(313u64), BigUint::from(1249u64), 
                                    BigUint::from(1613u64), BigUint::from(3121u64), BigUint::from(7489u64), BigUint::from(21061u64), 
                                    BigUint::from(21841u64), BigUint::from(348661u64), BigUint::from(1112388285061u64), BigUint::from(370244405487013669u64)];

    let mut primitive_element = BigUint::from(2u32);
    let limit = BigUint::from(10u32);
    while primitive_element < limit {
        let mut generator_found = true;
        for factor in prime_factors.iter() {
            let exponent = p_minus_one.checked_div(&factor).unwrap();
            let result = modular_exponentiation(primitive_element.clone(), exponent, &p);
            if result == BigUint::one() {
                generator_found = false;
                break
            }
        }
        println!("{:?}: {:?}", primitive_element, generator_found);
        primitive_element = primitive_element.checked_add(&BigUint::one()).unwrap();
    }
}

// Function performs bitwise modular exponentiation to be able to calculate a^b mod p when b is very large.
// It iterates over the bits that make up the exponent. The result starts a 1 and gets multiplied by the base if the current bit is a 1.
// Every round the base is squared because moveing to the next bit of the exponent is really just multiplying the exponent by two, hence 
// squaring the base.
#[allow(dead_code)]
pub fn modular_exponentiation(mut base: BigUint, mut exponent: BigUint, modulus: &BigUint) -> BigUint {
    let two = BigUint::from(2u32);
    let mut result = BigUint::one();
    base = base.rem(modulus);
    while exponent > BigUint::from(0u32) {
        // If the current bit is 1 multiply the result by the current base
        if exponent.clone().rem(&two) == BigUint::one() {
            result = result.checked_mul(&base).unwrap().rem(modulus);
        }

        // Square the base after every iteration
        base = base.checked_mul(&base).unwrap().rem(modulus);
        exponent = exponent.clone().checked_div(&two).unwrap();
    }
   
    result
}