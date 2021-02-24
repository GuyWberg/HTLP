use crate::structures;
use std::ops::{Sub, Mul, Add};
use num_integer::Integer;
use num_bigint_dig::{ToBigUint, RandBigInt};

extern crate num_bigint_dig as num_bigint;


pub fn gen(pp: &structures::Params ,s: num_bigint::BigUint) ->structures::Puzzle{
    let ONE=1.to_biguint().unwrap();
    let N_suared =(&pp.N).mul(&pp.N);
    let mut rng = rand::thread_rng();
    let r = rng.gen_biguint_range(&ONE,&(&N_suared).sub(&ONE));
    let u_out =(&pp.g).modpow(&r, &pp.N);
    let v_out =
        (((&pp.h).modpow(&((&r).mul(&pp.N)),&N_suared))
            .mul(((&pp.N).add(&ONE)).modpow(&s,&N_suared)))
            .mod_floor(&N_suared);
    return structures::Puzzle{u: u_out,v: v_out };
}
