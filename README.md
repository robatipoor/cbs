# cbs
[![Crates.io](https://img.shields.io/crates/v/cbs.svg?style=plastic)](http://crates.io/crates/cbs)
[![Build Status](https://travis-ci.org/robatipoor/cbs.svg?branch=master)](https://travis-ci.org/robatipoor/cbs)
[![Build status](https://ci.appveyor.com/api/projects/status/kr9iog6hyw3jfgqu/branch/master?svg=true)](https://ci.appveyor.com/project/robatipoor/cbs/branch/master)
[![License](https://img.shields.io/crates/l/cbs.svg)](https://crates.io/crates/cbs/)
### Work in progress
cbs is a command line utility that is designed to run on linux system , macOs and maybe windows. It provides an interface to manage clipboard from the command line

### Install cbs

```sh
# you need install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# and then install cbs
cargo install cbs
```

## How to use

- copy text to clipboard: `cbs copy "Text to be copied to clipboard"`
- copy text to primary: `cbs copy -s primary "Text to be copied to primary # only linux"`
- paste copied text from clipboard: `cbs paste`
- paste copied text from primary: `cbs paste -s primary # only linux`
- copy text to clipboard from stdin: `cat exmaple-file.txt | cbs`
- copy text to primary from stdin: `cat exmaple-file.txt | cbs -s primary # only linux`

## Usage

```sh
USAGE:
    cbs [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --select <selection>  to access clipboard or primary [default: clipboard]  [possible values: clipboard,primary]

SUBCOMMANDS:
    clear    clear text content
    copy     copy text content
    help     Prints this message or the help of the given subcommand(s)
    paste    paste text content
```