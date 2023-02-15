pub use crate::utils::*;
pub use crate::polynomial;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(82289148, 82289148), 82289148);
        assert_eq!(bgcd(82289148, 82289148), 82289148);
        assert_eq!(bgcd(0, 82289148), 82289148);
        assert_eq!(gcd(0, 82289148), 82289148);
        assert_eq!(gcd(61345370799, 25515131265), 9);
        assert_eq!(bgcd(61345370799, 25515131265), 9);
        assert_eq!(gcd(45068332734, 75461940078), bgcd(45068332734, 75461940078));
        assert_eq!(gcd(1125899906842624, 1024), bgcd(1125899906842624, 1024));
    }

    #[test]
    fn test_jacobi() {
        assert_eq!(jacobi(610, 987), -1);
        assert_eq!(jacobi(1001, 9907), -1);
        assert_eq!(jacobi(219, 383), 1);
        assert_eq!(jacobi(123, 1234), 1);
        assert_eq!(jacobi(2, 1234), 0);
        assert_eq!(jacobi(3, 27), 0);
    }

    #[test]
    fn test_is_perfect_power() {
        assert_eq!(is_perfect_power(2*2*2*2*2), true);
        assert_eq!(is_perfect_power(2*2*2*2*2*3), false);
    }

    #[test]
    fn test_polynomials() {
        let p = &polynomial::Polynomial{coeffs: vec![1,1]};
        let q = &polynomial::Polynomial{coeffs: vec![1, 2, 1]};
        let r = &polynomial::Polynomial{coeffs: vec![1, 3, 1]};
        assert_eq!(p * p, q);
        assert_eq!(p + q, r);
    }
}