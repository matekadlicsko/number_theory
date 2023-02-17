use std::{cmp::min, ops::{Rem, BitAnd, Shr, Shl, Sub, AddAssign, ShrAssign, SubAssign}};
use num::{Zero, One};

pub fn gcd<T>(mut a: T, mut b: T) -> T where
    T: Rem<Output = T> + Zero + Clone {
    if a.is_zero() {
        return b;
    } else if b.is_zero() {
        return a;
    }
    let mut t: T;
    while !b.is_zero() {
        t = b.clone();
        b = a % b;
        a = t;
    }
    return a
}


pub fn bgcd<T>(mut a: T, mut b: T) -> T where
    T: Rem<Output = T> + Zero + One + Copy
       + BitAnd<Output = T> + Shr<Output = T> 
       + Shl<Output = T> + Sub<Output = T> 
       + Ord + AddAssign + SubAssign + ShrAssign
       {
    if a.is_zero() {
        return b;
    }
    else if b.is_zero() {
        return a;
    }

    // We can assume that a and b are both odd
    // a = a' * 2^s, b = b' * 2^t, s <= t
    let mut s: T = T::zero();
    let mut t: T = T::zero();

    while (a & T::one()).is_zero() {
        s += T::one();
        a >>= T::one();    
    }
    
    while (b & T::one()).is_zero() {
        t += T::one();
        b >>= T::one();
    }

    s = min(s, t);

    while !b.is_zero() {
        if a > b {
            // gcd(a', b') = gcd(a' - b', b')
            // a' - b' = 2 ^ r * x, r > 0
            // thus gcd(a', b') = gcd(b', x)
            a -= b;
            while (a & T::one()).is_zero() {a >>= T::one();}
        }
        else if a < b {
            b -= a;
            while (b & T::one()).is_zero() {b >>= T::one();}
        }
        else {
            b = T::zero();
        }
    }
    // gcd(a, b) = 2^s * gcd(a', b') 
    return a << s
}


#[allow(dead_code)]
pub fn dumb_mod_exp(a: u64, n: u64, m: u64) -> u64 {
    let mut t = 1;
    for _i in 0..n {
        t = t * a % m; 
    }
    return t % m;
}

pub fn mod_exp(a: u64, n: u64, m: u64) -> u64 {
    let mut k = 0;
    let mut an = a;
    let mut result: u64 = 1;

    while (n >> k) > 0 {
        if (n >> k) & 1 == 1 {
            result = (result * an) % m;
        }
        an = (an * an) % m;
        k += 1;
    }
    return result;
}

// jacobi(a,n) = (a / n)
pub fn jacobi(mut a: u64, mut n: u64) -> i8 {
    if n % 2 == 0 {
        panic!("The Jacobi symbol (a / b) is not defined for even b.");
    }

    a = a % n;
    let mut t: i8 = 1;
    let mut r: u64;
    while a > 0 {
        while a % 2 == 0 {
            a = a >> 1; // equivalent to a = a / 2;
            r = n % 8;
            if r == 3 || r == 5 {
                t = -1;
            }
        }
        r = n;
        n = a;
        a = r;
        if a % 4 == 3 && n % 4 == 3 {
            t = -t;
        }
        a = a % n;
    }
    if n == 1 {
        return t;
    }
    return 0;
}


fn bit_length(n: u64) -> u64 {
    let mut t: u64 = 1;
    while n >> t != 0 {t += 1;}
    return t;
}

// Newton-Raphson
pub fn is_perfect_kth_power(n: u64, power: u64) -> bool {
    assert!((n > 1) && (power > 1));
    let bitlength = bit_length(n);
    let mut approximation = 2u64.pow(((bitlength + power - 1) / power) as u32);
    loop {
        let t: u64 = approximation.pow((power - 1) as u32);
        let b: u64 = ((power - 1) * approximation * t + n) / (power * t);
        if approximation <= b {
            approximation = b;
            break;
        } else {
            approximation = b;
        }
    }
    if n == approximation.pow(power as u32) {
        return true;
    }
    return false;
}


pub fn is_perfect_power(n: u64) -> bool {
    let bl = bit_length(n);

    for k in 2..bl + 1 {
        // We need not check for composite k-s
        // TODO: check only for precomputed list of primes when possible.
        if is_perfect_kth_power(n, k) {
            return true;            
        }
    }
    return false;
}