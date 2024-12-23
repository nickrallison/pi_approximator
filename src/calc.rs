use prime_factorization::Factorization;
use rayon::prelude::*;

/// Chi is the following piecewise function:
/// chi(x) = 0  if x % 2 == 0
/// chi(x) = 1  if x % 4 == 1
/// chi(x) = -1 if x % 4 == 3
const fn chi(n: u32) -> i8 {
    let n_rem_4: u8 = (n % 4) as u8;
    match n_rem_4 {
        0 => 0,
        1 => 1,
        2 => 0,
        3 => -1,
        _ => unreachable!(),
    }
}

/// This calculates the sum of chi(prime^x) for all x in [1, max_pow]
fn sum_chi(prime: u32, max_pow: u32) -> u64 {
    let iter = (0..=max_pow);
    iter.into_par_iter()
        .map(|pow| chi((prime).pow(pow)))
        .reduce(|| 0, |a, b| a + b) as u64
}

/// This calculates the number of lattice points on a circle with radius sqrt(N)
/// if N = 2^5 * 3^2 * 5^1 * 7^0, then this would return
/// 4 *
/// (chi(2^5) + chi(2^4) + chi(2^3) + chi(2^2) + chi(2^1) + chi(2^0)) +
/// (chi(3^2) + chi(3^1) + chi(3^0)) +
/// (chi(5^1) + chi(5^0)) +
/// (chi(7^0))
fn num_latice_points_sqrtn(n: u32) -> u64 {
    let factors = prime_factorization::Factorization::run(n);
    // this is a Vec<(prime, power)>
    let prime_factors = factors.prime_factor_repr();
    let points = prime_factors.into_par_iter()
        .map(|(prime, pow)| sum_chi(prime, pow))
        .reduce(|| 1, |a, b| a * b);
    4 * points
}

pub fn num_latice_points_in_circle_sqrtn(n: u32) -> u64 {
    let iter = (1..=n);
    iter.into_par_iter()
        .map(|rad| num_latice_points_sqrtn(rad))
        .reduce(|| 0, |a, b| a + b)
}

pub fn pi_approximation_sqrtn(n: u32) -> f64 {
    let lattice_points = num_latice_points_in_circle_sqrtn(n);
    lattice_points as f64 / n as f64
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(2, 0)]
    #[case(3, -1)]
    #[case(4, 0)]
    #[case(5, 1)]
    #[case(6, 0)]
    #[case(7, -1)]
    #[case(8, 0)]
    fn test_chi(#[case] input: u32, #[case] expected: i8) {
        assert_eq!(chi(input), expected);
    }
    
    #[rstest]
    #[case(2, 2, 1)]
    #[case(2, 3, 1)]
    #[case(2, 4, 1)]
    
    #[case(3, 1, 0)]
    #[case(3, 2, 1)]
    #[case(3, 3, 0)]
    #[case(3, 4, 1)]

    #[case(5, 1, 2)]
    #[case(5, 2, 3)]
    #[case(5, 3, 4)]
    fn test_sum_chi(#[case] prime: u32, #[case] pow: u32, #[case] expected: u64) {
        assert_eq!(sum_chi(prime, pow), expected);
    }
}
