# Conjecture on the Number of Representations of a Specific Type of Perfect Square

## Specification and Simplification of the Function f(n,k)

Consider the function f(n,k) defined by:

f(n,k) = n (n+k) (n+2k) (n+3k) + k<sup>4 (see
footnote[^1]</sup>)

This function is usually specified with k=1 (see first example below).
f(n,k) simplifies significantly by recognizing the product of the
arithmetic progression n, (n+k), (n+2k), (n+3k) can be written in terms
of a central square:

n(n+k)(n+2k)(n+3k) = (n<sup>2</sup> + 3nk + k<sup>2</sup>)<sup>2</sup> -
k<sup>4</sup>

Substituting this back into the definition of f(n,k), we obtain the
simplified form:

f(n,k) = (n<sup>2</sup> + 3nk + k<sup>2</sup>)<sup>2</sup>

Thus, f(n,k) always yields a perfect square. Also, noting the symmetry
in the simplified form, it is easy to see that f(n,k) = f(k,n). The
following conjecture reduces to analyzing the number of integer
representations of V<sup>2</sup>, where V = n<sup>2</sup> + 3nk +
k<sup>2</sup>, for a fixed case when n=k.

## The Conjectures

Let n<sub>0</sub> be a positive integer. n<sub>0</sub> = n = k in
f(n,k). We define the target value V\_{n<sub>0</sub>} as the value of
the function f(n<sub>0</sub>, n<sub>0</sub>):

V\_{n<sub>0</sub>} = f(n<sub>0</sub>, n<sub>0</sub>) = (5n<sub>0
</sub><sup>2</sup>)<sup>2</sup>

Let S\_{n<sub>0</sub>} be the set of all ordered pairs of positive
integers (n, k) such that f(n, k) = V\_{n<sub>0</sub>}.

S\_{n<sub>0</sub>} = { (n, k) ∊ ℤ<sup>+</sup>× ℤ<sup>+</sup> \|
n<sup>2</sup> + 3nk + k<sup>2</sup> = 5n<sub>0 </sub><sup>2 </sup>}

We denote O(n<sub>0</sub>) = \|S\_{n<sub>0</sub>}\| as the number of
such representations.

### Conjecture 1. (Odd Representation Count)

For every positive integer n<sub>0</sub>, the number of representations
O(n<sub>0</sub>) is always odd.

Proof: If there exists is any n,k pair, having n ≠ k<sub> </sub>such
that f(n,k) = V\_{n<sub>0</sub>}, then there is also a f(k,n) =
V\_{n<sub>0</sub>}. O(n<sub>0</sub>) is the count of all such
f(n,k) pairs plus 1. The plus 1 represents the one calculation where n =
k = n<sub>0 </sub>.

### Conjecture2. (Modular Equality)

For every positive integer n<sub>0</sub>, O(n<sub>0</sub>) mod 10 is
equal to σ<sub>2</sub>(n<sub>0</sub><sup>2</sup>) mod 10, where
σ<sub>2</sub>(m) is the sum of the squares of the divisors function.

O(n<sub>0</sub>) mod 10 = σ<sub>2</sub>(n<sub>0</sub><sup>2</sup>) mod
10

## Numerical Examples

### **1. Case **n**<sub>**0**</sub> = 1**

- Target Value: f(1,1) = V\_{1}* = *25 (5<sup>2</sup>).
- Solutions: (1 pair)  
  (**n=1, k=1**).
- Count O(1) = 1.
- σ<sub>2 </sub>(1<sup>2</sup>) = 1.
- Check: 1 mod 10 = 1 mod 10. ****(Holds)****

### **2. Case **n**<sub>**0**</sub> = 11**

- Target Value: f(11,11) = = V\_{11}* = *366,025 (605<sup>2</sup>).
- Solutions: (3 pairs)  
  (n=4, k=19),  
  (**n=11, k=11**),  
  (n=19, k=4).
- Count O(11) = 3.
- σ<sub>2</sub>(11<sup>2</sup>) = 14763.
- Check: 3 mod 10 = 14763 mod 10. ****(Holds)****

### ****3. Case ****n**<sub>**0**</sub>** = 121****

