use ff::PrimeField;

#[derive(PrimeField)]
// 2^(119) * 407 + 1
// 
#[PrimeFieldModulus = "270497897142230380135924736767050121217"]
#[PrimeFieldGenerator = "85408008396924667383611388730472331217"]
#[PrimeFieldReprEndianness = "little"]
//#[derive(Debug)]
pub struct Fp([u64; 3]);

