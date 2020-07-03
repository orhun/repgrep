# `repgrep`: An interactive replacer for `ripgrep`.

This is an interactive command line tool to make find and replacement easy.
It uses [`ripgrep`] to find, and then provides you with a simple interface to see
the replacements in real-time and conditionally replace matches.

Right now this only works with ascii or UTF-8 encoded files, other common encodings (such as UTF-16{le,be} are [planned for the future](https://github.com/acheronfail/repgrep/issues/12)).

## Usage

After installing, just use `rgr` (think: `rg` + `replace`).

The arguments are:

```bash
rgr <rg arguments> # See `rgr --help` for more details
```

![demo using rgr](./doc/demo.gif)

## Installation

First and foremost, make sure you've installed `ripgrep` (AKA: `rg`).
To do so see the [`ripgrep` installation instructions].

#### Precompiled binaries

See the [releases] page for pre-compiled binaries.

#### Via Cargo

**NOTE**: The minimum Rust version required is `1.42.0`.

```bash
cargo install repgrep
```

#### From Source (via Cargo)

**NOTE**: The minimum Rust version required is `1.42.0`.

```bash
git clone https://github.com/acheronfail/repgrep/
cd repgrep
cargo install --path .
```

[`ripgrep`]: https://github.com/BurntSushi/ripgrep
[releases]: https://github.com/acheronfail/repgrep/releases
[`ripgrep` installation instructions]: https://github.com/BurntSushi/ripgrep/#installation
