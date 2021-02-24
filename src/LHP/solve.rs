use crate::structures;
use std::ops::{Add, Mul, Div, Sub};
use num_integer::Integer;

use std::borrow::Cow;

extern crate num_bigint_dig;

use num_bigint::{ToBigUint};



pub fn solve(pp: &structures::Params, Z: &structures::Puzzle) ->num_bigint::BigUint{
    let ONE=1.to_biguint().unwrap();
    let N_suared =(&pp.N).mul(&pp.N);
    let w= num_bigint_dig::algorithms::mod_inverse(
        Cow::Borrowed(&(repeated_squaring(&Z.u, &pp.T, &pp.N)).modpow(&pp.N,&N_suared)),
        Cow::Borrowed(&N_suared)).unwrap().to_biguint().unwrap();
    let s=(((&Z.v).mul(&w).mod_floor(&N_suared)).sub(&ONE)).div(&pp.N);
    return s;
}


fn repeated_squaring(u:&num_bigint::BigUint, t:&num_bigint::BigUint, N:&num_bigint::BigUint) ->num_bigint::BigUint{
    let ONE=1.to_biguint().unwrap();
    let mut x =0.to_biguint().unwrap();
    let mut res = u.clone();
    loop {
        if x.eq(&t) == true {
            break;
        }
        x = x.add(&ONE);
        res = (&res).mul(&res);
    }
    return res.mod_floor(N);
}
