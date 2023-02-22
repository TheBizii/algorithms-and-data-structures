# Euclid's Algorithm
## Introduction

Euclid's algorithm is an efficient method for computing the [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor) of two integers (numbers), the largest number that divides them both without a [remainder](https://en.wikipedia.org/wiki/Remainder).

## 🤔 How does it work?

Let's calculate the greatest common divisor for _a = 25_ and _b = 20_.

The algorithm works by repeatedly diving those two numbers and calculating remainders.

### Iteration number one

Divide 25 by 20 and note that the quotient is 1 and the remainder is 5. In other words, you can express 25 as: 25 = 1 * 20 + 5.

### Iteration number two

Now take the previous quotient and divide it by the remainder. We get 20 / 5 = 4. In other words, 20 = 4 * 5 + 0. 

We can see that the remainder is 0 now, which means that we can stop iterations and return our greatest common divisor.

We successfully found the greatest common divisor between 25 and 20 and it is 5.

### Notes

We can also calculate GCD of negative numbers by transforming them into positive numbers first. Mathematical proof for this can be found [here](https://proofwiki.org/wiki/GCD_for_Negative_Integers).

Calculating GSD for more than two numbers is possible with a simple trick, which is as follows: GCD(a, b, c) == GCD(GCD(a, b), c). This can be applied to any number of integers.

## 📚 Further reading

 1. https://en.wikipedia.org/wiki/Euclidean_algorithm
 2. https://en.wikipedia.org/wiki/Greatest_common_divisor
 3. https://en.wikipedia.org/wiki/Remainder
 4. https://proofwiki.org/wiki/GCD_for_Negative_Integers
