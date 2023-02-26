# Extended Euclidean Algorithm
## Introduction

Extended Euclidean algorithm is an extension to the [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm "Euclidean algorithm"), and computes, in addition to the [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor "Greatest common divisor") (gcd) of integers __a__ and __b__, also the coefficients of [Bézout's identity](https://en.wikipedia.org/wiki/B%C3%A9zout%27s_identity "Bézout's identity"), which are integers __x__ and __y__ such that __ax + by = gcd(a, b)__.

## 🤔 How does it work?

Let's calculate the **greatest common divisor** and coefficients **x** and **y** for **a** = 17 and **b** = 23.

Greatest common divisor is calculated the same way as with the original version of the Euclidean algorithm. Its explanation and implementations can be found in this repository.

### Finding the greatest common divisor

As mentioned earlier, the explanation and implementations are available in this repository. Here's a quick way to find the greatest common divisor.

[1] 23 = 17 * (1) + 6

[2] 17 = 6 * (2) + 5

[3] 6 = 5 * (1) + 1

[4] 5 = 1 * (5) + 0

The greatest common divisor of numbers **a** and **b** is 1.

### Rewriting the above equations

After finding the greatest common divisor, it's time rewrite the above equations in the following way:

1. Take the equation [3] and express it in the following form.

1 = 6 - 5 * (1)

2. Rewrite the above expression by switching the sign in front of number 5.

1 = 6 + 5 * (-1)

3. Take the equation [2] and replace the number 5 in the expression above.

1 = 6 + (17 - 6 * (2)) * (-1)

4. Simplify the expression.

1 = 6 + (17 + 6 * (-2)) * (-1)

= 6 + 17 * (-1) + 6 * (2)

= 6 * (3) + 17 * (-1)

5. Take the equation [1] and replace the number 6 in the expression above.

1 = (23 - 17 * (1)) * (3) + 17 * (-1)

6. Simplify the expression.

1 = (23 + 17 * (-1)) * (3) + 17 * (-1)

= 23 * (3) + 17 * (-3) + 17 * (-1)

= 23 * (3) + 17 * (-4)

### Conclusion

The greatest common divisor of numbers **a** and **b** is 1, while their corresponding coefficients of Bézout's identity are **x = -4** and **y = 3**.

## 📚 Further reading

 1. https://en.wikipedia.org/wiki/Euclidean_algorithm
 2. https://en.wikipedia.org/wiki/Greatest_common_divisor
 3. https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
 4. https://en.wikipedia.org/wiki/B%C3%A9zout%27s_identity
 5. [Additional explanation on YouTube](https://www.youtube.com/watch?v=6KmhCKxFWOs)
