use clap::{CommandFactory, ValueEnum};
use clap_complete::Shell;
use daktilo::args::Args;
use std::env;
use std::io::Result;

/// Environment variable for the output directory.
const OUT_DIR_ENV: &str = "OUT_DIR";

/// Shell completions can be created with:
///
/// ```sh
/// cargo run --bin daktilo-completions
/// ```
///
/// in a directory specified by the environment variable OUT_DIR.
/// See <https://doc.rust-lang.org/cargo/reference/environment-variables.html>
fn main() -> Result<()> {
    let out_dir = env::var(OUT_DIR_ENV).unwrap_or_else(|_| panic!("{OUT_DIR_ENV} is not set"));
    let mut app = Args::command();
    for &shell in Shell::value_variants() {
        clap_complete::generate_to(shell, &mut app, env!("CARGO_PKG_NAME"), &out_dir)?;
    }
    println!("Completion scripts are generated in {out_dir:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    #[test]
    fn generate_completions() -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../");
        if let Ok(out_dir) = env::var(OUT_DIR_ENV) {
            path = path.join(out_dir);
        } else {
            path = path.join("target");
        }

        env::set_var(OUT_DIR_ENV, path);
        main()?;
        Ok(())
    }
}
