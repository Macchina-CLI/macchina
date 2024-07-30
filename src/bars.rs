/**
Returns a `usize` whose value can range from 0 up to 10 based on the given `value`.
This is used to calculate the number of blocks to show
in a bar.

For example:
- __CPU Usage__ ranges from 0 to 100%, this function can return a `usize`
  that tells the bar how many of its blocks should be represented as being used.

The same goes for __battery percentage__, as it ranges from 0 to 100%.
*/
pub fn num_to_blocks(value: u8) -> usize {
    match value {
        0..=10 => 1,
        11..=20 => 2,
        21..=30 => 3,
        31..=40 => 4,
        41..=50 => 5,
        51..=60 => 6,
        61..=70 => 7,
        71..=80 => 8,
        81..=90 => 9,
        91..=100 => 10,
        // 0 is reserved for errors
        _ => 0,
    }
}

/// Returns a `usize` whose value can range from 0 up to 10 based on the given `value`.
/// This is very similar to `num_to_blocks` but the calculations are done in a different way.
pub fn usage(used: u64, total: u64) -> usize {
    let used = used as f64;
    let total = total as f64;

    (used / total * 10f64).ceil() as usize
}
