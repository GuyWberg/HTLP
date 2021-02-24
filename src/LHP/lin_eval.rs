use crate::structures;
use std::ops::{Mul};
use num_integer::Integer;
extern crate num_bigint_dig;

pub fn add(pp: &structures::Params, Z1: &structures::Puzzle,Z2: &structures::Puzzle) ->structures::Puzzle{
    let u_out=(Z1.u.clone()).mul(&Z2.u).mod_floor(&pp.N);
    let v_out=(Z1.v.clone()).mul(&Z2.v).mod_floor(&(&pp.N).mul(&pp.N));
    return structures::Puzzle{u:u_out,v:v_out };
}