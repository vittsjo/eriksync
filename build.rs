#[macro_use]
extern crate clap;
use clap::Shell;

include!("src/cli.rs");

fn main() {
    let outdir = match std::env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };

    let bin_name = "eriksync";
    let mut app = build_cli();

    app.gen_completions(bin_name, Shell::Bash, outdir.clone());
    app.gen_completions(bin_name, Shell::Zsh, outdir.clone());
    app.gen_completions(bin_name, Shell::Fish, outdir.clone());
}
