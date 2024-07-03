use clap::Parser;
use std::path::{Path, PathBuf};

enum ExitCode {
    UsageError,
    TargetExists,
    InternalError,
}

type Result<T> = std::result::Result<T, ExitCode>;

#[derive(Clone, Copy)]
enum TargetExistsPolicy {
    Abort,
    Override,
    Merge,
}

fn runtime() -> Result<()> {
    let cli = CommandLine::parse();

    let p = if cli.r#override {
        TargetExistsPolicy::Override
    } else if cli.merge {
        TargetExistsPolicy::Merge
    } else {
        TargetExistsPolicy::Abort
    };

    if !cli.into {
        match cli.paths.as_slice() {
            [src, dst] => {
                copy_or_move_item(src, &dst, cli.copy, p)?;
            }
            _ => {
                eprint!(include_str!("error_path_len.txt"), count = cli.paths.len());
                return Err(ExitCode::UsageError);
            }
        }
    } else {
        if cli.paths.len() < 2 {
            eprintln!("movefile: No file to move");
            return Err(ExitCode::UsageError);
        }

        match cli.paths.as_slice() {
            [files @ .., target] => {
                for src in files {
                    let Some(file_name) = src.file_name() else {
                        eprintln!(
                            "movefile: No file name of '{}', can't move into directory",
                            src.display()
                        );
                        return Err(ExitCode::UsageError);
                    };

                    let dst = target.join(file_name);

                    copy_or_move_item(src, &dst, cli.copy, p)?;
                }
            }
            _ => {
                eprintln!("movefile: No file to move");
                return Err(ExitCode::UsageError);
            }
        }
    }

    Ok(())
}

fn copy_or_move_item(src: &Path, dst: &Path, copy: bool, p: TargetExistsPolicy) -> Result<()> {
    if copy {
        copy_item(src, dst, p)?;
    } else {
        move_item(src, dst, p)?;
    }
    Ok(())
}

fn move_item(src: &Path, dst: &Path, p: TargetExistsPolicy) -> Result<()> {
    match p {
        TargetExistsPolicy::Override => (),
        TargetExistsPolicy::Abort => {
            if dst.exists() {
                eprintln!("movefile: Target file '{}' exists", dst.display());
                return Err(ExitCode::TargetExists);
            }
        }
        TargetExistsPolicy::Merge => {
            eprintln!("movefile: TODO: move item with merge policy");
            return Err(ExitCode::InternalError);
        }
    }

    std::fs::rename(src, dst).map_err(|err| {
        eprintln!("movefile: Failed to move file: {err}");
        ExitCode::InternalError
    })?;
    Ok(())
}

fn copy_item(src: &Path, dst: &Path, p: TargetExistsPolicy) -> Result<()> {
    match p {
        TargetExistsPolicy::Abort => {
            if dst.exists() {
                eprintln!("movefile: Target file '{}' exists", dst.display());
                return Err(ExitCode::TargetExists);
            }
        }
        TargetExistsPolicy::Override => (),
        TargetExistsPolicy::Merge => {
            eprintln!("movefile: TODO: move item with merge policy");
            return Err(ExitCode::InternalError);
        }
    }

    if src.is_file() {
        std::fs::copy(src, dst).map_err(|err| {
            eprintln!("movefile: Failed to copy/move file: {err}");
            ExitCode::InternalError
        })?;
    } else if src.is_dir() {
        if !dst.exists() {
            copy_tree(src, dst, p)?;
        } else {
            eprintln!(
                "movefile: Can't copy directory '{}' to '{}', target is exists",
                src.display(),
                dst.display()
            );
            return Err(ExitCode::TargetExists);
        }
    } else {
        eprintln!(
            "movefile: Can't copy '{}', not a file or directory",
            src.display()
        );
        return Err(ExitCode::InternalError);
    }

    Ok(())
}

fn copy_tree(src: &Path, dst: &Path, p: TargetExistsPolicy) -> Result<()> {
    std::fs::create_dir_all(dst).map_err(|err| {
        eprintln!(
            "movefile: Failed to create directory '{}': {err}",
            dst.display()
        );
        ExitCode::InternalError
    })?;

    for item in std::fs::read_dir(src).map_err(|err| {
        eprintln!(
            "movefile: Failed to open directory '{}': {err}",
            src.display()
        );
        ExitCode::InternalError
    })? {
        let item = item.map_err(|err| {
            eprintln!(
                "movefile: Failed to open directory '{}': {err}",
                src.display()
            );
            ExitCode::InternalError
        })?;

        let file_type = item.file_type().map_err(|err| {
            eprintln!(
                "movefile: Failed to read file type of '{}': {err}",
                src.display()
            );
            ExitCode::InternalError
        })?;

        if file_type.is_file() {
            copy_item(&item.path(), &dst.join(item.file_name()), p)?;
        } else if file_type.is_dir() {
            copy_tree(&item.path(), &dst.join(item.file_name()), p)?;
        } else {
            eprintln!(
                "movefile: Failed to move file '{}', This is not directory or file",
                src.display()
            );
            return Err(ExitCode::InternalError);
        }
    }

    Ok(())
}

fn main() {
    std::process::exit(match runtime() {
        Ok(_) => 0,
        Err(ExitCode::UsageError) => 1,
        Err(ExitCode::TargetExists) => 2,
        Err(ExitCode::InternalError) => 100,
    })
}

/// Copy or move (rename) files.
///
/// EXIT CODE:
/// - 1     Usage error
/// - 2     Target exists
/// - 100   Internal error
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

    /// Override target if exists.
    #[clap(short, long)]
    r#override: bool,

    /// Merge source and target directory.
    #[clap(short, long)]
    merge: bool,

    paths: Vec<PathBuf>,
}
