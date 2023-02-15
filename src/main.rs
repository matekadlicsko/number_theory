pub mod polynomial;
mod utils;

pub use utils::*;
pub mod primality;


fn main() {
    let p = &polynomial::Polynomial{coeffs: vec![1,1]};
    let q = &polynomial::Polynomial{coeffs: vec![1,1]};
    println!("{}", p.degree()+ q.degree());
    println!("2 * (1 + x) = {:?}", p + q);
    println!("(1 + x)^2 = {:?}", p * q);
    //println!("{:?}", [0; 10].to_vec());
}