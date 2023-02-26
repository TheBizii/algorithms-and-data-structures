// Returns a tuple, where first element is GCD, second and third elements are coefficients s and t of Bezout's identity, such that:
// as + bt = gcd(a, b)
// If you're dealing with very large numbers, consider using the iterative version below (extended_euclidean_iterative).
pub fn extended_euclidean(a: i32, b: i32) -> (i32, i32, i32) {
    // When a is equal to zero, the above formula shoud look like: 0*0 + b*1 = gcd(0, b).
    // This simplifies to b = gcd(0, b), which is a correct result according to the rules of the Euclidean algorithm.
    if a == 0 {
        return (b, 0, 1);
    }

    // Recursively call extended_euclidean until we calculate the GCD of initial values of a and b.
    let (gcd, s1, t1) = extended_euclidean(b % a, a);

    // Function call stack should then backtrack and reconstruct the formula as shown in an explanation.
    let s = t1 - (b / a) * s1;
    let t = s1;

    return (gcd, s, t);
}

// This is the iterative version of the extended Euclidean algorithm. It is especially useful when dealing with very large numbers,
// because we're avoiding recursion, which could lead to stack overflow.
pub fn extended_euclidean_iterative(a: i32, b: i32) -> (i32, i32, i32) {
	let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;

        let temp_r = old_r;
        old_r = r;
        r = temp_r - quotient * r;
        
		let temp_s = old_s;
        old_s = s;
        s = temp_s - quotient * s;
        
		let temp_t = old_t;
        old_t = t;
        t = temp_t - quotient * t;
    }

    (old_r, old_s, old_t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = extended_euclidean(35, 15);
        assert_eq!(result, (5, 1, -2));
        let result = extended_euclidean_iterative(35, 15);
        assert_eq!(result, (5, 1, -2));

        let result = extended_euclidean(24, 18);
        assert_eq!(result, (6, 1, -1));
        let result = extended_euclidean_iterative(24, 18);
        assert_eq!(result, (6, 1, -1));

        let result = extended_euclidean(30, 50);
        assert_eq!(result, (10, 2, -1));
        let result = extended_euclidean_iterative(30, 50);
        assert_eq!(result, (10, 2, -1));

        let result = extended_euclidean(17, 23);
        assert_eq!(result, (1, -4, 3));
        let result = extended_euclidean_iterative(17, 23);
        assert_eq!(result, (1, -4, 3));

        let result = extended_euclidean(50, 30);
        assert_eq!(result, (10, -1, 2));
        let result = extended_euclidean_iterative(50, 30);
        assert_eq!(result, (10, -1, 2));

        let result = extended_euclidean(45, 120);
        assert_eq!(result, (15, 3, -1));
        let result = extended_euclidean_iterative(45, 120);
        assert_eq!(result, (15, 3, -1));
    }
}
