# Number Theory & Math (Rust)

Each file is a standalone program: `rustc --edition 2021 -O <file>.rs && ./<file>`. Every `main` asserts its own test cases.

| File | Description |
|---|---|
| 0007_reverse_integer.rs | Reverse the digits of an i32 with checked overflow |
| 0009_palindrome_number.rs | Palindrome check by reversing half the digits |
| 0012_integer_to_roman.rs | Greedy integer to Roman numeral conversion |
| 0013_roman_to_integer.rs | Roman numeral to integer with subtractive rule |
| 0029_divide_two_integers.rs | Integer division with bit shifts only |
| 0043_multiply_strings.rs | Grade-school multiplication of decimal strings |
| 0050_pow_x_n.rs | Fast floating-point exponentiation (binary powering) |
| 0060_permutation_sequence.rs | k-th permutation via the factorial number system |
| 0067_add_binary.rs | Add two binary strings with a carry |
| 0069_sqrt_x.rs | Integer square root by binary search |
| 0096_unique_bst_catalan_numbers.rs | Unique BST count = Catalan numbers (closed form + DP) |
| 0149_max_points_on_a_line.rs | Max collinear points via gcd-normalized slopes |
| 0168_excel_sheet_column_title.rs | Number to Excel column title (bijective base 26) |
| 0171_excel_sheet_column_number.rs | Excel column title to number |
| 0172_factorial_trailing_zeroes.rs | Trailing zeroes of n! via Legendre's formula |
| 0202_happy_number.rs | Happy number with Floyd cycle detection |
| 0204_count_primes.rs | Count primes below n with the Sieve of Eratosthenes |
| 0233_number_of_digit_one.rs | Count digit 1 in 0..=n by place-value analysis |
| 0263_ugly_number.rs | Check if only prime factors are 2, 3, 5 |
| 0264_ugly_number_ii.rs | n-th ugly number with three pointers |
| 0279_perfect_squares.rs | Fewest squares summing to n (Lagrange/Legendre theorems) |
| 0326_power_of_three.rs | Power-of-three test by dividing 3^19 |
| 0365_water_and_jug_problem.rs | Jug measuring via Bezout's identity |
| 0372_super_pow.rs | a^b mod 1337 with b as a digit array |
| 0509_fibonacci_matrix_exponentiation.rs | F(n) mod 1e9+7 for huge n by 2x2 matrix power |
| 0523_continuous_subarray_sum.rs | Subarray sum multiple of k via prefix residues |
| 1071_greatest_common_divisor_of_strings.rs | GCD of strings using length gcd |
| binomial_mod_prime.rs | nCr mod p with factorial tables and Lucas theorem |
| chinese_remainder_theorem.rs | Generalized CRT for non-coprime moduli |
| euler_totient_sieve.rs | Euler phi for a single n and as a sieve |
| extended_euclid_mod_inverse.rs | Extended Euclid and modular inverse |
| linear_sieve_smallest_prime_factor.rs | O(N) Euler sieve with smallest-prime-factor factorization |
| miller_rabin_primality.rs | Deterministic 64-bit Miller-Rabin primality test |
| pollard_rho_factorization.rs | 64-bit factorization with Pollard rho (Brent) |
| segmented_sieve.rs | Primes in a window [lo, hi] with large hi |
