<div align="center">

# rip unburied

### A safer, rust-based `rm`

[![crates](https://img.shields.io/crates/v/rip_unburied.svg)](https://crates.io/crates/rip_unburied)
[![CI](https://github.com/PurpleCraft999/rip-unburied/actions/workflows/ci.yml/badge.svg)](https://github.com/PurpleCraft999/rip-unburied/actions/workflows/ci.yml)
<!-- [![codecov](https://codecov.io/gh/MilesCranmer/rip2/graph/badge.svg?token=1Ezb7PjJ0Z)](https://codecov.io/gh/MilesCranmer/rip2) -->

</div>

`rip` is a rust-based `rm` with a focus on safety, ergonomics, and performance.  It favors a simple interface, and does *not* implement the xdg-trash spec or attempt to achieve the same goals.

Deleted files get sent to the graveyard 🪦 (typically `/tmp/graveyard-$USER`, see [notes](#notes) on changing this) under their absolute path, giving you a chance to recover them 🧟. No data is overwritten. If files that share the same path are deleted, they will be renamed as numbered backups.

rip unburied is a fork of [MilesCranmer/rip2](https://github.com/MilesCranmer/rip2) with miniscule improvements:

- **2024 rust**
- **like two new features**

## Quick start

Install:


Delete files and directories (no `-rf` needed):

```bash
rip file.txt dir1 dir2
```

Undo the last deletion:

```bash
rip -u
```

More details below.

## ⚰️ Installation

This package is supported on Linux, macOS, and Windows.

### Cargo

1. First [install Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html).
2. Then, install this package with cargo:

```bash
cargo install --locked rip_unburied
```

### Binaries

Binary releases for different architectures and operating systems are
made available on the GitHub releases page: https://github.com/PurpleCraft999/rip-unburied/releases

To install, simply open the archive and move the binary somewhere you can run it.

### Nix
**currently broken**

This repository is also flake-compatible, and backwards-compatible with non-flake systems. Just run the following to test it out:

```bash
nix develop "github:PurpleCraft999/rip-unburied"
```

### Other

<details><summary>A few other package repositories have contributed support:</summary>


### Additional Nix options

The repo uses `flake-compat` for compatibility, and `naersk` to build the Rust package from source.

<details><summary>Details:</summary>

**Add To Path Temporarily (With Flakes)**:

```bash
nix shell "github:PurpleCraft999/rip-unburied"
```

**Flake minimal setup**:

```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rip_unburied = {
      url = "github:PurpleCraft999/rip-unburied";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, rip_unburied, ... }:
  {
    nixosConfigurations.your-host = let
      system = "x86_64-linux";  # or your system
      lib = nixpkgs.lib;
    in lib.nixosSystem {
      inherit system;
      modules = [
        ./configuration.nix # or other configuration options
        # ...
        {
          environment.systemPackages = [
            rip_unburied.packages.${system}.default
          ];
        }
      ];
    };
  };
}
```
</details>


### openSUSE

```
zypper ar -f obs://utilities
zypper in rip2
```

</details>

## Usage

```text
Usage: rip [OPTIONS] [FILES]...
       rip [SUBCOMMAND]

Arguments:
    [FILES]...  Files and directories to remove

Options:
      --graveyard <GRAVEYARD>  Directory where deleted files rest
  -d, --decompose              Permanently deletes the graveyard
  -s, --seance                 Prints files that were deleted in the current directory
  -u, --unbury                 Restore the specified files or the last file if none are specified
  -i, --inspect                Print some info about FILES before burying
  -f, --force                  Non-interactive mode
  -h, --help                   Print help
  -V, --version                Print version
  -v, --verbose                Prints the files being moved to graveyard as it goes       
  
Sub-commands:
  completions  Generate shell completions file
  graveyard    Print the graveyard path
  help         Print this message or the help of the given subcommand(s)
```

Basic usage -- easier than rm

```bash
rip dir1/ file1
```

Undo the last deletion

```bash
rip -u
# Returned /tmp/graveyard-jack/home/jack/file1 to /home/jack/file1
```

Print some info (size and first few lines in a file, total size and first few files in a directory) about the target and then prompt for deletion

```bash
rip -i file1
# dir1: file, 1337 bytes including:
# > Position: Shooting Guard and Small Forward ▪ Shoots: Right
# > 6-6, 185lb (198cm, 83kg)
# Send file1 to the graveyard? (y/n) y
```

Print files that were deleted from under the current directory

```bash
rip -s
# /tmp/graveyard-jack/home/jack/file1
# /tmp/graveyard-jack/home/jack/dir1
```

Name conflicts are resolved

```bash
touch file1
rip file1
rip -s
# /tmp/graveyard-jack/home/jack/dir1
# /tmp/graveyard-jack/home/jack/file1
# /tmp/graveyard-jack/home/jack/file1~1
```

-u also takes the path of a file in the graveyard

```bash
rip -u /tmp/graveyard-jack/home/jack/file1
# Returned /tmp/graveyard-jack/home/jack/file1 to /home/jack/file1
```

-u also can expand to the current location
```bash
rip -u ../file1
# Returned /tmp/graveyard-jack/home/jack/file1 to /home/jack/file1
```

Combine -u and -s to restore everything printed by -s

```bash
rip -su
# Returned /tmp/graveyard-jack/home/jack/dir1 to /home/jack/dir1
# Returned /tmp/graveyard-jack/home/jack/file1~1 to /home/jack/file1~1
```

## Notes

**Aliases.**

You probably shouldn't alias `rm` to `rip`.  Unlearning muscle memory is hard, but it's harder to ensure that every `rm` you make (as different users, from different machines and application environments) is the aliased one.

What I instead recommend is aliasing `rm` to an echo statement that simply reminds you to use `rip`:

```bash
alias rm="echo Use 'rip' instead of rm."
```

**Graveyard location.**

You can see the current graveyard location by running `rip graveyard`.
If you have `$XDG_DATA_HOME` environment variable set, `rip` will use `$XDG_DATA_HOME/graveyard` instead of the `$TMPDIR/graveyard-$USER`.

If you want to put the graveyard somewhere else (like `~/.local/share/Trash`), you have two options, in order of precedence:

  1. Alias `rip` to `rip --graveyard ~/.local/share/Trash`
  2. Set the environment variable `$RIP_GRAVEYARD` to `~/.local/share/Trash`.

This can be a good idea because if the graveyard is mounted on an in-memory file system (as `/tmp` is in Arch Linux), deleting large files can quickly fill up your RAM. It's also much slower to move files across file systems, although the delay should be minimal with an SSD.

**Force mode.**

The `-f --force` flag enables non-interactive mode, which skips most prompts and automatically uses safe and reasonable behavior:

- Big files are copied to the graveyard without prompting
- Files already in the graveyard are permanently deleted without prompting
- Special, non-movable files will error

**Miscellaneous.**

In general, a deletion followed by a `--unbury` should be idempotent.

The deletion log is kept in `.record`, found in the top level of the graveyard.