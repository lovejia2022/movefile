use clap::Parser;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, ()>;

fn runtime() -> Result<()> {
    let cli = CommandLine::parse();

    if !cli.into {
        match cli.paths.as_slice() {
            [src, dst] => {
                copy_or_move(src, &dst, cli.copy)?;
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
                        eprintln!("movefile: File '{}' not exists", src.display());
                        return Err(());
                    };

                    let dst = target.join(file_name);

                    copy_or_move(src, &dst, cli.copy)?;
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

fn copy_or_move(src: &Path, dst: &Path, copy: bool) -> Result<()> {
    if copy {
        if src.is_file() {
            std::fs::copy(src, dst)
                .map_err(|err| eprintln!("movefile: Failed to copy/move file: {err}"))?;
        } else if src.is_dir() {
            if !dst.exists() {
                copy_tree(src, dst)?;
            } else {
                eprintln!(
                    "movefile: Can't copy directory '{}' to '{}', target is exists",
                    src.display(),
                    dst.display()
                );
                return Err(());
            }
        } else {
            eprintln!(
                "movefile: Can't copy '{}', not a file or directory",
                src.display()
            );
            return Err(());
        }
    } else {
        std::fs::rename(src, dst)
            .map_err(|err| eprintln!("movefile: Failed to move file: {err}"))?;
    }
    Ok(())
}

fn copy_tree(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst).map_err(|err| {
        eprintln!(
            "movefile: Failed to create directory '{}': {err}",
            dst.display()
        )
    })?;

    for item in std::fs::read_dir(src).map_err(|err| {
        eprintln!(
            "movefile: Failed to open directory '{}': {err}",
            src.display()
        )
    })? {
        let item = item.map_err(|err| {
            eprintln!(
                "movefile: Failed to open directory '{}': {err}",
                src.display()
            )
        })?;

        let file_type = item.file_type().map_err(|err| {
            eprintln!(
                "movefile: Failed to read file type of '{}': {err}",
                src.display()
            )
        })?;

        if file_type.is_file() {
            copy_or_move(&item.path(), &dst.join(item.file_name()), true)?;
        } else if file_type.is_dir() {
            copy_tree(&item.path(), &dst.join(item.file_name()))?;
        } else {
            eprintln!(
                "movefile: Failed to move file '{}', This is not directory or file",
                src.display()
            );
            return Err(());
        }
    }

    Ok(())
}

fn main() {
    if runtime().is_err() {
        std::process::exit(1);
    }
}

/// Copy or move (rename) files.
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
