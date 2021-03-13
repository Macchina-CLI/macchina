![Macchina preview image](screenshots/banner.png)

[![Crates.io](https://img.shields.io/crates/v/macchina?style=for-the-badge&label=VERSION&color=0D3B66)](https://crates.io/crates/macchina)
[![Crates.io](https://img.shields.io/crates/d/macchina?style=for-the-badge&label=DOWNLOADS&color=0D3B66)](https://crates.io/crates/macchina)
![reposize](https://img.shields.io/github/repo-size/grtcdr/macchina?color=0D3B66&logo=github&style=for-the-badge)
![loc](https://img.shields.io/tokei/lines/github/grtcdr/macchina?color=0D3B66&label=Lines%20of%20Code&logo=rust&style=for-the-badge)

# Table of Contents
- [About](#about)
- [Dependencies](#deps)
- [Benchmarks](#bench)
- [Features](#features)
- [Usage](#usage)
- [Installation](#install)
- [Platform Support](#platform-support)
- [Contributors](#contributors)
---

# About Macchina <a name="about"></a>

_Macchina_ lets you view basic system information, like your hostname, kernel, uptime, memory usage, and much more.
It provides you with convenient features and customization options but doesn't lose sight of its two main priorities: minimalism and performance.

![Macchina preview image](screenshots/preview.png)

---

# Dependencies <a name="deps"></a>
### Linux:
- `wmctrl`
- __Gentoo Only:__ `portage-utils`
### NetBSD:
- `wmctrl`
- `ripgrep`

The [dependencies wiki page](https://github.com/grtcdr/macchina/wiki/Dependencies) explains why these dependencies exist.

---

# Benchmarks <a name="bench"></a>
_Macchina_ is pretty fast, see for yourself!

- Execution time is measured using [hyperfine](https://github.com/sharkdp/hyperfine)
- Hiding elements with `--hide` significantly improves speed

## Linux
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 34.3 ± 0.8 | 32.9 | 35.9 | 1.00 |
| `neofetch` | 369.0 ± 3.3 | 362.9 | 376.1 | 10.77 ± 0.26 |

__Summary__: `macchina` runs __10.77 ± 0.26__ times __faster__ than `neofetch`

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

# Usage <a name="usage"></a>
_Macchina_ comes packed with arguments that let you customize its behavior and styling.

The [usage wiki page](https://github.com/grtcdr/macchina/wiki/Usage) can tell you all about them and how they work.

---

# Installation <a name="install"></a>
_Macchina_ is available on:

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
| macOS ²       |        Yes         |
| Windows       |        Soon        |

# :earth_africa: Contributors <a name="contributors"></a>
<a href="https://github.com/grtcdr/macchina/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=grtcdr/macchina" />
</a>

_Macchina_, like many other open-source projects, would not be where it is right now without the help of its contributors.

1. Support for __NetBSD__ would not be possible without the help and testing of NetBSD/pkgsrc package maintainer [pin](https://pkgsrc.se/bbmaint.php?maint=pin@NetBSD.org), Thank you!
2. Support for __macOS__ would not be possible without the help, testing and contributions of [123marvin123](https://github.com/123marvin123), Thank you!