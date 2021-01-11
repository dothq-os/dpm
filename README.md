<div align="center">

# dpm 📦

![Rust](https://github.com/dothq-os/dpm/workflows/Rust/badge.svg)

The fast, compatible package manager for dotos.

</div>

## Roadmap

This project is just getting started. If you want to see what the status is please see our [roadmap](https://github.com/dothq-os/dpm/projects/1).

## Development

You need to have the folowing tools installed to develop for this project:

- [Rust](https://www.rust-lang.org/tools/install)
- Linux ([Manjaro](https://manjaro.org/) or [ubuntu latest lts](https://ubuntu.com/download/desktop))
- `binutils`, `tar` and `rsync`

Then you can clone the repo:

```sh
git clone https://github.com/dothq-os/dpm

# or with the github cli

gh repo clone dothq-os/dpm
```

You can then use the standard rust tooling for development:

```sh
cargo build
cargo run
```

**Note:** It is not recommended to test dpm on your primary machine as it may overwrite os file. It is recommend to use a vm where possible.
