pub mod polynomial;
mod utils;
use std::{array, any::type_name};

pub use utils::*;
pub mod primality;

fn main() {
    let mut a = [1; 3];
    a[1] = 2;
    println!("{:?}", a);

}