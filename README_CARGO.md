![Macchina preview image](screenshots/banner.png)

[![Crates.io](https://img.shields.io/crates/v/macchina?style=for-the-badge&label=VERSION&color=0D3B66)](https://crates.io/crates/macchina)
[![Crates.io](https://img.shields.io/crates/d/macchina?style=for-the-badge&label=DOWNLOADS&color=0D3B66)](https://crates.io/crates/macchina)
![reposize](https://img.shields.io/github/repo-size/grtcdr/macchina?color=0D3B66&logo=github&style=for-the-badge)
![loc](https://img.shields.io/tokei/lines/github/grtcdr/macchina?color=0D3B66&label=Lines%20of%20Code&logo=rust&style=for-the-badge)

---

# About Macchina
_Macchina_ is a fetching program.
If you're interested in its library, check out [macchina-read](https://crates.io/crates/macchina-read).

_Macchina_ lets you view basic system information, like your hostname, kernel, uptime, memory usage, and much more.
It provides you with convenient features and customization options but doesn't lose sight of its two main priorities: minimalism and performance.

![Macchina preview image](screenshots/preview.png)

---

# Benchmarks
_Macchina_ is pretty fast, see for yourself!

- Execution time is measured using [hyperfine](https://github.com/sharkdp/hyperfine)
- Hiding elements with `--hide` significantly improves speed

## 🐧 Linux
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 19.5 ± 0.7 | 18.6 | 21.5 | 1.00 |
| `neofetch` | 375.6 ± 4.2 | 371.0 | 391.1 | 19.24 ± 0.70 |

__Summary__: `macchina` runs __19.24 ± 0.70__ times __faster__ than `neofetch`

## 👩🏽‍💻 macOS

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 5.0 ± 0.2 | 4.7 | 5.9 | 1.00 |
| `neofetch` | 484.5 ± 4.3 | 477.2 | 492.3 | 97.10 ± 3.84 |

__Summary__: `macchina` runs __97.10 ± 3.84__ times __faster__ than `neofetch`

## 🚩 NetBSD

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 21.6 ± 2.6 | 18.9 | 31.3 | 1.00 |
| `neofetch` | 269.9 ± 10.5 | 256.2 | 290.0 | 12.51 ± 1.58 |

__Summary__: `macchina` runs __12.51 ± 1.58__ times __faster__ than `neofetch`

---

# Features
### Themes
![Theme preview](screenshots/themes.png)

The [themes wiki page](https://github.com/grtcdr/macchina/wiki/Themes) contains a list of all the built-in themes that you can use.

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
- Package Count [[1]](https://github.com/grtcdr/macchina/wiki/Platform-Support)
- Shell
- Terminal
- Processor
  - Model
  - Thread count
- Local IP Address
- Uptime
- Memory Usage
- Battery
  - Percentage
  - Status
- Palette

---

# Usage

_Macchina_ comes packed with __arguments__ that let you customize its behavior and styling.

The [usage wiki page](https://github.com/grtcdr/macchina/wiki/Usage) can tell you all about them and how they work.

---

# Dependencies
## 🐧 Linux:
- `wmctrl`
- __Gentoo Only:__ `portage-utils`
## 🚩 NetBSD:
- `wmctrl`

The [dependencies wiki page](https://github.com/grtcdr/macchina/wiki/Dependencies) explains why these dependencies exist.

---

# Installation <a name="install"></a>

- Install it using cargo:
```
cargo install macchina
```

_Macchina's_ [installation wiki page](https://github.com/grtcdr/macchina/wiki/Installation) also lists multiple other ways you can install it. You might also prefer running the [prebuilt binary](https://github.com/grtcdr/macchina/releases) that corresponds with your operating system.
  
---

# Platform Support	

|  Platform   |      Support       |
| :-:         |        :-:         |
| GNU/Linux   |        Yes         |
| NetBSD      |        Yes         |
| macOS       |        Yes         |
| Windows     |        Partial     |

# 🌍 Contributors

[![Crates.io](https://contrib.rocks/image?repo=grtcdr/macchina)](https://github.com/grtcdr/macchina/graphs/contributors)

_Macchina_, like many other open-source projects, would not be where it is right now without the help of its contributors, thank you all so much!

- Support for __NetBSD__ would not be possible without the help and testing of NetBSD/pkgsrc package maintainer [pin](https://pkgsrc.se/bbmaint.php?maint=pin@NetBSD.org)
- Support for __macOS__ and __Windows__ would not be possible without the help, testing and major contributions of [123marvin123](https://github.com/123marvin123)