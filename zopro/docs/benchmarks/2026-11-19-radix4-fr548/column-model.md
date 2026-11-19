# Four-input inverse butterfly integer check

The model compares the radix-52 limb arithmetic with an independent modulo-p calculation. It checks every limb carry in 2,000 cases with operands drawn near zero, p and 2p or uniformly below 2p. It also compares complete 32-entry columns against five ordinary inverse decimation-in-frequency layers. The 64 columns use twelve power-of-two domain sizes, nontrivial inverse roots and varying column indices. All 2,048 outputs agreed. The exercise checks the field identities and the butterfly indices. Native instructions and full-proof timings have separate checks.

Run the check with `python3 artifacts/check_columns.py`. [Integer model](artifacts/check_columns.py) · [Result](artifacts/column-model.json)
