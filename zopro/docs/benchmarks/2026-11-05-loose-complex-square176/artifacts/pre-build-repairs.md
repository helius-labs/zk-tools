# Preparation failure retained

The first direct rustc model compilation failed with E0382 before any model execution: the explanatory positive identity consumed BigUint `t`, which the independent REDC oracle later borrows. `model-initial.rs` preserves that source. The repair changes only `assert_eq!(t+...)` to `assert_eq!(&t+...)`; no arithmetic expression, case, bound or output gate changes. The failed direct compile is recorded in the tool transcript; its original stderr was not separately saved to a local stream file. The subsequent workflow receipts are byte-preserved.
