#![allow(dead_code)]
use vergen::{ShaKind,Config};
// use structopt::clap::Shell;
// mod format {
//     include!("src/format.rs");
// }
// mod bars {
//     include!("src/bars.rs");
// }
// mod data {
//     include!("src/data/mod.rs");
// }
// mod theme {
//     include!("src/theme.rs");
// }
// mod cli {
//     include!("src/cli.rs");
// }
fn main() {
    // let name = "macchina";
    let outdir = match std::env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };

    let stamp_path = std::path::Path::new(&outdir).join("macchina-stamp");
    if let Err(err) = std::fs::File::create(&stamp_path) {
        panic!("failed to write {}: {}", stamp_path.display(), err);
    }


    // let mut cli = cli::build_cli();
    // cli.gen_completions(name, Shell::Fish, &outdir);
    // cli.gen_completions(name, Shell::Bash, &outdir);


    let mut config = Config::default();
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    // Fix this later 
    // This will break builds if the git folder is removed
    vergen::vergen(config).unwrap(); 
}

