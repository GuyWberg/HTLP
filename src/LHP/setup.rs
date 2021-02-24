use crate::structures;
use crate::util;
extern crate num_bigint_dig as num_bigint;

use std::ops::{Mul, Add, Sub, Div};
use num_bigint::{RandBigInt, ToBigUint};
use num_bigint_dig::{BigUint, ToBigInt};
use num_bigint_dig::algorithms::xgcd;

pub fn gen(security_param:u64,time: num_bigint::BigUint) ->structures::Params{
    let ONE=1.to_biguint().unwrap();
    let TWO=2.to_biguint().unwrap();
    let p=util::gen_prime(security_param).mul(&TWO).add(&ONE);
    let q=util::gen_prime(security_param).mul(&TWO).add(&ONE);
    let N_out =(&p).mul(&q);
    let g_tilde =random_invertible(&N_out);
    let g_out = (&N_out).sub(g_tilde.modpow(&TWO, &N_out));
    let pi_n=((&p).sub(&ONE)).mul((&q).sub(&ONE));
    let tuti=(&TWO).modpow(&time,&pi_n.div(&TWO));
    let h_out=g_out.modpow(&tuti,&N_out);
    return structures::Params{T:time,N: N_out,g:g_out,h:h_out};
}

pub fn random_invertible(n: &BigUint)->BigUint{
    let ONE=1.to_biguint().unwrap();
    let mut rng = rand::thread_rng();
    let g_tilde = rng.gen_biguint_range(&ONE, &(&n).sub(&ONE));
    return if xgcd(&g_tilde.to_bigint().unwrap(), &n.to_bigint().unwrap(), false).0 == 1.to_bigint().unwrap() {
        g_tilde
    } else {
        random_invertible(n)
    }
}
