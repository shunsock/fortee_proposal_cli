# fmt: format the code
# clippy: lint the code
# check: check the code
# test: test the code
# run: run the code
cargo watch \
  -x fmt \
  -x clippy \
  -x check \
  -x test \
  -x run
