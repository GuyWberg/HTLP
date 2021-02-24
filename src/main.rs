mod util;
mod structures;
mod LHP;
extern crate num_bigint_dig as num_bigint;

use num_bigint::ToBigUint;


fn main() {
    // Example: generate two puzzles and homomorphicly add them
    let pp= LHP::setup::gen(20, 10.to_biguint().unwrap()); // generate parameters that uses 20 bit representation, time parameter is 10
    let z1 = LHP::generate::gen(&pp, 434.to_biguint().unwrap()); // secret for the first puzzle is 434
    let mut s=LHP::solve::solve(&pp,&z1);
    println!("First puzzle solved. Secret: {}",s);
    let z2 = LHP::generate::gen(&pp, 10.to_biguint().unwrap()); // secret for the first puzzle 10
    s=LHP::solve::solve(&pp,&z2);
    println!("Second puzzle solved. Secret: {}",s);
    let z3 = LHP::lin_eval::add(&pp, &z1, &z2); // secret for the third puzzle 444
    s=LHP::solve::solve(&pp,&z3);
    println!("Third (homomorphic evaluation of the previous puzzles) puzzle solved. Secret: {}",s);
}