# H-CX-738: Chromatic Polynomial -- Chromatic Number of K_P1

> **Hypothesis**: The chromatic polynomial chi(K_n, k) = k(k-1)...(k-n+1) = k^(n) falling factorial. The chromatic number of K_6 is chi(K_6) = 6 = P_1. The complete graph on P_1 vertices requires exactly P_1 colors.

## Grade: 🟧 SPECULATIVE

## Results

### The Formula

```
Chromatic polynomial of complete graph:
  chi(K_n, k) = k * (k-1) * (k-2) * ... * (k-n+1)
              = (k)_n  (falling factorial)

Chromatic number:
  chi(K_n) = n   (minimum colors for proper coloring)

For K_(P_1) = K_6:
  chi(K_6) = 6 = P_1
  chi(K_6, k) = k(k-1)(k-2)(k-3)(k-4)(k-5)
```

### n=6 Prediction

```
chi(K_(P_1)) = P_1 = 6

Number of proper k-colorings of K_6:
  chi(K_6, 7) = 7! / 1! = 5040          (k = M_3)
  chi(K_6, 12) = 12*11*10*9*8*7 = 665280  (k = sigma)
  chi(K_6, P_1) = 6! = 720 = P_1!       (k = P_1, minimum)
```

### Verification

```
Predicted:  chi(K_6) = P_1 = 6
Observed:   chi(K_6) = 6
Error:      0.00%
p-value:    ~0.30 (tautological: chi(K_n) = n by definition)
```

### Texas Sharpshooter Check

chi(K_n) = n is true for ALL complete graphs. Saying chi(K_6) = 6 is not a prediction but a tautology. The structural content must come from the polynomial itself or its evaluation at TECS-L constants.

### Deeper Content: Evaluations at TECS-L Points

```
chi(K_6, k) at TECS-L values:

  chi(K_6, P_1) = 6! = 720
    720 = sigma * P_1 * 10 = sigma * tau(P_3)
    720 = 6! = Gamma(M_3) (Gamma function at M_3)

  chi(K_6, M_3) = 7!/1! = 5040
    5040 = 7! = M_3!

  chi(K_6, sigma) = 12!/6! = 665,280
    = sigma! / P_1!

  chi(K_6, sigma_phi) = 24!/18! = huge

Pattern: chi(K_(P_1), k) = k! / (k - P_1)!
```

### P_2=28 Generalization

```
chi(K_(P_2)) = P_2 = 28 (tautological)

chi(K_28, 29) = 29! / 1! (number of 29-colorings of K_28)

P_2 generalization: EXTENDS trivially (chi(K_n) = n for all n)
```

## Verification

- [x] Chromatic number of K_6 = 6 is standard
- [x] chi(K_6, k) = k!/(k-6)! is correct
- [ ] Primary claim is tautological

## Status

New. The chromatic number chi(K_(P_1)) = P_1 is tautological, but evaluations of the chromatic polynomial at TECS-L constants yield factorial expressions: chi(K_6, 7) = 7! connects the Mersenne prime to the factorial.
