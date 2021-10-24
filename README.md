<div align="center">
<h1>Macchina</h1>

Fast, minimal and customizable system information frontend.

Linux • macOS • Windows • NetBSD • FreeBSD • OpenWrt • Android

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
- [What's new in v4.0](#upgrading)
- [Performance](#performance)
- [Features](#features)
- [Configuration](#configuration)
- [Customization](#customization)
- [Runtime dependencies](#dependencies)
- [Installation](#installation)
- [Contributors](#contributors)

---

# 💬 About <a name="about"></a>

_macchina_ lets you view system information, like your kernel version, uptime,
memory usage, processor load and much more.

If you're interested in the library _macchina_ uses to fetch system information,
have a look at [libmacchina](https://github.com/Macchina-CLI/libmacchina);
fetching-related issues should be filed on that repository.

---

# ✨ What's new in `v4.0` <a name="upgrading"></a>

### Complying with the XDG base directory specification

The themes location has moved from `$XDG_DATA_HOME/macchina/themes` to
`$XDG_CONFIG_HOME/macchina/themes`.

**Why?**

`$XDG_DATA_HOME` is not meant for anything configuration/customization related.

### Command-line flags

We have deprecated many, if not most command-line flags after having ported them
to theme files.

**Why?**

- In previous versions, themes clashed with the main configuration file, and
  their purpose might have seemed to many as unknown.
- There are just way too many flags for a system information fetcher, running
  `--help` ends up confusing you rather than actually helping you out.
- We wanted to simplify and distinguish between the configuration file and
  themes, eliminating redundance.

**Where can I see the updated list?**

In our wiki, [follow this link](https://github.com/Macchina-CLI/macchina/wiki/Usage).


### Themes

Themes have transitioned from JSON to TOML.

Why?

- Because no one can read that JSON stuff.
- Because we want there to be continuity between the configuration and theme
  files, even though they serve completely different purposes.

**Where can I view the new format?**

Take a look at the provided [example theme](https://github.com/Macchina-CLI/macchina/blob/main/theme/Carbon.toml) file.

---

# ⚡️ Performance <a name="performance"></a>

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

# 🚀 Features <a name="features"></a>

## Themes

_macchina_ comes equipped with built-in themes that style their readouts, bars
and separators differently. You can see the list with the `--list-themes` flag
and use the one you like with `--theme <name>`. Did you know that you can [make
your own](#customization)?

Themes live outside the configuration file, so you can create a bunch of them,
and switch between them at any time.

## Bars

Bars provide a way to visualize data, through symbols rather than numbers, and
they can be enabled using the `--bar` flag.

## Doctor

_libmacchina_ can sometimes fail to fetch certain readouts, and _macchina_ has a
feature in place that allows you to see why they failed, this is done through
the `--doctor` flag.

---

# 🔧 Configuration <a name="configuration"></a>

The configuration file define the behavior of macchina, it does not allow for
much customization. See
[macchina.toml](https://github.com/Macchina-CLI/macchina/blob/main/macchina.toml)
for an example configuration file.

In order for _macchina_ to be able to read the configuration file, you need to
place `macchina.toml` in:
- `$XDG_CONFIG_HOME/macchina` on Linux and the BSDs.
- `$HOME/Library/Application Support/macchina` on macOS.
- `{FOLDERID_RoamingAppData}/macchina` on Windows.

# 🎨 Customization <a name="customization"></a>

Themes define the look, layout and styling of _macchina_.  See
[Carbon.toml](https://github.com/Macchina-CLI/macchina/blob/main/theme/Carbon.toml)
for an example theme.

In order for _macchina_ to be able to find your themes, you need to
place them in:
- `$XDG_CONFIG_HOME/macchina/themes` on Linux and the BSDs.
- `$HOME/Library/Application Support/macchina/themes` on macOS.
- `{FOLDERID_RoamingAppData}/macchina/themes` on Windows.

> Note: Avoid using the names of built-in themes.

To start using your theme:

1. Run `macchina --list-themes` to verify that macchina has listed your theme.
2. Add that same name you see in your terminal to the `theme` option in `macchina.toml`
3. You're good to go! _macchina_ will start using your theme.

---

# 📦️ Runtime dependencies <a name="dependencies"></a>

These dependencies are **not** required, but they extend what _macchina_ can
show.

### Linux:

- _wmctrl_
- Gentoo: _portage-utils_

### NetBSD:

- _wmctrl_

---

# 🏗️ Installation <a name="installation"></a>

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
