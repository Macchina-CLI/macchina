<div align="center">
<h1>macchina</h1>

Fast, minimal and customizable system information frontend.

Linux • macOS • Windows • NetBSD • FreeBSD • OpenWrt • Android

<a href="https://matrix.to/#/#macchina:matrix.org">
    <img src="https://img.shields.io/matrix/macchina:matrix.org" alt="Matrix" />
</a>

<a href="https://crates.io/crates/macchina">
    <img src="https://img.shields.io/crates/v/macchina?label=Version" alt="Version" />
</a>

<a href="https://crates.io/crates/macchina">
    <img src="https://img.shields.io/crates/d/macchina?label=Downloads" alt="Downloads" />
</a>

<a href="https://github.com/Macchina-CLI/macchina/actions">
   <img src="https://github.com/Macchina-CLI/macchina/actions/workflows/macchina.yml/badge.svg" alt="CI">
</a>

<img src="assets/preview.png" alt="Preview" />

</div>

## About

_macchina_ lets you view system information, like your kernel version, uptime,
memory usage, processor load and much more. _macchina_ is **basic** by default
and **extensible** by design.

If you're interested in the library _macchina_ uses to fetch system
information, have a look at
[libmacchina]; fetching-related
issues should be filed on that repository.

## Status

_macchina_ is currently in **maintenance mode**, i.e. bug fixes and little
optimizations are prioritized over the addition of new features. This is due to
the fact that the authors do not have the time to focus on moving the project
forward while keeping up with the demanding nature of our lives. _macchina_
**will**, **at some point**, although I can't say when, **leave this stage**
and implement all the bells and whistles the community has been requesting. We
want to ensure that this project can compete in a healthy way with other
similar projects, so we can provide you with an alternative that is both usable
from the standpoint of both the advanced, or novice user. Furthermore, we want
to keep the promise of continuing to attain great performance, the goal we
initially set out to achieve.

We hope you understand our situation and continue to support
_macchina_.

## Benchmarks <a name="benchmarks"></a>

Check out the [benchmarks wiki page](https://github.com/Macchina-CLI/macchina/wiki/Benchmarks).

## Features

### Themes

_macchina_ has a theming system which you can use to customize pretty much any
visual aspect of the program. Themes live **outside** the configuration file,
so you can create a bunch of them and switch between them at any time.

Why are they separate?

- **Modularity** — themes are an engine of their own, and their sole purpose is
  to provide an interface that allows for the modification of _macchina's_
  visual components. It makes sense to separate them from the main
  configuration file.

- **Portability** — sure, the configuration file is shareable, but what if you
  wanted to share the look of _your macchina_ and not its behavior? What if you
  wanted to switch between dozens of themes that you very carefully designed?
  The way we handle customization answers this need.

Learn how to [make your own](#customization).

### Doctor

In the event of fetching failures, which can occur for various reasons, the
`--doctor` flag that can tell you why that might be happening.

## Configuration

See the [configuration wiki page](https://github.com/Macchina-CLI/macchina/wiki/Configuration).

## Customization

Have a look at the [customization wiki page](https://github.com/Macchina-CLI/macchina/wiki/Customization).

## Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/macchina.svg)](https://repology.org/project/macchina/versions)

Check out the [installation wiki
page](https://github.com/Macchina-CLI/macchina/wiki/Installation).

[Prebuilt binaries](https://github.com/grtcdr/macchina/releases) are also
provided with every release, feel free to use them.

## Contributors

[![Crates.io](https://contrib.rocks/image?repo=grtcdr/macchina)](https://github.com/grtcdr/macchina/graphs/contributors)

_macchina_, like many other open source projects, would not be where it is
right now without the help of its contributors — Whether you've helped drive it
forward by contributing to the codebase, packaged it so we didn't have to, or
recommended it to someone — _We love you_ :heart:

The following is a list of awesome people that have truly shaped _macchina_:
- [pin](https://pkgsrc.se/bbmaint.php?maint=pin@NetBSD.org): Provided massive
  amounts of help, feedback and testing, and is currently packaging _macchina_ on
  **NetBSD**.
- [123marvin123](https://github.com/123marvin123): Co-author of _(lib)macchina_ and
  author of countless high-quality contributions and primarily, support for
  macOS and Windows.
- [uttarayan21](https://github.com/uttarayan21): Co-author of _(lib)macchina_ and
  author of numerous shipshape contributions and primarily, support for Android and OpenWrt.

Looking to help? [Read this first.](CONTRIBUTING.md)

# ASCII Art

Some of the ASCII art displayed in macchina is **not** our own.

If you (the artist) are not okay with us using your ASCII art, please [contact
me](mailto:ba.tahaaziz@gmail.com).

- FreeBSD ASCII art (small variant) was taken from Dylan Araps'
  [pfetch](https://github.com/dylanaraps/pfetch)
- macOS ASCII art (big variant) was taken from Dylan Araps'
  [Neofetch](https://github.com/dylanaraps/neofetch)
- macOS ASCII art (small variant) was made by Joan Stark (jgs)
- Linux ASCII art (big variant) was made by Joan Stark (jgs)
- Linux ASCII art (small variant) was taken from Christopher Johnson's ASCII
  art collection (unknown artist)

[libmacchina]: https://github.com/Macchina-CLI/libmacchina
