<div align="center">
<h1>macchina</h1>

Fast, minimal and customizable system information frontend.

Linux • macOS • Windows • NetBSD • FreeBSD • OpenWrt • Android

<img src="assets/preview.png" alt="Preview" />
</div>

## About

macchina lets you view system information, like your kernel version, uptime,
memory usage, processor load and much more. _macchina_ is basic by default and
extensible by design.

If you're interested in the library _macchina_ uses to fetch system
information, have a look at [libmacchina][libmacchina]; fetching-related
issues should be filed on that repository.

## Status

macchina is now in *maintenance mode*, the only form of contribution we
will accept moving forward is bug fixes. macchina is a perfectly good
fetcher, it has achieved its intended purpose, for me as well as others.

## Benchmarks

Check out the [benchmarks wiki page](https://github.com/Macchina-CLI/macchina/wiki/Benchmarks).

## Features

### Themes

macchina has a theming system which you can use to customize pretty much
any visual aspect of the program. Themes live *outside* the
configuration file, so you can create a bunch of them and switch between
them at any time.

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

In the event of fetching failures, which can occur for various reasons,
the `--doctor` flag can tell you why that might be happening.

## Configuration

See the [configuration wiki page][configuration].

## Customization

Have a look at the [customization wiki page][customization].

## Installation

Check out the [installation wiki page][installation].  We also provide
[prebuilt binaries][releases] with every release.

## Contributors

macchina like many other open source projects, would not be where it is
right now without the help of its contributors. Whether you've helped
drive it forward by contributing to the codebase, packaged it so we
didn't have to, or recommended it to someone you know — we truly
appreciate your support!

The following is a list of awesome people that have truly shaped macchina:
- [pin](https://pkgsrc.se/bbmaint.php?maint=pin@NetBSD.org): Provided
  massive amounts of help, feedback and testing, and is currently
  packaging macchina on NetBSD.
- [123marvin123](https://github.com/123marvin123): Co-author of (lib)macchina and
  author of countless high-quality contributions and primarily, support for
  macOS and Windows.
- [uttarayan21](https://github.com/uttarayan21): Co-author of
  (lib)macchina and author of numerous shipshape contributions and
  primarily, support for Android and OpenWrt.

Looking to help? [Read this first.][contributing]

[libmacchina]: https://github.com/Macchina-CLI/libmacchina
[releases]: https://github.com/Macchina-CLI/macchina/releases
[installation]: https://github.com/Macchina-CLI/macchina/wiki/Installation
[configuration]: https://github.com/Macchina-CLI/macchina/wiki/Configuration
[customization]: https://github.com/Macchina-CLI/macchina/wiki/Customization
[contributing]: .github/CONTRIBUTING.md
