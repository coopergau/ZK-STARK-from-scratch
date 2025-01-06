# ZK STARK From Scratch

## Overview
Zero Knowledge STARK protocol for proving the knowledge of the pre image to a n output of the MiMC hash function.

## Steps
1. CI Statement:
The prover knows numbers I and O such that making I the input to the MiMC hash function gives O as output. I is known only to the prover, O is public. the MiMC hash function has these specific params:
- 127 rounds. (This means the trace will be 128 elements in the polynomial f and a power of 2 is good for the FFT.)
- The first round uses the user submitted number (I) as input starts as the first input.
- Every round adds the predetermined constant k to the input then cubes it to create the round output.
- The output from every round is used as the input for the next round until the last round is complete and the final output is returned.

2. CI constraints:
x_0 = I,
x_i = (x_{i-1} + k)^3 for integers i, 1 <= i <= 127
x_127 = O

3. Polynomial Constraints:
f: G -> F
F modulus (p) - 226156424291633194186662080095093570025917938800079226639565593765455339521 or (64 + 2^240)(128) + 1
F generator - 7
G order - 128
g (G generator) - 7^{(p-1)/128} mod p

f(g^0) = I,
f(g^i) = (f(g^{i-1}) + k)^3 for 1 <= i <= 127,
f(g^127) = O

These create three constraint polynomials:
1. f(x) - I = 0, for x = g
2. f(gx) - (f(x) + k)^3 = 0, for x = g^i, 0 <= i <= 127
3. f(x) - O = 0, for x = g^127

4. Reed-Soloman Coding
- Extend the evaluation domain of f to L so f:L->F.
|L| = 4096 because its decently big but much less than p.
So the generator of L is 7^{(p-1)/4096} mod p. This will create a superset of G that has 4096 elements.
Take the coefficient form of f and extend it over L to get the polynomial in evaluation form over L.

5. Constraint Polynomial
- Compute the constraint polynomials c:L->F

## What to do right now
- Get the proof working first and then add the zero knowledge part of f'(x) = f(x) + u(x)r(x) so the queries can be any point in L
- make poly division function
- add some tests for poly mul and div

## Proof generation steps
1. User submits I and O
2. Trace gets generated and f poly is interpolated
3. f evaluations get extended over domain L
4. Create constraint polynomials
5. Create composite polynomial from constraint polys
Then commitments and querying and stuff