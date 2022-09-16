# Changelog

## `6.1.2`

- Fixed an issue that caused installations through `cargo` to fail due to a
  malformed lockfile.

##  `6.1.1`

- Updated dependencies to their latest versions
- Removed `--export-config` flag
- Renamed `CHANGELOG.txt` to `CHANGELOG.md`

## `6.1.0`

Yanked.

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

- Fix incorrect target_os for a cfg flag.

