# Pull Request Template

## Description

Please include a summary of the change and which issue (if any) is fixed.
A brief description of the algorithm and your implementation method can be helpful too. If the implemented method/algorithm is not so
well-known, it would be helpful to add a link to an article explaining it with more details.

## Type of change

Please delete options that are not relevant.

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)

## Checklist:

- [ ] I ran bellow commands using the latest version of **rust nightly**.
- [ ] I ran `cargo clippy --all -- -D warnings` just before my last commit and fixed any issue that was found.
- [ ] I ran `cargo fmt` just before my last commit.
- [ ] I ran `cargo test` just before my last commit and all tests passed.
- [ ] I added my algorithm to the corresponding `mod.rs` file within its own folder, and in any parent folder(s).
- [ ] I added my algorithm to `DIRECTORY.md` with the correct link.
- [ ] I checked `COUNTRIBUTING.md` and my code follows its guidelines.

Please make sure that if there is a test that takes too long to run ( > 300ms), you `#[ignore]` that or
try to optimize your code or make the test easier to run. We have this rule because we have hundreds of
tests to run; If each one of them took 300ms, we would have to wait for a long time.
