<h1 align="center"> Macchina </h1>

<p align="center">
  <img src="screenshots/preview.png"/>
  <a href="https://forthebadge.com/images/badges/made-with-rust.svg" alt="Made With Rust Badge"></a>
</p>

![madewith](https://img.shields.io/static/v1?label=MADE%20WITH&message=RUST&color=ef4041&style=for-the-badge)
![reposize](https://img.shields.io/github/repo-size/grtcdr/macchina?color=BEE5BF&logo=github&style=for-the-badge)
![loc](https://img.shields.io/tokei/lines/github/grtcdr/macchina?color=FFD1BA&label=Lines%20of%20Code&logo=rust&style=for-the-badge)

## Table of Contents:
- [About](#about)
- [Changelog](#change)
- [Benchmarks](#bench)
- [Features](#features)
- [Installation](#install)
- [Platform Support](#platform-support)

---

# About Macchina <a name="about"></a>
Macchina lets you view basic system information, like your hostname, your kernel version, memory usage, and much more.
No one wants a slow fetcher, and macchina's main goal is to provide you with handy features while keeping performance a priority.

# Changelog <a name="change"></a>
Instead of sleeping, I made some achievements:
- Simplify Macchina's padding implementation
- Simpler and more maintainable bar system:
  - In order to implement a new feature, which is a different bar glyph and brackets for different themes, the old bar implementation had to be revised, because it was nothing short of a __mess__... It's now half as long, _probably_ faster, but the end result was, drumroll please... __clean code__.

---

# Benchmarks <a name="bench"></a>
Macchina is pretty fast, see for yourself:

- Execution time is measured using [hyperfine](https://github.com/sharkdp/hyperfine)

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 22.2 ± 0.7 | 21.0 | 25.1 | 1.00 |
| `neofetch` | 243.9 ± 2.3 | 240.0 | 246.9 | 11.01 ± 0.37 |

__Summary__: `macchina` runs __11.01 ± 0.37__ times __faster__ than `neofetch`

- Note that hiding elements using Macchina's __--hide__ argument significantly improves speed

---

# Features <a name="features"></a>
Macchina displays basic system information such as:
- Hostname
- Product
  - Manufacturer
  - Model name
- Distribution
- Desktop Environment
- Kernel version
- Package count
- Shell
- Terminal
- Processor
  - Model name
  - Frequency
  - Thread count
- Uptime
- Memory
  - Used / Total
- Battery
  - Percentage
  - Status
- Palette

> Package count: Arch-based distributions only, as it will print __0__ on any other distribution

Macchina supports the following arguments:

`--no-color` - Disable colors

`--color <color>` - Specify the key color

`--separator-color <color>` - Specify the separator color

`--random-color` - Let Macchina pick a random color for you

`--palette` - Display palette

`--short-sh` - Shorten shell output (/bin/zsh -> zsh)

`--hide <element>` - Hide elements such as host, os, kern, etc.

`--bar` - Display memory usage and battery percentage as bars

![Preview of bar argument](screenshots/bars.png)

`--theme <theme_name>` - Change themes

`--padding <amount>` - Specify the amount of (left) padding to use

`--help` -  Display help menu

`--version` - Print version

---

# Installation <a name="install"></a>
Macchina is available on:

- [AUR](https://aur.archlinux.org/packages/macchina/)

  Install using your favorite AUR helper or by running:
  ```
  git clone https://aur.archlinux.org/macchina.git
  cd macchina
  makepkg -si
  ```
- [crates.io](https://crates.io/crates/macchina)

  Install using cargo:
  ```
  cargo install macchina
  ```

---

# Platform Support <a name="platform-support"></a>

|  Platform     |      Support       |
| :-:           |        :-:         |
| Linux         | :heavy_check_mark: |
| BSD           |     :question:     |
| MacOS         |                    |
| Windows       |                    |

Cells containing :heavy_check_mark:: Macchina supports that platform

Cells containing :question:: Macchina has not been tested yet on that platform

Empty cells: Macchina does not support that platform
