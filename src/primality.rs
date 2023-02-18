pub use crate::utils::{mod_exp, jacobi, is_perfect_power};
pub use rand::{seq::IteratorRandom, thread_rng};
pub use num::traits::Pow;

pub enum Primality {
    Prime,
    Composite,
    ProbablyPrime
}

pub mod probabilistic { 
    use super::*;

    pub fn fermat_primality_test(n: u64, n_iter: usize) -> Primality {
        let mut rng = thread_rng();
        let v: Vec<u64> = (2..n).collect();
        let bases = v.iter().choose_multiple(&mut rng, n_iter);
        
        for a in bases {
            if mod_exp(*a, n - 1,  n) != 1 {
                return Primality::Composite;
            }
        }
        Primality::ProbablyPrime
    }

    pub fn solovay_strassen_primality_test(n: u64, n_iter: usize) -> Primality {
        if n % 2 == 0 {
            return Primality::Composite;
        }
        let mut rng = thread_rng();
        let v: Vec<u64> = (2..n).collect();
        let bases = v.iter().choose_multiple(&mut rng, n_iter);
        
        for a in bases {
            let jacobi_ = jacobi(*a, n);
            let jacobi_symbol: u64 = match jacobi_ {
                0 | 1 => jacobi_ as u64,
                _ => n - 1, 
            };
    
            if jacobi_symbol == 0 {
                println!("jacobi = 0");
                return Primality::Composite;
            } else if mod_exp(*a, (n - 1) / 2, n) != jacobi_symbol {
                println!("jacobi = {}", jacobi_symbol);
                return Primality::Composite;
            }
        }
        Primality::ProbablyPrime
    }


    pub fn miller_rabin_primality_test(n: u64, n_iter: u64) -> Primality {
        if n % 2 == 0 {
            return Primality::Composite;
        }
    
        let mut rng = thread_rng();
        let v: Vec<u64> = (2..n).collect();
        let bases = v.iter().choose_multiple(&mut rng, n_iter as usize);
    
        // n - 1 = 2^s * n', s > 1
        let s: u64 = (n - 1).trailing_zeros() as u64;
        let n_dash: u64 = (n - 1) >> s;
    
    
        for a in bases {
            let mut b = mod_exp(*a, n_dash, n);
            let mut j: u64 = 0;
            
            while (j <= s - 2) && (b != n - 1) {
                b = b * b % n;
                if b == 1 {
                    return Primality::Composite;
                }
                j +=1;
            }
            if b != n - 1 {
                return Primality::Composite;
            }
        }
        if n_iter < n {
            return Primality::ProbablyPrime;
        }
        Primality::Prime
        }
}

pub mod deterministic {
    // TODO
}