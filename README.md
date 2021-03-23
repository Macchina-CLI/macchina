<div align="center">
<h1>Macchina</h1>

Fast, minimal and customizable system information fetcher.

Linux • macOS • Windows • NetBSD

<img src="screenshots/preview.png" alt="Preview" />

<a href="https://crates.io/crates/macchina">
    <img src="https://img.shields.io/crates/v/macchina?label=Version" alt="Version" />
</a>

<a href="https://crates.io/crates/macchina">
    <img src="https://img.shields.io/crates/d/macchina?label=Downloads" alt="Downloads" />
</a>

<a href="https://saythanks.io/to/ba.tahaaziz@gmail.com">
    <img src="https://img.shields.io/badge/Say-Thanks-1EAEDB.svg" alt="Say Thanks" />
</a>
</div>


# Table of Contents
- [About](#about)
- [Benchmarks](#bench)
- [Features](#features)
- [Dependencies](#deps)
- [Usage](#usage)
- [Installation](#install)
- [Platform Support](#platform-support)
- [Contributors](#contributors)
---

# About Macchina <a name="about"></a>
_Macchina_ lets you view basic system information, like your hostname, kernel, uptime, memory usage, and much more.
It provides you with convenient features and extensive customization options but doesn't lose sight of its two main priorities, minimalism and performance.

Are you ricing your desktop and would like a program to display your system information, or you want it to load as soon as you open your terminal?

_Macchina_ has you covered.

If you're interested in the library _Macchina_ uses to fetch your system information, check out [libmacchina](https://github.com/Macchina-CLI/libmacchina), any issues relating to _fetching_ should be filed on that repository.

---

# Benchmarks <a name="bench"></a>
_Macchina_ is pretty fast, see for yourself!

- Execution time is measured using [hyperfine](https://github.com/sharkdp/hyperfine)
- Hiding elements with `--hide` significantly improves speed

## 🐧 Linux
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 19.5 ± 0.7 | 18.6 | 21.5 | 1.00 |
| `neofetch` | 375.6 ± 4.2 | 371.0 | 391.1 | 19.24 ± 0.70 |

__Summary__: `macchina` runs __19.24 ± 0.70__ times __faster__ than `neofetch`

## 🪟 Windows
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 9.0 ± 0.4 | 8.2 | 11.7 | 1.00 |
| `neofetch` | 2207.7 ± 53.8 | 2127.1 | 2325.9 | 244.38 ± 13.43 |

__Summary__: `macchina` runs __244.38 ± 13.43__ times __faster__ than `neofetch`

## 👩🏽‍💻 macOS

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 5.0 ± 0.2 | 4.7 | 5.9 | 1.00 |
| `neofetch` | 484.5 ± 4.3 | 477.2 | 492.3 | 97.10 ± 3.84 |

__Summary__: `macchina` runs __97.10 ± 3.84__ times __faster__ than `neofetch`

## 🚩 NetBSD

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 20.9 ± 2.0 | 19.3 | 30.9 | 1.00 |
| `neofetch` | 380.6 ± 26.0 | 344.8 | 422.1 | 18.25 ± 2.16 |

__Summary__: `macchina` runs __18.25 ± 2.16__ times __faster__ than `neofetch`

---

# Features <a name="features"></a>
## Themes
![Theme preview](screenshots/themes.png)

The [themes wiki page](https://github.com/grtcdr/macchina/wiki/Themes) contains a list of all the built-in themes that you can switch between using the `--theme` flag.

## Bars
Bars provide a way to visualize data, and each theme styles them differently. They can be enabled using the `--bar` flag.

![Preview of bar argument](screenshots/bars.png)

## What it fetches
- Host
  - Username
  - Hostname
- Product
- Kernel
- Distribution
- Operating System
- Desktop Environment
- Window Manager
- Package Count <sup>[[1]](https://github.com/grtcdr/macchina/wiki/Platform-Support)</sup>
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

## Meet Doctor
_Macchina_ comes with a very special flag, allowing you to know why certain elements, for example, your operating system information might not be appearing, it can differentiate between errors, and warnings, and its comprehensive output should help you understand where the issue is coming from.

To use this feature, run `macchina --doctor`

![Doctor flag preview](screenshots/doctor.png)

---

# Usage <a name="usage"></a>
_Macchina_ comes packed with __arguments__ that let you customize its behavior and styling.

The [usage wiki page](https://github.com/grtcdr/macchina/wiki/Usage) can tell you all about them and how they work.

---

# Dependencies <a name="deps"></a>
## 🐧 Linux:
- `wmctrl`
- __Gentoo Only:__ `portage-utils`
## 🚩 NetBSD:
- `wmctrl`

The [dependencies wiki page](https://github.com/grtcdr/macchina/wiki/Dependencies) explains why these dependencies exist.

---

# Installation <a name="install"></a>
### 📦 crates.io
```
cargo install macchina
```

### AUR
```
git clone https://aur.archlinux.org/macchina.git
cd macchina
makepkg -si
```

### pkgsrc
```
pkgin install macchina
```

### 🍻 Homebrew
```
brew install Macchina-CLI/homebrew-tap/macchina
```

_Macchina's_ [installation wiki page](https://github.com/grtcdr/macchina/wiki/Installation) also lists the other ways you can install the program. You might also prefer running the [prebuilt binary](https://github.com/grtcdr/macchina/releases) that corresponds with your operating system.

---

# Platform Support <a name="platform-support"></a>

|  Platform |      Support       |
| :-:       |        :-:         |
| GNU/Linux |        Yes         |
| NetBSD    |        Yes         |
| macOS     |        Yes         |
| Windows   |        80%         |

# 🌍 Contributors <a name="contributors"></a>

[![Crates.io](https://contrib.rocks/image?repo=grtcdr/macchina)](https://github.com/grtcdr/macchina/graphs/contributors)

_Macchina_, like many other open-source projects, would not be where it is right now without the help of its contributors, thank you all so much!

- Support for __NetBSD__ would not be possible without the help and testing of NetBSD/pkgsrc package maintainer [pin](https://pkgsrc.se/bbmaint.php?maint=pin@NetBSD.org)
- Support for __macOS__ and __Windows__ would not be possible without the help, testing and major contributions of [123marvin123](https://github.com/123marvin123)
