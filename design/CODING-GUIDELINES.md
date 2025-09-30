# AI Rust Coding Standards

- Prefer existing crates over custom implementations, especially for parsing.
- Structure the code so that each class has one corresponding file.
- Prefer a functional approach over imperative programming.

## Nix Shell

Please create a `shell.nix` file that:

- Has all necessary dependencies
- Loads the `./rust-toolchain.toml` as an override and uses the version specified there.
