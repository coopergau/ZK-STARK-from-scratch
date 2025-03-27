// FRI: Fast Reed-Solomon Interactive Oracle Proof of Proximity
// Basically, low degree testing to show that the composition poly is a low degree polynomial.

/* Notes:
 - "Codewords" or "Reed-Soloman Codewords" refers to a the set of evaluations of a polynomial at the nth roots of unity of its domain.
 - FRI is a protocol between prover and verifier that establishes that a specific codeowrd belongs to a low degree polynomial.
    - low meaning at most ρ times the length of the codeword - what is ρ? (code's rate - reciprocal of expansion factor/blowup factor)
 - Prover knows the codeword
 - Verifier knows merkle root and leaf nodes of their choosing
*/


