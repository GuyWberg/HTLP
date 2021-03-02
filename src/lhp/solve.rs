extern crate num_bigint_dig;

use std::borrow::Cow;
use std::ops::{Add, Div, Mul, Sub};

use num_bigint::ToBigUint;
use num_integer::Integer;

use crate::structures;

/**
Solves puzzle (using repeated squaring).
**/
pub fn solve(pp: &structures::Params, z: &structures::Puzzle) -> num_bigint::BigUint {
    let one = 1.to_biguint().unwrap();
    let n_squared = (&pp.n).mul(&pp.n);
    let w = num_bigint_dig::algorithms::mod_inverse(
        Cow::Borrowed(&(repeated_squaring(&z.u, &pp.t, &pp.n)).modpow(&pp.n, &n_squared)),
        Cow::Borrowed(&n_squared)).unwrap().to_biguint().unwrap(); // (w)^{-N}=u^{-N*2^T} mod N
    let s = (((&z.v).mul(&w).mod_floor(&n_squared)).sub(&one)).div(&pp.n); // (u*(w)^{-N}-1)/N
    return s;
}


fn repeated_squaring(u:&num_bigint::BigUint, t:&num_bigint::BigUint, n:&num_bigint::BigUint) ->num_bigint::BigUint{
    let one = 1.to_biguint().unwrap();
    let mut x = 0.to_biguint().unwrap();
    let mut res = u.clone();
    loop {
        if x.eq(&t) == true {
            break;
        }
        x = x.add(&one);
        res = (&res).mul(&res);
    }
    return res.mod_floor(n); // u^{2^T} mod N
}
