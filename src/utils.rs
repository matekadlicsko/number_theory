use std::{cmp::min, ops::{Rem, BitAnd, Shr, Shl, Sub, AddAssign, ShrAssign, SubAssign}};
use num::{Zero, One, Integer};

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


pub fn mod_exp<T, U>(a: T, n: U, m: T) -> T where
    U:  Integer + Shr<Output = U> + Ord + BitAnd<Output = U> + AddAssign + Clone,
    T:  One + Rem<Output = T> + Clone {
    let mut an = a;
    let mut result: T = T::one();

    let mut x = n;

    while x > U::zero() {
        if (x.clone() & U::one()).is_one() {
            result = (result * an.clone()) % m.clone();
        }
        an = (an.clone() * an.clone()) % m.clone();
        x = x >> U::one();
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

pub enum PerfectPower<T, U> {
    NotPerfectPower,
    Decomp(T, U)
}

// Newton-Raphson
pub fn is_perfect_kth_power(n: u64, power: u64) -> PerfectPower<u64, u64> {
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
        return PerfectPower::Decomp(approximation, power);
    }
    PerfectPower::NotPerfectPower
}

pub fn is_perfect_power(n: u64) -> PerfectPower<u64, u64> {
    let bl = bit_length(n);

    for k in 2..bl + 1 {
        // We need not check for composite k-s
        // TODO: check only for precomputed list of primes when possible.
        match is_perfect_kth_power(n, k) {
            PerfectPower::Decomp(a, n) => return PerfectPower::Decomp(a, n),
            _ => "do nothing?"
        };
    }
    return PerfectPower::NotPerfectPower;
}