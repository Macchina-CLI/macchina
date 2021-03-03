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

Macchina lets you view basic system information, like your hostname, kernel version, uptime, memory usage, and much more.
No one wants a slow fetcher, and Macchina's selling point is providing you with convenient features while keeping performance a priority and minimalism a goal.

![Macchina preview image](screenshots/preview.png)

# Changelog <a name="change"></a>
- [x] Fix shell and terminal failing to fetch for NetBSD

# Todo <a name="todo"></a>
- [ ] Implement battery fetching for NetBSD


---

# Dependencies <a name="deps"></a>
- [wmctrl](http://tripie.sweb.cz/utils/wmctrl/) to print your window manager

---

# Benchmarks <a name="bench"></a>
Macchina is pretty fast, see for yourself:

- Execution time is measured using [hyperfine](https://github.com/sharkdp/hyperfine)

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 22.2 ± 0.7 | 21.0 | 25.1 | 1.00 |
| `neofetch` | 243.9 ± 2.3 | 240.0 | 246.9 | 11.01 ± 0.37 |

__Summary__: `macchina` runs __11.01 ± 0.37__ times __faster__ than `neofetch`

- Note that hiding elements using Macchina's __--hide <element>__ significantly improves speed

---

# Features <a name="features"></a>
## Themes:
![Theme preview](screenshots/themes.png)

## Macchina displays basic system information such as:
- Host
  - Username
  - Hostname
- Product
  - Manufacturer
  - Model name & version
- Kernel
  - Name
  - Version
- Distribution
- Desktop Environment
- Window Manager
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

Package count supports package managers of:
- Arch-based distributions
- Debian-based distributions
- NetBSD

Macchina requires [wmctrl](http://tripie.sweb.cz/utils/wmctrl/) to be installed to print your Window Managern, and only some window managers will be displayed as wmctrl "only works with window managers which implement the EWMH specification"

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
| Linux         |        Yes         |
| NetBSD        |        90%         |
| MacOS         |                    |
| Windows       |                    |
