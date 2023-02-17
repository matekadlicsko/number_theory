use std::cmp::{max, PartialEq};
use std::ops::{Add, Mul, Sub, AddAssign};
use num::traits::{Zero};


#[derive(Clone, Debug)]
pub struct Polynomial<T> {
    pub coeffs: Vec<T>
}

impl<T> Polynomial<T> where
    T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> +
       Clone + PartialEq + Zero {
    pub fn drop_trailing_zeros(&self) -> Self {
        if self.coeffs.len() <= 1 {
            return Self{coeffs: self.coeffs.to_vec()};
        }
        let last_coeff = self.coeffs[self.coeffs.len() - 1].clone();
        if last_coeff.is_zero() {
            return Self{coeffs: self.coeffs[..self.coeffs.len() - 1].to_vec()}.drop_trailing_zeros();
        }
        return Self{coeffs: self.coeffs.to_vec()};
    }

    pub fn degree(&self) -> usize {
        return self.drop_trailing_zeros().coeffs.len() - 1;
    }

    pub fn nth_coeff(&self, &n: &usize) -> T {
        if n <= self.degree() {
            return self.coeffs[n].clone();
        }
        return T::zero();
    }
}

impl<T> Add<&Polynomial<T>> for &Polynomial<T> where
    T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> +
       Clone + PartialEq + Zero {
    type Output = Polynomial<T>;
    fn add(self, other: &Polynomial<T>) -> Polynomial<T> {
        let mut coeffs: Vec<T> = Vec::new(); 
        for i in 0..=max(self.degree(), other.degree()) {
            let ith_comp: T = self.nth_coeff(&i).clone() + other.nth_coeff(&i).clone();
            coeffs.push(ith_comp);
        }
        return Polynomial{ coeffs };        
    }
}

impl<T> Mul<&Polynomial<T>> for &Polynomial<T> where
    T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> +
       Clone + PartialEq + Zero + AddAssign {
    type Output = Polynomial<T>;
    fn mul(self, other: &Polynomial<T>) -> Polynomial<T> {
        let degree: usize = self.degree() + other.degree();
        let mut coeffs: Vec<T> = vec![T::zero(); degree + 1];

        for i in 0..=self.degree() {
            for j in 0..=other.degree() {
                coeffs[i + j] += self.coeffs[i].clone() * other.coeffs[j].clone();
            }
        }
        return Polynomial{ coeffs };        
    }
}

impl<T> PartialEq<Polynomial<T>> for Polynomial<T> where
    T:  Add<Output = T> + Mul<Output = T> + Sub<Output = T> +
        Clone + PartialEq + Zero {
    fn eq(&self, other: &Polynomial<T>) -> bool {
        self.drop_trailing_zeros().coeffs == other.drop_trailing_zeros().coeffs
    }

    fn ne(&self, other: &Polynomial<T>) -> bool {
        self.drop_trailing_zeros().coeffs != other.drop_trailing_zeros().coeffs
    }
}