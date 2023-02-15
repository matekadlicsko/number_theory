use std::cmp::max;
use std::ops::{Add, Mul};

#[derive(Debug)]
#[derive(Clone)]
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

impl Add<&Polynomial> for &Polynomial {
    type Output = Polynomial;
    fn add(self, other: &Polynomial) -> Polynomial {
        let mut coeffs = Vec::new(); 
        for i in 0..=max(self.degree(), other.degree()) {
            coeffs.push(self.nth_coeff(&i) + other.nth_coeff(&i));
        }
        return Polynomial{ coeffs };        
    }
}

impl Mul<&Polynomial> for &Polynomial {
    type Output = Polynomial;
    fn mul(self, other: &Polynomial) -> Polynomial {
        let degree: usize = self.degree() + other.degree();
        let mut coeffs: Vec<u64> = Vec::<u64>::with_capacity(degree + 1);
        for _ in 0..=degree {
            coeffs.push(0);
        }
        println!("coeffs: {:?}", coeffs);
        for i in 0..=self.degree() {
            for j in 0..=other.degree() {
                coeffs[i + j] += self.coeffs[i] * other.coeffs[j];
            }
        }
        return Polynomial{ coeffs };        
    }
}