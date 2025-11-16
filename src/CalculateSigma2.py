# sigma2_of_square.py
# Takes an integer n, squares it, and computes sigma_2(squared)
# sigma_2(m) = sum of d^2 for all positive divisors d of m

import math

def sigma_2_of_square(n: int) -> int:
    """
    Compute sigma_2(n^2) = sum_{d|n^2} d^2.
    Uses divisor generation from prime factorization for efficiency.
    """
    if n == 0:
        return 0  # define sigma_2(0) as 0 here
    n_abs = abs(n)
    # trial division prime factorization
    factors = {}
    x = n_abs
    # factor out 2s
    while x % 2 == 0:
        factors[2] = factors.get(2, 0) + 1
        x //= 2
    f = 3
    limit = int(math.isqrt(x)) + 1
    while f <= limit and x > 1:
        while x % f == 0:
            factors[f] = factors.get(f, 0) + 1
            x //= f
            limit = int(math.isqrt(x)) + 1
        f += 2
    if x > 1:
        factors[x] = factors.get(x, 0) + 1

    # For n^2, each prime exponent doubles: e -> 2e.
    # Formula for sigma_k for prime p with exponent a: (p^{k(a+1)} - 1) / (p^k - 1)
    # here k = 2, a = 2e
    result = 1
    for p, e in factors.items():
        a = 2 * e
        numerator = pow(p, 2 * (a + 1)) - 1  # p^{2(a+1)} - 1
        denominator = pow(p, 2) - 1         # p^2 - 1
        term = numerator // denominator
        result *= term
    return result

# Simple CLI usage
if __name__ == "__main__":
    import sys
    if len(sys.argv) >= 2:
        try:
            n = int(sys.argv[1])
        except ValueError:
            print("Please pass an integer.")
            sys.exit(1)
    else:
        n = int(input("Enter an integer: ").strip())

    squared = n * n
    sigma2 = sigma_2_of_square(n)
    print(f"n = {n}")
    print(f"n^2 = {squared}")
    print(f"sigma_2(n^2) = {sigma2}")