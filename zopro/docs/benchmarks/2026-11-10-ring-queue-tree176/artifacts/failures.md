# Preserved preparation/helper failures

1. First prepare execution wrote the deterministic model, then stopped before
   producing a plan or running tests/model: inherited historical pins expected
   old zopro-core/build.rs SHA aa65bee85dda3e5b1b2fc8bfe98a2f6b5e6975a7eff423baa3ec72f55cf920a2;
   actual accepted SHA is0729616c93c5f3cc14eec2126eeff561d73b59f3c6633d99fd37e6d4f59968fc.
   Repair keeps the already generated model byte-exact, inherits unchanged
   private/tool inputs and takes production hashes from the current176-file
   bitmap-reset adoption manifest. The failed preparer and generated model
   are preserved under before-source-pin-repair names. This does not change
   the recipe, test population, count rules or model observations.

2. First setup-review helper compilation fails E0382: a String path is moved
   into fs::read before the assignment borrows it. Borrowing &p fixes it.
   The failed source is retained. No reader or model ran in that failed compile.

These setup/helper commands have tool-transcript evidence, not complete xtask
receipts. All six scientific workflow commands succeed on their first run.
