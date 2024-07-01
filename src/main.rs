use clap::Parser;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, ()>;

fn runtime() -> Result<()> {
    let cli = CommandLine::parse();

    if !cli.into {
        match cli.paths.as_slice() {
            [src, dst] => {
                if cli.copy {
                    std::fs::rename(src, dst)
                } else {
                    std::fs::copy(src, dst).map(|_| ())
                }
                .map_err(|err| eprintln!("movefile: Failed to move file: {err}"))?;
            }
            _ => {
                eprint!(include_str!("error_path_len.txt"), count = cli.paths.len());
                return Err(());
            }
        }
    } else {
        if cli.paths.len() < 2 {
            eprintln!("movefile: No file to move");
            return Err(());
        }

        match cli.paths.as_slice() {
            [files @ .., target] => {
                for src in files {
                    let Some(file_name) = src.file_name() else {
                        eprintln!("movefile: {} not exists", src.display());
                        return Err(());
                    };

                    let dst = target.join(file_name);

                    if cli.copy {
                        std::fs::rename(src, dst)
                    } else {
                        std::fs::copy(src, dst).map(|_| ())
                    }
                    .map_err(|err| eprintln!("movefile: Failed to move file: {err}"))?;
                }
            }
            _ => {
                eprintln!("movefile: No file to move");
                return Err(());
            }
        }
    }

    Ok(())
}

fn main() {
    if runtime().is_err() {
        std::process::exit(1);
    }
}

/// Move (rename) files.
#[derive(Parser, Debug)]
struct CommandLine {
    /// Move files into a directory.
    ///
    /// This requires at least 2 paths, and last path is a directory name.
    #[clap(short, long)]
    into: bool,

    /// Copy file instead of move.
    #[clap(short, long)]
    copy: bool,

    paths: Vec<PathBuf>,
}
