# {{project-name}}

Implementation of the tutorial for the [Rust Wasm Book](https://rustwasm.github.io/docs/book/introduction.html).

Additional details at [rust-wasm](https://rustwasm.github.io)

**Initial Setup Requires**:

- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer)
- [cargo generate](https://github.com/cargo-generate/cargo-generate)
- [npm](https://docs.npmjs.com/getting-started)

**Test by Building, Installing and Serving**:

To serve project run:

`make start`

this might result in:

<span style="color: red">error:0308010C:digital envelope routines::unsupported</span>

in which case change to the latest LTS node version (e.g. `v16.13.1`):

```sh
nvm install 16.13.1
nvm use 16.13.1
```

and navigate to [http://127.1:8080](http://127.1:8080).
