use crate::structures;
use std::ops::{Sub, Mul, Add};
use num_integer::Integer;
use num_bigint_dig::{ToBigUint, RandBigInt};

extern crate num_bigint_dig as num_bigint;

/**
Generate puzzle given the public parameters with some secret.
**/
pub fn gen(pp: &structures::Params ,s: num_bigint::BigUint) ->structures::Puzzle{
    let one =1.to_biguint().unwrap();
    let n_squared =(&pp.n).mul(&pp.n); // N^2
    let mut rng = rand::thread_rng();
    let r = rng.gen_biguint_range(&one, &(&n_squared).sub(&one));
    let u_out =(&pp.g).modpow(&r, &pp.n); // g^r mod N
    let v_out =
        (((&pp.h).modpow(&((&r).mul(&pp.n)),&n_squared))
            .mul(((&pp.n).add(&one)).modpow(&s, &n_squared)))
        .mod_floor(&n_squared); // h^{rN}(1+N)^s mod N^2
    return structures::Puzzle{u: u_out,v: v_out };
}
