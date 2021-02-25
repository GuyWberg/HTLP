use crate::structures;
use std::ops::{Add, Mul, Div, Sub};
use num_integer::Integer;

use std::borrow::Cow;

extern crate num_bigint_dig;

use num_bigint::{ToBigUint};


/**
Solves puzzle (using repeated squaring).
**/
pub fn solve(pp: &structures::Params, z: &structures::Puzzle) ->num_bigint::BigUint{
    let one =1.to_biguint().unwrap();
    let n_squared =(&pp.n).mul(&pp.n);
    let w= num_bigint_dig::algorithms::mod_inverse(
        Cow::Borrowed(&(repeated_squaring(&z.u, &pp.t, &pp.n)).modpow(&pp.n, &n_squared)),
        Cow::Borrowed(&n_squared)).unwrap().to_biguint().unwrap();
    let s=(((&z.v).mul(&w).mod_floor(&n_squared)).sub(&one)).div(&pp.n);
    return s;
}


fn repeated_squaring(u:&num_bigint::BigUint, t:&num_bigint::BigUint, n:&num_bigint::BigUint) ->num_bigint::BigUint{
    let one =1.to_biguint().unwrap();
    let mut x =0.to_biguint().unwrap();
    let mut res = u.clone();
    loop {
        if x.eq(&t) == true {
            break;
        }
        x = x.add(&one);
        res = (&res).mul(&res);
    }
    return res.mod_floor(n);
}
