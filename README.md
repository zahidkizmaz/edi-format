# EDIFACT Formatter

[![License](https://img.shields.io/github/license/zahidkizmaz/edi-format.svg)](https://github.com/zahidkizmaz/edi-format/blob/main/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/edi-format?style=flat&link=https%3A%2F%2Fcrates.io%2Fcrates%2Fedi-format)](https://crates.io/crates/edi-format)

Simple EDI file formatter.

![Demo](demo.gif)

### Installation

#### Cargo

```sh
cargo install edi-format
```

#### Shell

##### Unix

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/zahidkizmaz/edi-format/releases/latest/download/edi-format-installer.sh | sh
```

##### Powershell

```sh
powersh -c "irm https://github.com/zahidkizmaz/edi-format/releases/latest/download/edi-format-installer.ps1 | iex"
```

#### Nix Flakes

Example nix flake with dev shell:

```nix
{
  description = "Simple development shell flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    edi-format.url = "github:zahidkizmaz/edi-format";
  };
  outputs = { self, nixpkgs, flake-utils, edi-format }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = [
          edi-format.packages.${system}.edi-format
        ];
      };
    }
  );
}

```

OR just simply run:

```sh
nix run github:zahidkizmaz/edi-format -- --help
```

#### Building from source

[Install rust and cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you don't have.

```sh
git clone https://github.com/zahidkizmaz/edi-format.git
cd edi-format
cargo build --release
./target/release/edi-format --version
edi-format 0.1.0
```

### Usage

```
Usage: edi-format [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to format

Options:
      --dry-run                Do not modify the file but show formatted content in stdout
  -l, --log-level <LOG_LEVEL>  Log level eg: trace, debug, info, warn, error [default: INFO]
  -h, --help                   Print help
  -V, --version                Print version
```

#### Example Usages

##### Format a file:

```sh
$ cat tests/valid_not_formatted.edi
UNA:+.? 'UNB+IATB:1+6XPPC:ZZ+LHPPC:ZZ+940101:0950+1'UNH+1+PAORES:93:1:IA'MSG+1:45'IFT+3+XYZCOMPANY AVAILABILITY'ERC+A7V:1:AMD'IFT+3+NO MORE FLIGHTS'ODI'TVL+240493:1000::1220+FRA+JFK+DL+400+C'PDI++C:3+Y::3+F::1'APD+74C:0:::6++++++6X'TVL+240493:1740::2030+JFK+MIA+DL+081+C'PDI++C:4'APD+EM2:0:1630::6+++++++DA'UNT+13+1'UNZ+1+1'

$ edi-format --dry-run tests/valid_not_formatted.edi
2024-04-27T06:26:09.441402Z INFO edi_format: Running in dry-run mode
UNA:+.? '
UNB+IATB:1+6XPPC:ZZ+LHPPC:ZZ+940101:0950+1'
UNH+1+PAORES:93:1:IA'
MSG+1:45'
IFT+3+XYZCOMPANY AVAILABILITY'
ERC+A7V:1:AMD'
IFT+3+NO MORE FLIGHTS'
ODI'
TVL+240493:1000::1220+FRA+JFK+DL+400+C'
PDI++C:3+Y::3+F::1'
APD+74C:0:::6++++++6X'
TVL+240493:1740::2030+JFK+MIA+DL+081+C'
PDI++C:4'
APD+EM2:0:1630::6+++++++DA'
UNT+13+1'
UNZ+1+1'%
```

##### Format stdin:

```sh
$ cat ./tests/valid_formatted.edi | edi-format --stdin
UNA:+.? '
UNB+IATB:1+6XPPC:ZZ+LHPPC:ZZ+940101:0950+1'
UNH+1+PAORES:93:1:IA'
MSG+1:45'
IFT+3+XYZCOMPANY AVAILABILITY'
ERC+A7V:1:AMD'
IFT+3+NO MORE FLIGHTS'
ODI'
TVL+240493:1000::1220+FRA+JFK+DL+400+C'
PDI++C:3+Y::3+F::1'
APD+74C:0:::6++++++6X'
TVL+240493:1740::2030+JFK+MIA+DL+081+C'
PDI++C:4'
APD+EM2:0:1630::6+++++++DA'
UNT+13+1'
UNZ+1+1'%
```

##### Format in Neovim

**Neovim: add `edi` filetype**

```lua
vim.filetype.add({
  extension = { edi = "edi" },
})
```

###### conform-nvim

**Custom formatter in conform-nvim conform-nvim**

```lua
  formatters = {
    edi_format = {
      command = "edi-format",
      args = { "-l", "error", "--stdin" },
    },
  },
```

###### efm-langserver

**Example efm-langserver config**

```yaml
version: 2
tools:
  edi-format: &edi-format
    format-stdin: true
    format-command: "edi-format -l error --stdin"
languages:
  edi:
    - <<: *edi-format
```

##### Reference

https://en.wikipedia.org/wiki/EDIFACT
