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

- [About](#about)
- [What's new in v2.1](#upgrading)
- [Performance](#performance)
- [Features](#features)
- [Configuration](#configuration)
- [Runtime dependencies](#runtime-dependencies)
- [Installation](#installation)
- [Contributors](#contributors)

---

# About

_macchina_ lets you view system information, like your kernel version, uptime,
memory usage, processor load and much more.

If you're interested in the library _macchina_ uses to fetch system information,
have a look at [libmacchina](https://github.com/Macchina-CLI/libmacchina);
fetching-related issues should be filed on that repository.

---

# What's new in v2.1 <a name="upgrading"></a>

### LocalIP Readout

You are now required to specify your network interface for the local IP readout to work properly.

- In your __macchina.toml__, add the following:

```toml
# The interface name might differ on your machine, please check in with your network utility e.g. `ip address`
interface = "wlan0"
```

Why the sudden change?

- We changed IP crates. The previous crate would ping Google DNS servers in
  order to fetch your local IP. And nobody wants that.

- We understand that a lot of you are developers and/or power users, and need
  your fetcher to be quick, powerful and extensible. If it's a docker container
  whose local IP you wanna grab, a virtual machine or anything that relies on a
  network interface to communicate with the outside world, we've got you
  covered.

### Kernel Readout

You can now shorten the output of the kernel readout through the new
`--long-kernel` flag or by adding the following to your __macchina.toml__:

```toml
# When set to false, only the version of your 
# operating system's kernel will be displayed.
long_kernel = false
```

---

# Performance

_macchina_ is lightning fast, see for yourself:

- Execution time is measured using
  [hyperfine](https://github.com/sharkdp/hyperfine)

## 🐧 Linux

This benchmark was performed using an **Intel® Core™ i5-3350P CPU @ 3.10GHz**

| Command    | Mean [ms] | Min [ms] | Max [ms] |
| :--------- | --------: | -------: | -------: |
| `macchina` | 4.8 ± 0.4 |      4.5 |      8.1 |

## 🍏 macOS

| Command    | Mean [ms] | Min [ms] | Max [ms] |
| :--------- | --------: | -------: | -------: |
| `macchina` | 5.0 ± 0.2 |      4.7 |      5.9 |

## 🚩 NetBSD

This benchmark was performed inside a virtual machine using an **Intel® Core™
i5-8265U CPU @ 1.60GHz**

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
and separators differently. You can see the list with the `--list-themes` flag
and use the one you like with `--theme <name>`.

## Bars

Bars provide a way to visualize data, through symbols rather than numbers, and
they can be enabled using the `--bar` flag.

## Doctor

_libmacchina_ can sometimes fail to fetch certain readouts, and _macchina_ has a
feature in place that allows you to see why they failed, this is done through
the `--doctor` flag.

---

# Configuration

See
[macchina.toml](https://github.com/Macchina-CLI/macchina/blob/main/macchina.toml)
for an example configuration file.

- In order for _macchina_ to be able to read the configuration file, you need
  to place `macchina.toml` in:
  - `$XDG_CONFIG_HOME/macchina` on Linux and the BSDs.
  - `$HOME/Library/Application Support/macchina` on macOS.
  - `{FOLDERID_RoamingAppData}/macchina` on Windows.

You can also create custom themes in `JSON` format. Themes allow for more
customization and are separate from the main configuration file. See
[Carbon.json](https://github.com/Macchina-CLI/macchina/blob/main/theme/Carbon.json)
for an example theme.

- In order for _macchina_ to be able to read your custom themes, you need to
  place them in:
  - `$XDG_DATA_HOME/macchina/themes` on Linux and the BSDs.
  - `$HOME/Library/Application/macchina/themes` on macOS. 
  - `{FOLDERID_RoamingAppData}/macchina/themes` on Windows.

You can have as many as you want, just avoid using the names of built-in
themes.

To start using your theme:

1. Run `macchina --list-themes` to verify that macchina has listed your theme.
2. Inside `macchina.toml`, add `theme = <name_of_theme_without_json_extension>`.
3. You're good to go! _macchina_ will start using your theme.

---

# Runtime dependencies

These dependencies are **not** required, but they extend what _macchina_ can
show.

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

The
[installation wiki page](https://github.com/grtcdr/macchina/wiki/Installation)
lists some other ways you can install the program or the steps to compile from
source.

You might prefer running the
[prebuilt binary](https://github.com/grtcdr/macchina/releases) that corresponds
with your operating system.

---

# Contributors

[![Crates.io](https://contrib.rocks/image?repo=grtcdr/macchina)](https://github.com/grtcdr/macchina/graphs/contributors)

_macchina_, like many other open-source projects, would not be where it is right
now without the help of its contributors, thank you all so much!

- Support for **NetBSD** would not have been possible without the help and
  testing of NetBSD/pkgsrc package maintainer
  [pin](https://pkgsrc.se/bbmaint.php?maint=pin@NetBSD.org)
- Support for **macOS** and **Windows** would not have been possible without the
  help, testing and major contributions of
  [123marvin123](https://github.com/123marvin123)
- Support for **OpenWrt** and **Android** was made possible through the many
  contributions of [uttarayan21](https://github.com/uttarayan21)
