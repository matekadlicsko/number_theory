use std::cmp::{max, PartialEq};
use std::ops::{Add, Mul, Sub, SubAssign, AddAssign, MulAssign, Div, Rem};
use num::traits::{Zero, One};


#[derive(Clone, Debug)]
pub struct Polynomial<T> {
    pub coeffs: Vec<T>
}

impl<T> Polynomial<T> where T: Clone + Zero {
    pub fn nth_coeff(&self, &n: &usize) -> T {
        if n < self.coeffs.len() {
            return self.coeffs[n].clone();
        }
        return T::zero();
    }
}


impl<T> Polynomial<T> where 
T:  Mul<Output = T> + AddAssign + Clone + Zero + One + MulAssign {
    pub fn call(&self, x: T) -> T {
        let mut sum: T = T::zero();
        let mut t: T = T::one();
        for i in 0..self.coeffs.len() {
            sum += self.coeffs[i].clone() * t.clone();
            t *= x.clone();
        }
        sum
    }
}


impl<T> Polynomial<T> where
    T: Add<Output = T> + Clone + PartialEq + Zero {
    pub fn drop_trailing_zeros(&self) -> Self {
        if self.coeffs.len() <= 1 {
            return Self{coeffs: self.coeffs.to_vec()};
        }
        let last_coeff = self.coeffs[self.coeffs.len() - 1].clone();
        if last_coeff.is_zero() {
            return Self{coeffs: self.coeffs[..self.coeffs.len() - 1].to_vec()}.drop_trailing_zeros();
        }
        Self{coeffs: self.coeffs.to_vec()}
    }


    pub fn degree(&self) -> usize {
        self.drop_trailing_zeros().coeffs.len() - 1
    }

    
    pub fn leading_coefficient(&self) -> T {
        match self.drop_trailing_zeros().coeffs.last() {
            Some(n)  => n.clone(),
            None => panic!("Last coefficient is none!")
        }
    }
}

// Polynomial - Scalar operations
impl<T> Sub<T> for Polynomial<T> where 
    T:  SubAssign + Clone + Zero + PartialEq {
    type Output = Polynomial<T>;

    fn sub(self, rhs: T) -> Self::Output {
        let mut coeffs = self.coeffs.clone();
        coeffs[0] -= rhs;
        return Polynomial{ coeffs };
    }
    
}

impl<T> Add<T> for Polynomial<T> where 
    T:  AddAssign + Clone + Zero + PartialEq {
    type Output = Polynomial<T>;

    fn add(self, rhs: T) -> Self::Output {
        let mut coeffs = self.coeffs.clone();
        coeffs[0] += rhs;
        return Polynomial{ coeffs };
    }
    
}


impl<T> Mul<T> for Polynomial<T> where
    T: Mul<Output = T> + Clone + PartialEq + Zero {
    type Output = Polynomial<T>;
    fn mul(self: Polynomial<T>, scalar: T) -> Self::Output {
        Polynomial::<T>{ coeffs: self.coeffs.iter().map(|v| v.clone() * scalar.clone()).collect()}
    }
}

impl<T> Div<T> for Polynomial<T> where
    T: Div<Output = T> + Clone + PartialEq + Zero {
    type Output = Polynomial<T>;
    fn div(self: Polynomial<T>, scalar: T) -> Self::Output {
        Polynomial::<T>{ coeffs: self.coeffs.iter().map(|v| v.clone() / scalar.clone()).collect()}
    }
}

impl<T> Rem<T> for Polynomial<T> where
    T: Rem<Output = T> + Clone + PartialEq + Zero {
    type Output = Polynomial<T>;
    fn rem(self: Polynomial<T>, scalar: T) -> Self::Output {
        Polynomial::<T>{ coeffs: self.coeffs.iter().map(|v| v.clone() % scalar.clone()).collect()}
    }
}


// Polynomial - Polynomial operations
impl<T> Add<Polynomial<T>> for Polynomial<T> where
    T: Add<Output = T> + Clone + Zero + PartialEq  {
    type Output = Polynomial<T>;
    fn add(self, other: Polynomial<T>) -> Self::Output {
        let mut coeffs: Vec<T> = Vec::new(); 
        for i in 0..=max(self.degree(), other.degree()) {
            let ith_comp: T = self.nth_coeff(&i).clone() + other.nth_coeff(&i).clone();
            coeffs.push(ith_comp);
        }
        Polynomial{ coeffs }        
    }
}


