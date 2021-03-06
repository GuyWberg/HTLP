extern crate num_bigint_dig as num_bigint;

use std::ops::{Add, Div, Mul, Sub};

use num_bigint::{RandBigInt, ToBigUint};
use num_bigint_dig::{BigUint, ToBigInt};
use num_bigint_dig::algorithms::xgcd;

use crate::structures;
use crate::util;


/**
Generate public parameters: time bound (T), modulus (N), and the puzzle parameters h,g
NOTICE: this parameters are generated privately, therefore rendering this scheme as private coin
**/
pub fn setup(security_param: u64, time: num_bigint::BigUint) -> structures::Params {
    let one = 1.to_biguint().unwrap();
    let two = 2.to_biguint().unwrap();
    let p = util::gen_prime(security_param).mul(&two).add(&one);
    let q = util::gen_prime(security_param).mul(&two).add(&one);
    let n_out = (&p).mul(&q); // N = pq
    let g_tilde = random_invertible(&n_out);
    let g_out = (&n_out).sub(g_tilde.modpow(&two, &n_out)); // g = -g^2 mod N
    let pi_n = ((&p).sub(&one)).mul((&q).sub(&one)); // \varphi(n) = (p-1)(q-1)
    let tuti = (&two).modpow(&time, &pi_n.div(&two)); // 2^T (easy to compute knowing p,q)
    let h_out = g_out.modpow(&tuti, &n_out); // h = g^{2^T}
    return structures::Params { t: time, n: n_out, g: g_out, h: h_out };
}

/**
Chooses random invertible element in the ring Z/nZ
**/
pub fn random_invertible(n: &BigUint) -> BigUint {
    let one = 1.to_biguint().unwrap();
    let mut rng = rand::thread_rng();
    let g_tilde = rng.gen_biguint_range(&one, &(&n).sub(&one));
    return if xgcd(&g_tilde.to_bigint().unwrap(), &n.to_bigint().unwrap(), false).0 == 1.to_bigint().unwrap() {
        g_tilde
    } else {
        random_invertible(n)
    };
}
