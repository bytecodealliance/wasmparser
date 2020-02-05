#/bin/bash

# record current bench results
cargo bench --bench benchmark -- --noplot --save-baseline after --sample-size 30

# switch to master and record its bench results
git checkout -f --recurse-submodules master && \
cargo bench --bench benchmark -- --noplot --save-baseline before --sample-size 30

# compare
cargo install critcmp --force && \
critcmp before after
