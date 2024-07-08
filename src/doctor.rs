use crate::data::Readout;
use colored::Colorize;
use libmacchina::traits::ReadoutError;

#[cfg(windows)]
fn activate_virtual_terminal() {
    colored::control::set_virtual_terminal(true).expect("Could not activate virtual terminal.");
}

fn split_failed_items<'a>(
    failed_items: &'a [&Readout],
) -> (Vec<&'a Readout<'a>>, Vec<&'a Readout<'a>>) {
    let err_items: Vec<_> = failed_items
        .iter()
        .filter(|p| !matches!(p.1.as_ref().err(), Some(ReadoutError::Warning(_))))
        .copied()
        .collect();

    let warn_items: Vec<_> = failed_items
        .iter()
        .filter(|p| matches!(p.1.as_ref().err(), Some(ReadoutError::Warning(_))))
        .copied()
        .collect();

    (err_items, warn_items)
}

fn print_errors<'a>(err_items: &[&'a Readout<'a>]) {
    if err_items.is_empty() {
        println!("🎉 You are good to go! No failures detected.");
    }

    for failed_item in err_items {
        let key = failed_item.0;
        let error = failed_item.1.as_ref().err().unwrap().to_string();

        println!(
            "Readout \"{}\" failed with message: {}",
            key.to_string().bright_blue(),
            error.bright_red()
        );
    }
}

fn print_warnings<'a>(warn_items: &[&'a Readout<'a>], total_failed_items: usize) {
    if warn_items.is_empty() {
        return;
    }

    let warn_len = warn_items.len().to_string().bright_yellow();
    let err_len = total_failed_items.to_string().bright_red();
    println!("\n{warn_len} of the {err_len} unsuccessful read(s) resulted in a warning:");

    for warn_item in warn_items {
        let key = warn_item.0;
        let warn = warn_item.1.as_ref().err().unwrap().to_string();

        println!(
            " Readout \"{}\" threw a warning with message: {}",
            key.to_string().bright_blue(),
            warn.yellow()
        );
    }
}

pub(crate) fn print_doctor(data: &[Readout]) {
    let failed_items: Vec<_> = data.iter().filter(|p| p.1.is_err()).collect();
    let (err_items, warn_items) = split_failed_items(&failed_items);

    #[cfg(windows)]
    activate_virtual_terminal();

    println!(
        "We've collected a total of {} {} including {} {} and {} {} resulting in a {}.\n",
        data.len().to_string().bright_green(),
        "readouts".bright_green(),
        err_items.len().to_string().bright_red(),
        "failed read(s)".bright_red(),
        warn_items.len().to_string().bright_yellow(),
        "read(s)".bright_yellow(),
        "warning".bright_yellow()
    );

    print_errors(&err_items);
    print_warnings(&warn_items, failed_items.len());
}
