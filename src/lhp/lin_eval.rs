use crate::structures;
use std::ops::{Mul};
use num_integer::Integer;
extern crate num_bigint_dig;

/**
Homomorphism combine two puzzles (s.t. the resulting puzzle's secret is the sum of the input's puzzles secrets).
**/
pub fn add(pp: &structures::Params, z1: &structures::Puzzle, z2: &structures::Puzzle) ->structures::Puzzle{
    let u_out=(z1.u.clone()).mul(&z2.u).mod_floor(&pp.n);
    let v_out=(z1.v.clone()).mul(&z2.v).mod_floor(&(&pp.n).mul(&pp.n));
    return structures::Puzzle{u:u_out,v:v_out };
}