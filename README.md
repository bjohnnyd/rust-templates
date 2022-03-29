# My Rust Templates

My collection of rust crate templates to easily start a project.

Requires [cargo-generate][gen-link]:

```sh
cargo install cargo-generate
```

to create a rust project with a specific template:

```sh
cargo generate --git bjohnnyd/rust-templates <TEMPLATE_DIR>
```

for example:

```sh
cargo generate --git bjohnnyd/rust-templates wasm
```
to generate into the current directory:

```sh
cargo generate --init --git bjohnnyd/rust-templates wasm
```

more cargo generate information can be found in the [docs][gen-docs].

[gen-link]: https://github.com/cargo-generate/cargo-generate
[gen-docs]: https://cargo-generate.github.io/cargo-generate/index.html
