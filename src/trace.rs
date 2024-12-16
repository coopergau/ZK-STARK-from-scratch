use crate::finite_field::field_params::Fp;

pub fn mimc(input: Fp, rounds: u32) {
    let mut hash_value = input;
    let mut trace = vec![input];
    for i in 1..=rounds {
       println!("{:?}", hash_value);
    }
    println!("{:?}", trace);
}