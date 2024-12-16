use num_primes::BigUint;
use num_primes::Verification;
use num_traits::identities::One;
use num_traits::ops::checked::{CheckedMul, CheckedAdd, CheckedDiv, CheckedSub};
use num_traits::Pow;
use std::ops::Rem;

/* This function is used to find the following properties of a finite field for the STARK:
1. Field modulus, p
2. Field generator (needed for ff crate)
3. Generator of cyclic subgroup of order 128 (because the trace has 128 elements) 

We want the subgroup to have order 128. This happens if p - 1 divides 128, where p is the field modulus.
This means we can test for p values using the equation p = (k)128 + 1, for some natural number k.

There is currently a bug with the is_prime function where it sometimes returns the wrong answer, so the function was used to find candidates that were then verified to be prime.
*/

const SUBGROUP_ORDER: &str = "128";

pub fn find_prime_field() {
    let subgroup_order = BigUint::parse_bytes(SUBGROUP_ORDER.as_bytes(), 10).unwrap();
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
            println!("{:?}", is_prime);
            println!("{:?}", counter);
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

pub fn find_primitive_element() {
    let p = BigUint::parse_bytes(b"226156424291633194186662080095093570025917938800079226639565593765455339521", 10).unwrap();
    let p_minus_one = p.checked_sub(&&BigUint::one()).unwrap();
    let prime_factors = [BigUint::from(2u64), BigUint::from(5u64), BigUint::from(13u64), BigUint::from(37u64), BigUint::from(53u64), 
                                    BigUint::from(109u64), BigUint::from(157u64), BigUint::from(313u64), BigUint::from(1249u64), 
                                    BigUint::from(1613u64), BigUint::from(3121u64), BigUint::from(7489u64), BigUint::from(21061u64), 
                                    BigUint::from(21841u64), BigUint::from(348661u64), BigUint::from(1112388285061u64), BigUint::from(370244405487013669u64)];

    let mut primitive_element = BigUint::from(2u32);
    loop {
        for factor in prime_factors.iter() {
            let exponent = p_minus_one.checked_div(&factor).unwrap();
            let remainder = primitive_element.pow(exponent).rem(&p);
            println!("{:?}", remainder);
        }
        break;
    }
}

// remainder part overflows the memory so we need a function for modular exponentiation: works through the exponentiation while reducing during the process to avoid memory overflow