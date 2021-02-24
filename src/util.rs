use num_bigint::*;

pub fn gen_prime(size:u64) -> num_bigint::BigUint{
    let mut rng = rand::thread_rng();
    return rng.gen_biguint(size as usize);
}