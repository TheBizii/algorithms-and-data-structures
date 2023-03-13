// This function generates a list of all prime numbers up to a given limit of n.
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    // Create a list of all numbers from 2 to n. Zero and one are excluded, because we know they are not prime.
    // At the end of the function, true will signify whether the corresponding number is prime or not.
    let mut primes = vec![true; n - 1];

    // We only need to loop from 2 to sqrt(n).
    let sqroot = (n as f64).sqrt() as usize;

    // Perform the Sieve of Eratosthenes.
    for i in 2..=sqroot {
        // Skip if already marked as non-prime.
        if !primes[i - 2] {
            continue;
        }

        for j in (i * i..=n).step_by(i) {
            primes[j - 2] = false;
        }
    }

    // Remaining numbers marked as true are prime.
    return primes
        .iter()
        .enumerate()
        .filter(|(_, &p)| p)
        .map(|(index, _)| index + 2)
        .collect::<Vec<_>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sieve_of_eratosthenes(6);
        assert_eq!(result, vec![2, 3, 5]);

        let result = sieve_of_eratosthenes(37);
        assert_eq!(result, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]);

        let result = sieve_of_eratosthenes(62);
        assert_eq!(
            result,
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61]
        );

        let result = sieve_of_eratosthenes(18);
        assert_eq!(result, vec![2, 3, 5, 7, 11, 13, 17]);
    }
}
