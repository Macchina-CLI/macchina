![Macchina preview image](screenshots/banner.png)

[![Crates.io](https://img.shields.io/crates/v/macchina?style=for-the-badge&label=VERSION&color=0D3B66)](https://crates.io/crates/macchina)
[![Crates.io](https://img.shields.io/crates/d/macchina?style=for-the-badge&label=DOWNLOADS&color=0D3B66)](https://crates.io/crates/macchina)
![reposize](https://img.shields.io/github/repo-size/grtcdr/macchina?color=0D3B66&logo=github&style=for-the-badge)
![loc](https://img.shields.io/tokei/lines/github/grtcdr/macchina?color=0D3B66&label=Lines%20of%20Code&logo=rust&style=for-the-badge)

---

# About Macchina
Macchina is a fetching program, not a library.

Macchina lets you view basic system information, like your hostname, kernel, uptime, memory usage, and much more.
It provides you with convenient features and customization options but doesn't lose sight of its two main priorities: minimalism and performance.

![Macchina preview image](screenshots/preview.png)

---

# Dependencies
- [wmctrl](http://tripie.sweb.cz/utils/wmctrl/) to print your window manager
- If you're on __Gentoo__, you'll want to install:
  - [portage-utils](https://packages.gentoo.org/packages/app-portage/portage-utils) to see package count
- If you're on __NetBSD__, you'll want to install:
  - [ripgrep](https://github.com/BurntSushi/ripgrep) to see battery information

---

# Benchmarks
## Linux
Macchina is pretty fast, see for yourself!

- Execution time is measured using [hyperfine](https://github.com/sharkdp/hyperfine)

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 34.3 ± 0.8 | 32.9 | 35.9 | 1.00 |
| `neofetch` | 369.0 ± 3.3 | 362.9 | 376.1 | 10.77 ± 0.26 |

__Summary__: `macchina` runs __10.77 ± 0.26__ times __faster__ than `neofetch`

- Hiding elements with __--hide__ significantly improves speed

---

# Features
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
- Package Count ¹
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

1. Package count supports the following package managers:
- Pacman
- Portage
- APT
- XBPS
- pkgsrc
- Homebrew

---

# Usage

Macchina comes packed with arguments that let you customize Macchina and its behavior.

The [usage wiki page](https://github.com/grtcdr/macchina/wiki/Usage) can tell you all about them and how they work.

---

# Installation
Macchina is available on:

- [AUR](https://aur.archlinux.org/packages/macchina/)
  ```
  git clone https://aur.archlinux.org/macchina.git
  cd macchina
  makepkg -si

- [NetBSD](https://pkgsrc.se/sysutils/macchina)
  ```
  pkgin install macchina
  ```

- [crates.io](https://crates.io/crates/macchina)
  ```
  cargo install macchina
  ```
  
---

# Platform Support	

|  Platform     |      Support       |
| :-:           |        :-:         |
| Linux         |        Yes         |
| NetBSD ¹      |        Yes         |
| MacOS ²       |        Yes         |
| Windows       |        Soon        |

1. Support for __NetBSD__ would not be possible without the help and testing of NetBSD/pkgsrc package maintainer [pin](https://pkgsrc.se/bbmaint.php?maint=pin@NetBSD.org), Thank you!
2. Support for __macOS__ would not be possible without the help, testing and contributions of [123marvin123](https://github.com/123marvin123), Thank you!