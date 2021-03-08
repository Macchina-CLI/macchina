![Macchina preview image](screenshots/banner.png)

[![Crates.io](https://img.shields.io/crates/v/macchina?style=for-the-badge&label=VERSION&color=0D3B66)](https://crates.io/crates/macchina)
[![Crates.io](https://img.shields.io/crates/d/macchina?style=for-the-badge&label=DOWNLOADS&color=0D3B66)](https://crates.io/crates/macchina)
![reposize](https://img.shields.io/github/repo-size/grtcdr/macchina?color=0D3B66&logo=github&style=for-the-badge)
![loc](https://img.shields.io/tokei/lines/github/grtcdr/macchina?color=0D3B66&label=Lines%20of%20Code&logo=rust&style=for-the-badge)

## Table of Contents:
- [About](#about)
- [Changelog](#change)
- [Todo](#todo)
- [Dependencies](#deps)
- [Benchmarks](#bench)
- [Features](#features)
- [Installation](#install)
- [Platform Support](#platform-support)

---

# About Macchina <a name="about"></a>

Macchina lets you view basic system information, like your hostname, kernel, uptime, memory usage, and much more.
It provides you with convenient features and customization options but doesn't lose sight of its two main priorities: minimalism and performance.

![Macchina preview image](screenshots/preview.png)

# Changelog <a name="change"></a>
This version (v0.5.2) does not bring any new changes to Macchina:
- Update README benchmarks
- Remove a repetition in README_CARGO.md

# Todo <a name="todo"></a>
- Shell version: this is kind of tricky...

---

# Dependencies <a name="deps"></a>
- [wmctrl](http://tripie.sweb.cz/utils/wmctrl/) to print your window manager
- If you're on __NetBSD__, you'll want to install:
  - [ripgrep](https://github.com/BurntSushi/ripgrep)

---

# Benchmarks <a name="bench"></a>
Macchina is pretty fast, see for yourself:

- Execution time is measured using [hyperfine](https://github.com/sharkdp/hyperfine)

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 42.6 ± 0.5 | 41.8 | 45.2 | 1.00 |
| `neofetch` | 371.1 ± 3.0 | 366.4 | 379.9 | 8.71 ± 0.13 |

__Summary__: `macchina` runs __8.71 ± 0.13__ times __faster__ than `neofetch`

- Note that hiding elements with __--hide__ significantly improves speed

---

# Features <a name="features"></a>
## Themes:
![Theme preview](screenshots/themes.png)

## Macchina displays basic system information such as:
- Host
  - Username
  - Hostname
- Product
- Kernel
- Distribution
- Desktop Environment
- Window Manager
- Package count
- Shell
- Terminal
- Processor
  - Model
  - Thread count
- Uptime
- Memory
  - Used / Total
- Battery
  - Percentage
  - Status
- Palette

Package count supports package managers of:
- Arch-based distributions
- Debian-based distributions
- Gentoo
- NetBSD

## Macchina supports the following arguments:

`--no-color / -n` - Disable colors

`--color / -c <color>` - Specify the key color

`--separator-color / -C <color>` - Specify the separator color

`--random-color / -r` - Let Macchina pick a random color for you

`--palette / -p` - Display palette

`--short-sh / -S` - Shorten shell output (/bin/zsh -> zsh)

`--short-uptime / -U` - Shorten shell output (/bin/zsh -> zsh)

`--hide / -H <element>` - Hide elements such as host, os, kern, etc.

`--show-only / -X <element>` - Display only the provided elements

`--bar / -b` - Display memory usage and battery percentage as bars

![Preview of bar argument](screenshots/bars.png)

`--theme / -t <theme>` - Specify the theme

`--padding / -P <amount>` - Specify the amount of (left) padding to use

`--spacing / -s <amount>` - Specify the amount of spacing to use

`--debug / -d` - Print debug information, use when an element fails to display.

`--help / -h` -  Print help text

`--version / -v` - Print version

---

# Installation <a name="install"></a>
Macchina is available on:

- [AUR](https://aur.archlinux.org/packages/macchina/)
  ```
  git clone https://aur.archlinux.org/macchina.git
  cd macchina
  makepkg -si
  ```

- [NetBSD](https://pkgsrc.se/sysutils/macchina)
  ```
  pkgin install macchina
  ```

- [crates.io](https://crates.io/crates/macchina)
  ```
  cargo install macchina
  ```

---

# Platform Support <a name="platform-support"></a>

|  Platform     |      Support       |
| :-:           |        :-:         |
| Linux         |        Yes         |
| NetBSD [1]    |        Yes         |
| MacOS         |                    |
| Windows       |                    |

[1]: Support for NetBSD would not be possible without the help and testing of NetBSD package maintainer [pin](https://pkgsrc.se/bbmaint.php?maint=pin@NetBSD.org). Thanks __pin__!