- Target Value: f(121, 121) = V\_{121}* *= 5,358,972,025
  (73205<sup>2</sup>).
- Solutions: (5 pairs)
- (n=29, k=229), (n=44, k=209),  
  (**n=121, k=121**),  
  (n=209, k=44), (n=229, k=29).
- Count O(121) = 5 (Odd).
- σ<sub>2</sub>(121<sup>2</sup>) = 216,145,205.
- Check: 5 mod 10 = 216,145,205 (mod 10). ****(Holds)****

### ****4. Case ****n**<sub>**0**</sub>** = 6061****

- Target Value: f(6061, 6061) = V\_{6061}* * = 33,737,829,934,746,025
  (183,678,605<sup>2</sup>).

- Solutions (27 pairs):

  (n=319, k=13079), (n=836, k=12331), (n=1039, k=12044),  
  (n=1681, k=11161), (n=2204, k=10469), (n=2731, k=9796),  
  (n=2831, k=9671), (n=3364, k=9019), (n=3389, k=8989),  
  (n=k=4031, k=8236), (n=4579, k=7619), (n=4796, 7381),  
  (n=5489, k=6644), (**n=6061, k=6061**), (n=6644, k=5489),  
  (n=7381, k=4796), (n=7619, k=4579), (n=8236, k=4031),  
  (n=8989, k=3389), (n=9019, k=3364), (n=9671, k=2831),  
  (n=9796, k=2731), (n=k=10469, 2204), (n=11161, k=1681),  
  (n=12044, k=1039), (n=12331, k=836), (n=13079, k=319).

- Count O(6061) = 27 (Odd).

- σ<sub>2</sub>(6061<sup>2</sup>) = 1,366,162,675,926,867.

- Check: 27 (mod 10) = 1,366,162,675,926,867 mod 10. ****(Holds)****

****I have run through 67500 tests with n=k and the conjecture has held.
But I understand that is not a proof.****

## ****Questions****

1.  ****For n = k = ****n**<sub>**0**</sub>** ,i****s there a method to
    solve for all ****n****, k ****where**** f(n,k) = V\_{
    n**<sub>**0**</sub>** }, ****i.e. S\_{ ****n**<sub>**0
    **</sub>**}****?****
2.  ****Is there a method to ****determine**** the number of solutions
    to f(n,k) = V\_{ n**<sub>**0**</sub>** }? ****i.e.
    O(****n**<sub>**0**</sub>**).****
3.  ****Can Conjecture 2 be ****proved****? Was conjecture 2 already
    known or a natural result of some other proved relation between
    *****σ***<sub>***2***</sub>**() and perfect squares ****formed via
    f(n,k)****?****

## Additional Supporting Documentation

****The following are sequences in the Online Encyclopedia****

****For n=1 to ∞****

- ****For difference=1: The square roots of the f(n,1) values are the
  sequence \#A028387. ****
- ****For difference=2: The square roots of the f(n,2) values are the
  positive values in sequence \#A028875****
- ****For difference=3: The square roots of the f(n,3) values are the
  positive values in sequence \#A190576****
- ****For difference=4: The square roots of the f(n,4) values are the
  positive values in sequence \#A134594****
- ****For difference=5 through difference=25, no sequences were found in
  the OEIS referencing the square roots.****

****There are 12 spreadsheets at in this repository that have 1,000,000+ f(n,k) 
combinations where there is a V\_{**n<sub>0</sub>** } among the results. These
sheets were generated by first running the Rust language program, main.rs, in the 
src folder. The program iterates over integer pairs n,k where n=k=n<sub>0</sub>. 
After generating f(n,k) a search is done for all n <> k pairs where 
f(n,k) = f(n<sub>0</sub>,n<sub>0</sub>) generates the same V\_(n<sub>0</sub> value.
The database key fields contain not the f(n0) values but the square root of
f(n<sub>0</sub>). For entries where f(n,k). As shown in the proof of conjecture 1 above, 
the count of f(n,k) = f(n<sub>0</sub>,n<sub>0</sub>) values will be odd.

[^1]:
    > *When k = 1 the sequence for n = 1,2,3,… is outlined in the Online
    > Encyclopedia of Integer Sequences; article \#062938.
    > (https://oeis.org/A062938)*
