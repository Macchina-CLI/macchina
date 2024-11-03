# Changelog

## `6.4.0`

Orhun Parmaksız:
- Bump `ansi-to-tui` to its latest version
- Replace dependency on `color-to-tui` by enabling the `serde` feature from the `ratatui` crate
- Revert the vendoring model introduced in version `6.3.0` (#334)

Apologies to any package maintainer that has to undo their changes, the vendoring fix should've been addressed more appropriately in the first place.

## `6.3.1`

grtcdr:
- Use fixed version for libmacchina

Many thanks to 0323pin for raising the build issue caused by the previous release.

## `6.3.0`

grtcdr:
- Vendor `ansi-to-tui` and `color-to-tui` as submodules
- Trigger CI jobs only when source code is changed
- Show the disk path in the disk readout value

Rolv Apneseth:
- Add a new `--disks` flag, allowing users to specify multiple disks to show disk usage for.

## `6.2.1`

- Fix ratatui dependency issue causing installations to fail

0323pin:
- Fix outdated `config.toml` documentation

## `6.2.0`

- Add a new `--memory-percentage` flag to display used memory in percentage.

Rolv Apneseth:
- Minor fixes to the documentation (#274)
- Display readouts in the configured order (#275)
- Add a new GPU readout (#278)
- Add a new disk space readout (#283)

Adrian Groh:
- Add a workaround for failing Android build (#280)
- Fix lint warnings (#281)

Matthias Baer:
- Migrate from `tui-rs` to `ratatui` (#287)
- Improve CI

feefs:
- Implement a dedicated spacing customization option for the palette

Xiangkun Liu:
- Fix some of the palette's color names

Charlie Lin:
- Fix build issues between ratatui and color-to-tui

Thank you to everyone that has contributed to this release, your support
is greatly appreciated.

## `6.1.8`

- Don't panic when unwrapping ReadoutKey::from_str (Fixes: https://github.com/Macchina-CLI/macchina/issues/270)

## `6.1.7`

- All dependencies have been bumped to their latest version.

## `6.1.6`

libmacchina has been bumped to `v6.3.1` which:
- Fixes a bug that causes framerate to appear as nil on certain macOS systems
  (Author: 123marvin123)

## `6.1.5`

- Fixed `--version` not displaying the version (Author: 123marvin123)

## `6.1.4`

### Fixes

- We hope this release addresses the build issues that have been occurring lately.

### Features

libmacchina has been bumped to `v6.3.0` which includes a new feature:
- Implement backlight readout for macOS (Author: 123marvin123)

## `6.1.3`

Yanked.

See [this comment for specifics](https://github.com/Macchina-CLI/macchina/issues/263#issuecomment-1250045395).

## `6.1.2`

- Fixed an issue that caused installations through `cargo` to fail due to a
  malformed lockfile.

##  `6.1.1`

- Updated dependencies to their latest versions
- Removed `--export-config` flag
- Renamed `CHANGELOG.txt` to `CHANGELOG.md`

## `6.1.0`

Yanked.

See this [commit message for specifics](https://github.com/Macchina-CLI/macchina/commit/fb31328cf75e3e945a70b80cb1891a062a63de5e).

## `6.0.6`

Bump libmacchina to v6.1.0:
- Fixes a bug that causes the package readout to display "0 (cargo)" if
  $CARGO_HOME/bin is empty.
  (https://github.com/Macchina-CLI/libmacchina/commit/22a7df0f74e7d14c34cbfc35b40b61d5f2b5d199)
- Fixes a bug that causes the network readout to return an IPv6 address in some cases.
  (https://github.com/Macchina-CLI/libmacchina/commit/608a1dde39def981d2750f4221c217151b80437e)

Contributors:
- luckman212

## `6.0.5`

- Fix incorrect `target_os` for a particular `cfg!` flag.