impl<T> Sub<Polynomial<T>> for Polynomial<T> where
    T: Sub<Output = T> + Clone + Zero + PartialEq  {
    type Output = Polynomial<T>;
    fn sub(self, other: Polynomial<T>) -> Self::Output {
        let mut coeffs: Vec<T> = Vec::new();
        for i in 0..=max(self.degree(), other.degree()) {
            let ith_comp: T = self.nth_coeff(&i).clone() - other.nth_coeff(&i).clone();
            coeffs.push(ith_comp);
        }
        Polynomial{ coeffs }
    }
}


impl<T> Mul<Polynomial<T>> for Polynomial<T> where
    T: Mul<Output = T> + Sub<Output = T> +
       Clone + PartialEq + Zero + AddAssign {
    type Output = Polynomial<T>;
    fn mul(self, other: Polynomial<T>) -> Polynomial<T> {
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

fn div_rem<T>(lhs: Polynomial<T>, rhs: Polynomial<T>) -> (Polynomial<T>, Polynomial<T>) where 
    T:  Div<Output = T> + AddAssign + Mul<Output = T> + Clone + Zero + PartialEq + SubAssign {
    if rhs.is_zero() {
        panic!("Division by zero.");
    }

    let mut remainder = Polynomial::<T>::zero();
    let mut quotient = lhs.clone();
    let mut t: T = T::zero();

    while !quotient.is_zero() & (quotient.degree() >= rhs.degree()) {
        if t.clone() == quotient.leading_coefficient() {
            println!("t is zero");
            break;
        }

        t = quotient.leading_coefficient();

        
        remainder = remainder.clone() + t.clone();
        quotient = quotient + remainder.clone() * t.clone();

    }

    (quotient, remainder)
}


impl<T> Div<Polynomial<T>> for Polynomial<T> where
    T:  Div<Output = T> + AddAssign + Mul<Output = T> +
        Clone + Zero + PartialEq + SubAssign {
    type Output = Polynomial<T>;

    fn div(self, rhs: Polynomial<T>) -> Self::Output {
        div_rem(self, rhs).0
    }
    
}


impl<T> Rem<Polynomial<T>> for Polynomial<T> where
    T:  Div<Output = T> + AddAssign + Mul<Output = T> +
        Clone + Zero + PartialEq + SubAssign {
    type Output = Polynomial<T>;

    fn rem(self, rhs: Polynomial<T>) -> Self::Output {
        div_rem(self, rhs).1
    }
    
} 


// Identities
impl <T> Zero for Polynomial<T> where
    T:   Zero + Add<Output = T> + PartialEq + Clone {
    fn is_zero(&self) -> bool {
        self.clone().drop_trailing_zeros().coeffs== vec![T::zero()]
    }

    fn zero() -> Self {
        Polynomial{ coeffs: vec![T::zero()] }
    }
}

impl <T> One for Polynomial<T> where
    T:  Mul<Output = T> + Sub<Output = T> + One + 
        Clone + PartialEq + Zero + AddAssign {
    fn is_one(&self) -> bool {
        self.clone().drop_trailing_zeros().coeffs== vec![T::one()]
    }

    fn one() -> Self {
        Polynomial{ coeffs: vec![T::one()] }
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


// Ref poly - poly operations
macro_rules! forward_ref_ref_binop {
    (impl $imp:ident, $method:ident) => {
        impl<T> $imp<&Polynomial<T>> for &Polynomial<T> where
            T:  AddAssign + Zero + PartialEq + Clone
                + Mul<Output = T> + Sub<Output = T>
                + Div<Output = T> + AddAssign + SubAssign {
            type Output = Polynomial<T>;

            fn $method(self, other: &Polynomial<T>) -> Self::Output {
                self.clone().$method(other.clone())
            }
        }
    }
}

forward_ref_ref_binop!(impl Add, add);
forward_ref_ref_binop!(impl Mul, mul);
forward_ref_ref_binop!(impl Sub, sub);
forward_ref_ref_binop!(impl Div, div);
forward_ref_ref_binop!(impl Rem, rem);

macro_rules! forward_ref_ref_scalar {
    (impl $imp:ident, $method:ident) => {
        impl<T> $imp<&T> for &Polynomial<T> where
            T:  AddAssign + Zero + PartialEq + Div<Output = T>
                + Mul<Output = T> + SubAssign + Clone
                + Rem<Output=T> {
            type Output = Polynomial<T>;

            fn $method(self, other: &T) -> Polynomial<T> {
                self.clone().$method(other.clone())
            }
        }
    }
}

// Ref poly - scalar operations
forward_ref_ref_scalar!(impl Add, add);
forward_ref_ref_scalar!(impl Mul, mul);
forward_ref_ref_scalar!(impl Sub, sub);
forward_ref_ref_scalar!(impl Rem, rem);
forward_ref_ref_scalar!(impl Div, div);