<div align="center">
<h1>macchina</h1>

Fast, minimal and customizable system information frontend.

Linux • macOS • Windows • NetBSD • FreeBSD • OpenWrt • Android

<a href="https://crates.io/crates/macchina">
    <img src="https://img.shields.io/crates/v/macchina?label=version" alt="Version" />
</a>

<a href="https://crates.io/crates/macchina">
    <img src="https://img.shields.io/crates/d/macchina?label=downloads" alt="Downloads" />
</a>

<a href="https://github.com/Macchina-CLI/macchina/actions">
   <img src="https://img.shields.io/github/checks-status/Macchina-CLI/macchina/main" alt="CLI">
</a>

<img src="assets/preview.png" alt="Preview" />

</div>

# Table of Contents

- [About](#about)
- [Benchmarks](#benchmarks)
- [Features](#features)
- [Configuration](#configuration)
- [Customization](#customization)
- [Dependencies](#dependencies)
- [Installation](#installation)
- [Packaging Status](#packaging-status)
- [Contributors](#contributors)
- [ASCII Art](#ascii-art)

# About

_macchina_ lets you view system information, like your kernel version, uptime,
memory usage, processor load and much more. _macchina_ is **basic** by default
and **extensible** by design.

If you're interested in the library _macchina_ uses to fetch system
information, have a look at
[libmacchina]; fetching-related
issues should be filed on that repository.

# Benchmarks <a name="benchmarks"></a>

> Execution time is measured using [hyperfine](https://github.com/sharkdp/hyperfine)

_macchina_ is **lightning fast**, see for yourself:

## Linux

This benchmark was performed using an **Intel® Core™ i5-3350P CPU @ 3.10GHz**

| Command    | Mean [ms] | Min [ms] | Max [ms] |
| :--------- | --------: | -------: | -------: |
| `macchina` | 4.8 ± 0.4 |      4.5 |      8.1 |

## macOS

| Command    | Mean [ms] | Min [ms] | Max [ms] |
| :--------- | --------: | -------: | -------: |
| `macchina` | 5.0 ± 0.2 |      4.7 |      5.9 |

## NetBSD

This benchmark was performed inside a virtual machine using an **Intel® Core™
i5-8265U CPU @ 1.60GHz**

| Command    |  Mean [ms] | Min [ms] | Max [ms] |
| :--------- | ---------: | -------: | -------: |
| `macchina` | 17.7 ± 1.6 |     16.8 |     27.2 |

## Windows

| Command    | Mean [ms] | Min [ms] | Max [ms] |
| :--------- | --------: | -------: | -------: |
| `macchina` | 9.0 ± 0.4 |      8.2 |     11.7 |

# Features

## Themes

_macchina_ has a theming system which you can use to customize pretty much any
visual aspect of the program. Themes live **outside** the configuration file,
so you can create a bunch of them and switch between them at any time.

Why are they separate?

- **Modularity** — themes are an engine of their own, and their sole purpose is
  customizing the look of the program, so it makes sense to separate them from
  the configuration file.
- **Portability** — sure, the configuration file is shareable, but what if you
  wanted to share the look of _your macchina_ and not its behavior? What if you
  wanted to switch between dozens of themes that you very carefully designed? The
  way we handle customization answers this need.

Learn how to [make your own](#customization), and maybe have a look at the
[documentation](https://github.com/Macchina-CLI/macchina/wiki/Theme-Documentation)
while you're at it.

## Doctor

[libmacchina] can sometimes fail
to fetch certain readouts, and _macchina_ has a feature in place that describes
why they failed. You don't have to guess what went wrong, `--doctor` has your
back!

# Configuration

The configuration file defines the behavior of macchina, it does not allow for
much customization. See [macchina.toml](macchina.toml) for an example
configuration file.

In order for _macchina_ to be able to read the configuration file, you need to
place `macchina.toml` inside:

- `$XDG_CONFIG_HOME/macchina` on Linux and BSDs.
- `$HOME/.config/macchina` or `$HOME/Library/Application Support/macchina` on macOS.
- `{FOLDERID_RoamingAppData}/macchina` on Windows.

# Customization

Themes define the look, layout and styling of _macchina_. See
[Hydrogen.toml](themes/Hydrogen.toml) for an example theme.

In order for _macchina_ to be able to find your themes, you need to place them
in:

- `$XDG_CONFIG_HOME/macchina/themes` on Linux and BSDs.
- `$HOME/.config/macchina/themes` or `$HOME/Library/Application Support/macchina/themes` on macOS.
- `{FOLDERID_RoamingAppData}/macchina/themes` on Windows.

To start using your theme:

1. Run `macchina --list-themes` to verify that macchina has listed your theme.

2. If all goes well, you should see one or more themes listed, e.g.:

```
<path_where_themes_were_fount>:
- Helium
- Hydrogen
```

3. In [macchina.toml](macchina.toml), set `theme` to one of the listed themes,
   e.g.:

```bash
theme = "Helium" # case-sensitive
```

4. You're good to go! _macchina_ will start using your theme.

# Dependencies

### Build dependencies

The following are required dependencies if you're building from source or installing through `cargo`:

- _libgit2_

### Runtime dependencies

The following are _optional_ dependencies, they only extend what macchina can fetch:

**Linux**:

- _wmctrl_

**NetBSD**:

- _wmctrl_

# Installation

### Cargo

```
cargo install macchina
```

### Pkgsrc

```
pkgin install macchina
```

### openSUSE Tumbleweed

```
zypper in macchina
```

### Homebrew

```
brew install macchina
```

### Termux

```
pkg install macchina
```

### Windows

```
scoop install macchina
```

### Nix

- Where `<channel>` is `nixpkgs` or `nixos`:

```bash
nix-env -iA <channel>.macchina
```

### Arch Linux

- _Option 1:_ Use the AUR package that pulls and installs the latest release:

```bash
git clone https://aur.archlinux.org/macchina.git
cd macchina
makepkg -si
```

- _Option 2:_ Use the AUR package that pulls and installs from upstream:

```bash
git clone https://aur.archlinux.org/macchina-git.git
cd macchina
makepkg -si
```

The [installation wiki
page](https://github.com/grtcdr/macchina/wiki/Installation) lists some other
ways you can install the program or the steps to compile from source.

You might prefer running the [prebuilt
binary](https://github.com/grtcdr/macchina/releases) that corresponds with your
operating system.

# Packaging Status

[![Packaging status](https://repology.org/badge/vertical-allrepos/macchina.svg)](https://repology.org/project/macchina/versions)

# Contributors

[![Crates.io](https://contrib.rocks/image?repo=grtcdr/macchina)](https://github.com/grtcdr/macchina/graphs/contributors)

_macchina_, like many other open-source projects, would not be where it is
right now without the help of its contributors — Thank you all so much!

Looking to help? [Read this first.](CONTRIBUTING.md)

The following are some awesome people that have helped make _macchina_ a really
cool fetcher:

- Support for **NetBSD** would not have been possible without the help and
  testing of NetBSD/pkgsrc package maintainer
  [pin](https://pkgsrc.se/bbmaint.php?maint=pin@NetBSD.org)
- Support for **macOS** and **Windows** would not have been possible without
  the help, testing and major contributions of
  [123marvin123](https://github.com/123marvin123)
- Support for **OpenWrt** and **Android** was made possible through the many
  contributions of [uttarayan21](https://github.com/uttarayan21)

# ASCII Art

Some of the ASCII art displayed in macchina is **not** our own.

If you (the artist) are not okay with us using your ASCII art, please [contact
me](mailto:ba.tahaaziz@gmail.com).

- FreeBSD ASCII art (small variant) was taken from Dylan Araps'
  [pfetch](https://github.com/dylanaraps/pfetch)
- macOS ASCII art (big variant) was taken from Dylan Araps'
  [Neofetch](https://github.com/dylanaraps/neofetch)
- macOS ASCII art (small variant) was made by Joan Stark (jgs)
- Linux ASCII art (big variant) was made by Joan Stark (jgs)
- Linux ASCII art (small variant) was taken from Christopher Johnson's ASCII
  art collection (unknown artist)

[libmacchina]: https://github.com/Macchina-CLI/libmacchina
