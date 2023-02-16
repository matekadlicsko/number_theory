pub mod polynomial;
mod utils;

pub use utils::*;
pub mod primality;
use num::BigInt;

fn main() {
    let n1 = match "12012156372189739281216757210576102012012223892738921798326187361278368721631225467389874637823746372838746543782374637283476543728374637283746543728253243534255432354234523452345234523452345234523452345234523452345234623623523452345234582848248248284".parse::<BigInt>() {
        Ok(n) => n,
        Err(_) => panic!("what")
    };
    let n2 = match "123565214386213485217365412738765123873265321983721987398217398217893721983773265812".parse::<BigInt>() {
        Ok(n) => n,
        Err(_) => panic!("what")
    };
    println!("{:?}", gcd(n1, n2));
}

pub use crate::utils::*;
//pub use crate::polynomial;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(82289148, 82289148), 82289148);
        assert_eq!(bgcd(82289148, 82289148), 82289148);
        assert_eq!(bgcd(0, 82289148), 82289148);
        assert_eq!(gcd(0, 82289148), 82289148);
        assert_eq!(gcd(61345370799 as u64, 25515131265 as u64), 9);
        assert_eq!(bgcd(61345370799, 25515131265), 9);
        assert_eq!(gcd(45068332734, 75461940078), bgcd(45068332734, 75461940078));
        assert_eq!(gcd(1125899906842624, 1024), bgcd(1125899906842624, 1024));
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
    fn test_polynomials() {
        let p = &polynomial::Polynomial{coeffs: vec![1,1]};
        let q = &polynomial::Polynomial{coeffs: vec![1, 2, 1]};
        let r = &polynomial::Polynomial{coeffs: vec![2, 3, 1]};
        assert_eq!((p * p).coeffs, q.coeffs);
        assert_eq!((p + q).coeffs, r.coeffs);
    }
}