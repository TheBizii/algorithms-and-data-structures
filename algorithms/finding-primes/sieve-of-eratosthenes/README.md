
# Sieve of Eratosthenes

## Introduction
Sieve of Eratosthenes is an ancient algorithm for finding all [prime numbers](https://en.wikipedia.org/wiki/Prime_number) up to any given limit.

## 🤔 How does it work?
Let's find all prime numbers less than or equal to 30.

The algorithm works by generating a list of all numbers from 2 to 30 and iteratively marking the multiples of each prime number as composite (not prime).

To find all the prime numbers less than or equal to 30, proceed as follows.

First, generate a list of integers from 2 to 30:

**2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30**

The first number in the list is 2; cross out every 2nd number in the list after 2 by counting up from 2 in increments of 2 (these will be all the multiples of 2 in the list). Numbers that are not bold should be considered as crossed out:

**2 3** 4 **5** 6 **7** 8 **9** 10 **11** 12 **13** 14 **15** 16 **17** 18 **19** 20 **21** 22 **23** 24 **25** 26 **27** 28 **29** 30

The next number in the list after 2 is 3; cross out every 3rd number in the list after 3 by counting up from 3 in increments of 3 (these will be all the multiples of 3 in the list):

**2 3** 4 **5** 6 **7** 8 9 10 **11** 12 **13** 14 15 16 **17** 18 **19** 20 21 22 **23** 24 **25** 26 27 28 **29** 30

The next number not yet crossed out in the list after 3 is 5; cross out every 5th number in the list after 5 by counting up from 5 in increments of 5 (i.e. all the multiples of 5):

**2 3** 4 **5** 6 **7** 8 9 10 **11** 12 **13** 14 15 16 **17** 18 **19** 20 21 22 **23** 24 25 26 27 28 **29** 30

The next number not yet crossed out in the list after 5 is 7; the next step would be to cross out every 7th number in the list after 7, but they are all already crossed out at this point, as these numbers (14, 21, 28) are also multiples of smaller primes because 7 × 7 is greater than 30. The numbers not crossed out at this point in the list are all the prime numbers below 30:

**2, 3, 5, 7, 11, 13, 17, 19, 23, 29**

### Notes
When implementing the algorithm yourself, a useful trick to reduce the number of iterations is to only iterate from 2 to the square root of the given limit. In the above example, the square root of 30 is around 5.

When dealing with large numbers, this version of the algorithm may not be optimal due to the memory consumption. For this reason, consider using a [segmented sieve](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Segmented_sieve).

## 📚 Further reading

1. https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes

2. https://en.wikipedia.org/wiki/Prime_number

3. [Segmented sieve](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Segmented_sieve)

4. [Additional explanation on YouTube](https://www.youtube.com/watch?v=klcIklsWzrY)