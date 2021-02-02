<h1 align="center"> Macchina </h1>

<p align="center">
  <img src="preview.png"/>
  <a href="https://forthebadge.com/images/badges/made-with-rust.svg" alt="Made With Rust Badge"></a>
</p>

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)
![reposize](https://img.shields.io/tokei/lines/github/grtcdr/macchina?color=FFD1BA&label=LINES%20OF%20CODE&logo=rust&style=for-the-badge)
![loc](https://img.shields.io/tokei/lines/github/grtcdr/macchina?color=FFD1BA&label=Lines%20of%20Code&style=for-the-badge)

## Table of Contents:
- [About](#about)
- [Changelog](#change)
- [Benchmarks](#bench)
- [Features](#features)
- [Installation](#install)
- [Platform Support](#platform-support)

---

## About Macchina <a name="about"></a>
Macchina lets you flex... I mean view system information.
It's an alternative to slower BASH fetchers but isn't as featureful,
so if you're willing to sacrifice features for speed, then Macchina is the right tool for you.

---

## Changelog <a name="change"></a>
- Macchina can now pick and use a color randomly for you using `--random-color` argument.
- Compress preview image
- Add more badges

---

## Benchmarks <a name="bench"></a>
Macchina is pretty fast, see for yourself:

- Execution time is measured using [hyperfine](https://github.com/sharkdp/hyperfine)

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `macchina` | 22.2 ± 0.7 | 21.0 | 25.1 | 1.00 |
| `neofetch` | 243.9 ± 2.3 | 240.0 | 246.9 | 11.01 ± 0.37 |

__Summary__: `macchina` runs __11.01 ± 0.37__ times __faster__ than `neofetch`

- Note that hiding elements using Macchina's __--hide__ argument significantly improves speed

---

## Features <a name="features"></a>
Macchina displays basic system information such as:
- Hostname
- Operating system
- Kernel version
- Package count _(Arch-based distributions only, will print __0__ on any other distribution)_
- Shell path/name in which macchina was ran
- Terminal instance name in which macchina was ran
- Processor _model name_, _frequency_ and _thread count_
- Memory usage
- Uptime
- Battery _percentage_ and _status_

Macchina supports the following arguments:
- `--no-color`      ->    disable colors
- `--color`         ->    display information using the specified color
- `--random-color`  ->    let macchina choose a color for you
- `--palette`       ->    display palette
- `--short-sh`      ->    shorten shell output (/bin/zsh => zsh)
- `--hide`          ->    hide elements such as host, os, kern, etc.
- `--help`          ->    display the help menu
- `--version`       ->    print Macchina's version

---

## Installation <a name="install"></a>

Macchina is not yet ready to be deployed on [crates.io](https://crates.io/), but you can compile it from source and play around with it.

Here's how _you_ can do that:

1. Clone the repo: `git clone https://github.com/grtcdr/macchina`
2. Navigate to the folder and compile from source: `cd macchina && cargo build`
3. __target/__ has been generated by cargo and Macchina's binary file can now be run: `./target/debug/macchina`

__Bonus__: To run macchina from anywhere on your system, you have two options:

1. Place `macchina/target/debug/macchina` somewhere in your __$PATH__, like _~/.local/bin_ or _/usr/bin_.

:heavy_exclamation_mark: Any changes you make to the source code will apply to the macchina binary file but you'll need to place the newly built binary file on your __$PATH__ __again__ to run it from _anywhere on your system_ with your new changes.

2. Create a new symlink for Macchina:

:heavy_exclamation_mark: This symlink will point to the binary file, so everytime you modify the source code and rebuild, running `$ macchina` from _anywhere on your system_ will run the newly built binary file.

---

## Will Macchina Work on Your Macchina? <a name="platform-support"></a>

|  Platform     |      Support       |
| :-:           |        :-:         |
| Linux         | :heavy_check_mark: |
| BSD           |     :question:     |
| MacOS         |                    |
| Windows       |                    |

> Cells containing :question: have not yet been tested.
