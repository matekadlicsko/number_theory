use std::cmp::max;
use std::ops::{Add, Mul, Div};

#[derive(Debug)]
pub struct Polynomial {
    pub coeffs: Vec<u64>
}

impl Polynomial {
    pub fn drop_trailing_zeros(&self) -> Self {
        if self.coeffs.len() <= 1 {
            return Self{coeffs: self.coeffs.to_vec()};
        }
        else if self.coeffs[self.coeffs.len() - 1] == 0 {
            return Self{coeffs: self.coeffs[..self.coeffs.len() - 1].to_vec()}.drop_trailing_zeros();
        }
        return Self{coeffs: self.coeffs.to_vec()};
    }

    pub fn degree(&self) -> usize {
        return self.drop_trailing_zeros().coeffs.len() - 1;
    }

    pub fn nth_coeff(&self, &n: &usize) -> u64 {
        if n <= self.degree() {
            return self.coeffs[n];
        }
        return 0;
    }
}


impl Add for Polynomial {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut coeffs = Vec::new(); 
        for i in 0..=max(self.degree(), other.degree()) {
            coeffs.push(self.nth_coeff(&i) + other.nth_coeff(&i));
        }
        return Self{ coeffs };        
    }
}

impl Mul for Polynomial {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut coeffs = Vec::new(); 
        for i in 0..=self.degree() + other.degree() {
            coeffs.push(self.nth_coeff(&i) + other.nth_coeff(&i));
        }
        return Self{ coeffs };        
    }
}