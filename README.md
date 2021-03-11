![Macchina preview image](screenshots/banner.png)

[![Crates.io](https://img.shields.io/crates/v/macchina?style=for-the-badge&label=VERSION&color=0D3B66)](https://crates.io/crates/macchina)
[![Crates.io](https://img.shields.io/crates/d/macchina?style=for-the-badge&label=DOWNLOADS&color=0D3B66)](https://crates.io/crates/macchina)
![reposize](https://img.shields.io/github/repo-size/grtcdr/macchina?color=0D3B66&logo=github&style=for-the-badge)
![loc](https://img.shields.io/tokei/lines/github/grtcdr/macchina?color=0D3B66&label=Lines%20of%20Code&logo=rust&style=for-the-badge)

# Table of Contents
- [About](#about)
- [Changelog](#change)
- [Todo](#todo)
- [Dependencies](#deps)
- [Benchmarks](#bench)
- [Features](#features)
- [Usage](#usage)
- [Installation](#install)
- [Platform Support](#platform-support)

---

# About Macchina <a name="about"></a>

Macchina lets you view basic system information, like your hostname, kernel, uptime, memory usage, and much more.
It provides you with convenient features and customization options but doesn't lose sight of its two main priorities: minimalism and performance.

![Macchina preview image](screenshots/preview.png)

# Changelog <a name="change"></a>
- [x] Remove `def` theme to fix a memory allocation bug, `def` theme is the default theme and is no different from running `macchina` without the `--theme` argument.
- [x] Macchina now reads battery information from `/sys/class/power_supply/BAT1` if `/sys/class/power_supply/BAT0` doesn't exist.
- [x] Add `--show-only` argument to help text.

# Todo <a name="todo"></a>
- [ ] @123marvin123 is working on macOS support!
- [ ] ASCII Art

---

# Dependencies <a name="deps"></a>
- [wmctrl](http://tripie.sweb.cz/utils/wmctrl/) to print your window manager
- If you're on __Gentoo__, you'll want to install:
  - [portage-utils](https://packages.gentoo.org/packages/app-portage/portage-utils) to see package count
- If you're on __NetBSD__, you'll want to install:
  - [ripgrep](https://github.com/BurntSushi/ripgrep) to see battery information

---

# Benchmarks <a name="bench"></a>
Macchina is pretty fast, see for yourself!

- Execution time is measured using [hyperfine](https://github.com/sharkdp/hyperfine)

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 42.6 ± 0.5 | 41.8 | 45.2 | 1.00 |
| `neofetch` | 371.1 ± 3.0 | 366.4 | 379.9 | 8.71 ± 0.13 |

__Summary__: `macchina` runs __8.71 ± 0.13__ times __faster__ than `neofetch`

- Note that hiding elements with __--hide__ significantly improves speed

---

# Features <a name="features"></a>
### Themes
![Theme preview](screenshots/themes.png)

### Bars
![Preview of bar argument](screenshots/bars.png)

### What it fetches
- Host
  - Username
  - Hostname
- Product
- Kernel
- Distribution
- Desktop Environment
- Window Manager
- Package count ¹
- Shell
- Terminal
- Processor
  - Model
  - Thread count
- Uptime
- Memory Usage
- Battery
  - Percentage
  - Status
- Palette

---

1. Package count supports package managers of:
- Arch-based distributions
- Debian-based distributions
- Gentoo
- Void Linux
- NetBSD

---

# Usage <a name="usage"></a>
Macchina comes packed with arguments that let you customize Macchina's output and its behavior.

The [usage wiki page](https://github.com/grtcdr/macchina/wiki/Usage) can tell you all about them and how they work.

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
| NetBSD ¹      |        Yes         |
| MacOS         |        Soon        |
| Windows       |                    |

1. Support for NetBSD would not be possible without the help and testing of NetBSD package maintainer [pin](https://pkgsrc.se/bbmaint.php?maint=pin@NetBSD.org). Thanks __pin__!
