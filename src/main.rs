pub mod polynomial;
mod utils;

pub use utils::*;

use crate::polynomial::Polynomial;
pub mod primality;

fn main() {
    let p: Polynomial<u128> = polynomial::Polynomial{coeffs: vec![1,1]};
    let q:Polynomial<u128> = polynomial::Polynomial{coeffs: vec![1, 2, 1]};

    println!("({}) * ({}) = {}", p.clone(), q.clone(), p * q);
}




#[cfg(test)]
mod tests {
    use num::ToPrimitive;
    use num::BigInt;
    use super::*;

    #[test]
    fn test_gcd() {
        let a: u64 = 82289148;
        let b: u64 = 61345370799;
        let c: u64 = 25515131265;
        let n1 = match "12012156372189739281216757210576102012012223892738921798326187361278368721631225467389874637823746372838746543782374637283476543728374637283746543728253243534255432354234523452345234523452345234523452345234523452345234623623523452345234582848248248284"
                               .parse::<BigInt>() {
                                    Ok(n) => n,
                                    Err(_) => panic!("what")
                                };
        let n2 = match "123565214386213485217365412738765123873265321983721987398217398217893721983773265812"
                                .parse::<BigInt>() {
                                    Ok(n) => n,
                                    Err(_) => panic!("what")
                                };

        assert_eq!(gcd(a, a), a);
        assert_eq!(bgcd(a, a), a);
        assert_eq!(bgcd(0 as u64, a), a);
        assert_eq!(gcd(0 as u64, a), a);
        assert_eq!(gcd(b, c), 9 as u64);
        assert_eq!(bgcd(b, c), 9 as u64);
        assert_eq!(gcd::<BigInt>(n1, n2).to_i8().expect("not a small enough number"), 76);
    }

    #[test]
    fn test_jacobi() {
        assert_eq!(jacobi(610, 987), -1);
        assert_eq!(jacobi(1001, 9907), -1);
        assert_eq!(jacobi(219, 383), 1);
        assert_eq!(jacobi(3, 27), 0);
    }

    #[test]
    fn test_is_perfect_power() {
        assert_eq!(is_perfect_kth_power(2*2*2*2*2, 5), true);
        assert_eq!(is_perfect_power(2*2*2*2*2*3), false);
    }

    #[test]
    fn test_polynomial_ops() {
        let p = &polynomial::Polynomial{coeffs: vec![1,1]};
        let q = &polynomial::Polynomial{coeffs: vec![1, 2, 1]};
        let r = &polynomial::Polynomial{coeffs: vec![2, 3, 1]};
        assert_eq!(p * p, *q);
        assert_eq!(p + q, *r);

        //  trailing zero test
        let t = &polynomial::Polynomial{coeffs: vec![1,1]};
        let u = &polynomial::Polynomial{coeffs: vec![1, 1, 0]};
        assert_eq!(t, u);
    }

    #[test]
    fn test_polynomial_call() {
        let q = polynomial::Polynomial{coeffs: vec![1, 2, 1]};
        assert_eq!(q.call(0), 1);
        assert_eq!(q.call(-1), 0);
    }
}