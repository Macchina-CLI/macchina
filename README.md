<div align="center">
<h1>Macchina</h1>

Fast, minimal and customizable system information frontend.

Linux • macOS • Windows • NetBSD • OpenWrt • Android

<img src="screenshots/preview.png" alt="Preview" />

<a href="https://crates.io/crates/macchina">
    <img src="https://img.shields.io/crates/v/macchina?label=Version" alt="Version" />
</a>

<a href="https://crates.io/crates/macchina">
    <img src="https://img.shields.io/crates/d/macchina?label=Downloads" alt="Downloads" />
</a>

</div>

---

# Table of Contents

- [Upgrading to v2.0](#upgrading)
- [About](#about)
- [Performance](#performance)
- [Features](#features)
- [Configuration](#configuration)
- [Runtime dependencies](#runtime-dependencies)
- [Installation](#installation)
- [Platform Support](#platform-support)
- [Contributors](#contributors)

---

# Upgrading to v2.0

The second major version of macchina includes a breaking change in _macchina's_
configuration file that could result in an error if you don't update your
configuration accordingly.

### `<2.0`

macchina.toml:

```toml
palette = "<True|False>"
```

### `>=2.0`

macchina.toml:

```toml
# this is optional, leaving it out/commenting it hides the palette
palette = "<Dark|Light|Full>"
```

### Thanks to

- [FantasyTeddy](https://github.com/FantasyTeddy) for the new customization option; _macchina_ can now show your
  dark, light or all the color variants that your colorscheme is set to
  display.

---

# About

_macchina_ lets you view system information, like your kernel version, uptime,
memory usage, processor load and much more.

If you're interested in the library _macchina_ uses to
fetch system information, have a look at
[libmacchina](https://github.com/Macchina-CLI/libmacchina);
fetching-related issues should be filed on that repository.

---

# Performance

_macchina_ is lightning fast, see for yourself:

- Execution time is measured using
  [hyperfine](https://github.com/sharkdp/hyperfine)

## 🐧 Linux

This benchmark was performed using an __Intel® Core™ i5-3350P CPU @ 3.10GHz__

| Command    | Mean [ms] | Min [ms] | Max [ms] |
|:---        | --------: | --------:| --------:|
| `macchina` | 4.8 ± 0.4 |      4.5 |      8.1 |

## 🍏 macOS

| Command    | Mean [ms] | Min [ms] | Max [ms] |
| :--------- | --------: | -------: | -------: |
| `macchina` | 5.0 ± 0.2 |      4.7 |      5.9 |

## 🚩 NetBSD

This benchmark was performed inside a virtual machine using an __Intel® Core™
i5-8265U CPU @ 1.60GHz__

| Command    |  Mean [ms] | Min [ms] | Max [ms] |
| :--------- | ---------: | -------: | -------: |
| `macchina` | 17.7 ± 1.6 |     16.8 |     27.2 |

## 💻 Windows

| Command    | Mean [ms] | Min [ms] | Max [ms] |
| :--------- | --------: | -------: | -------: |
| `macchina` | 9.0 ± 0.4 |      8.2 |     11.7 |

---

# Features

## Themes

_macchina_ comes equipped with built-in themes that style their readouts, bars
and separators differently. No extra configuration required.

## Bars

Bars provide a way to visualize data, and each theme styles them differently.
They can be enabled using the `--bar` flag.

<div align="center">
   <img src="screenshots/bars.png" alt="Preview of the bar flag" />
</div>

## Doctor

_libmacchina_ can sometimes fail to fetch certain readouts, and _macchina_ has
a feature in place that allows you to analyze why they failed, this is done
through the `--doctor` flag.

<div align="center">
   <img src="screenshots/doctor.png" alt="Preview of the doctor flag" />
</div>

---

# Configuration

See
[macchina.toml](https://github.com/Macchina-CLI/macchina/blob/main/macchina.toml)
for an example configuration file.

- In order for _macchina_ to be able to read the configuration file, you need
  to place `macchina.toml` in `$XDG_CONFIG_HOME/macchina/`

You can also create custom themes in `JSON` format, themes allow for more
customization and are separate from the main configuration file.  See
[Carbon.json](https://github.com/Macchina-CLI/macchina/blob/main/theme/Carbon.json)
for an example theme.

- In order for _macchina_ to be able to read your custom themes, you need to
  place them in `$XDG_DATA_HOME/macchina/themes/`. You can have as many as you
  want, just avoid using the names of built-in themes.

To start using your theme: 
1. Run `macchina --list-themes` to verify that macchina has listed your theme
2. Inside `macchina.toml`, add `theme = <name_of_theme_without_json_extension>`
3. You're good to go! _macchina_ will start using your theme.

---

# Runtime dependencies

These dependencies are __not__ required, but they extend what _macchina_ can show.

### Linux:

- _wmctrl_
- Gentoo: _portage-utils_

### NetBSD:

- _wmctrl_

---

# Installation

### Cargo
```
cargo install macchina
```

### Pkgsrc
```
pkgin install macchina
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

### Arch Linux
- Use the AUR package that pulls and installs the latest release: 
```bash
git clone https://aur.archlinux.org/macchina.git
cd macchina
makepkg -si
```

- Or use the AUR package that pulls and builds from upstream:
```bash
git clone https://aur.archlinux.org/macchina-git.git
cd macchina
makepkg -si
```

### Nix
- Where `<channel>` is `nixpkgs` or `nixos`:
```bash
nix-env -iA <channel>.macchina
```

The [installation wiki
page](https://github.com/grtcdr/macchina/wiki/Installation) lists some other
ways you can install the program or the steps to compile from source. 

You might prefer running the [prebuilt
binary](https://github.com/grtcdr/macchina/releases) that corresponds with your
operating system.

---

# Platform Support

| Platform  | Support |
| :-------: | :-----: |
|   Linux   |    ✓    |
|   NetBSD  |    ✓    |
|   macOS   |    ✓    |
|  Windows  |   80%   |
|  OpenWrt  |    ✓    |
|  Android  |    ✓    |

# Contributors

[![Crates.io](https://contrib.rocks/image?repo=grtcdr/macchina)](https://github.com/grtcdr/macchina/graphs/contributors)

_macchina_, like many other open-source projects, would not be where it is
right now without the help of its contributors, thank you all so much!

- Support for **NetBSD** would not have been possible without the help and
  testing of NetBSD/pkgsrc package maintainer
  [pin](https://pkgsrc.se/bbmaint.php?maint=pin@NetBSD.org)
- Support for **macOS** and **Windows** would not have been possible without
  the help, testing and major contributions of
  [123marvin123](https://github.com/123marvin123)
- Support for **OpenWrt** and **Android** was made possible through the many
  contributions of [uttarayan21](https://github.com/uttarayan21)
