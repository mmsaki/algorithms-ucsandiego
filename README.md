# Learning project

Run test for rust program:

```bash
cargo test
```

...in progress

# Algorithms: UCSanDiegoX ALGS200x

Algorithmic Design and Techniques

# Week 1: Prgramming Challenges

- [ ] Programming challenge 1.1: Sum of Two Digits
- [ ] Programming challenge 1.2: Maximum Pairwise Product

# Week 2: Algorithm Warm-up

- [x] Fibonacci Numbers
- [x] Programming challenge 2.1: Fibonacci Number

```python
# return the fibinacci number at n
# [0, 1, n-1+n-2]
# Easy recursion algorithm
def fib(n):
  if n < 2:
    return n
  else:
    return fib(n-1) + fib(n-2)
```

It starts to slow for large numbers because some recursions are being perfomed over and over again. See speed of fib(100)

- [x] Programming challenge 2.2: Last Digit of Fibonacci Number

```python
# better algorithm, we can just store values in an array and return the last index, no need to compute over and over again
def fib(n):
  res = [0,1]
  if n < 2:
    return res[n]
  else:
    for i in range(2, n+1):
      res.append(res[i-1] + res[i-2])
  return res[-1]
```

Much quicker algo. See difference in speed try fib(20000)

- [x] Programming challenge 2.3: Greatest Common Divisor

```python
# For gcd(a,b) we can just iterate through all number in a range
# a+b. Then update our best value of `d`
def gcd(a,b):
  d = 1
  for i in range(1, a+b):
    if a%i == 0 and b%i == 0:
      d = i
  return d
```

We can easily do `gcd(30023,400343)`, but once we get to numbers with larger values greater than 20 we run into a problem. We can fix it if know the fact that we can use the remainders of the two values to find equivalent gcds. gcd(6,20) == gcd(6,2) where 2 is the remainder from `20/6` or `20%6` or `pow(20, 1, 6)` or `divmod(20, 6)`,

```python
# better alternative is to use our remainder to reduce the number of steps needed to come to a solution
def gcd(a,b):
  if b == 0:
    return a
  else:
    return gcd(b, a%b)
```

Now we can compute larger numbers with faster run times `gcd(6003402534523452345023452345,3045200200002000200000345234653452340)` is $35$. Good for prime numbers and cryptography.

- [ ] Big-O Notation
- [ ] Programming challenge 2.4: Least Common Multiple
- [ ] Programming challenge 2.5: Fibonacci Number Again
- [ ] Programming challenge 2.6: Last Digit of the Sum of Fibonacci Numbers
- [ ] Programming challenge 2.7: Last Digit of the Sum of Fibonacci Numbers Again

# Week 3: Greedy Algorithms

- [ ] Programming challenge 3.1: Money Challenge
- [ ] Celebration Party
- [ ] Maximizing Loot
- [ ] Programming challenge 3.2: Maximum Value of the Loot
- [ ] Programming challenge 3.3: Maximum Advertisement Revenue
- [ ] Programming challenge 3.4: Collecting Signatures
- [ ] Programming challenge 3.5: Maximum Number of Prizes
- [ ] Programming challenge 3.5: Maximum Salary

# Week 4: Devide-and-Conquer

- [ ] Programming challenge 4.1: Collecting Signatures
- [ ] Polynomial Multiplication
- [ ] Master Theorem
- [ ] Programming challenge 4.2: Majority Element
- [ ] Sorting Problem
- [ ] Quick Sort
- [ ] Programming challenge 4.4: Number of Inversions
- [ ] Programming challenge 4.5: Organizing a Lottery
- [ ] Programming challenge 4.6: Closest Points

# Week 5: Dynamic Programming 1

- [ ] Challenge Problem
- [ ] Programming Challenge 5.1: Money Change Again
- [ ] Programming Challenge 5.2: Primitive Calculator
- [ ] String Comparison
- [ ] Programming Challenge 5.3: Edit Distance
- [ ] Programming Challenge 5.4: Longest Common Subsequence of Two Sequences
- [ ] Programming Challenge 5.5: Least Common Subsequence of Three Sequences

# Week 6: Dynamic Programming 2

- [ ] Knapsack
- [ ] Programming Challenge 6.1: Maximum Amount of Gold
- [ ] Programming Challenge 6.2: Partitioning Souvenirs
- [ ] Placing Parentheses
- [ ] Programming Challenge 6.3: Maximum Value of Arithmetic Expression

# Final Exam

- [ ] Preparing for Final Exam
- [ ] Practice Exam
- [ ] Final Exam
