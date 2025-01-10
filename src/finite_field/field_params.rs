use ff::{PrimeField, Field};

#[derive(PrimeField)]
// (64 + 2^240)(128) + 1
#[PrimeFieldModulus = "226156424291633194186662080095093570025917938800079226639565593765455339521"]
#[PrimeFieldGenerator = "7"]
#[PrimeFieldReprEndianness = "little"]
pub struct Fp([u64; 4]);

