# Integration Tests

Integration tests are very easy to write with cw-orch. Start by creating a `tests` folder in your contract's dir.

```bash
mkdir counter/tests
```

Then create a file called `integration_tests.rs` in the `tests` folder.

```bash
touch counter/tests/integration_tests.rs
```

Now we can write our tests. Here's an example of a test that deploys the contract, increments the counter and then resets it.

```rust,ignore
{{#include ../../../contracts/counter/tests/integration_tests.rs:all}}
```
