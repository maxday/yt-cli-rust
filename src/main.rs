use std::fs::{read_dir, remove_file, File};
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use structopt::StructOpt;

fn main() -> Result<()> {
    let args = TodoCli::from_args();
    let folder = "./items/";
    match args.cmd {
        TodoCommand::Add(args) => add(folder, &args),
        TodoCommand::List(args) => list(folder, &args),
        TodoCommand::Remove(args) => remove(folder, &args),
    }
}

#[derive(StructOpt)]
struct TodoCli {
    #[structopt(subcommand)]
    cmd: TodoCommand,
}
#[derive(StructOpt)]
enum TodoCommand {
    #[structopt(about = "add an item", name = "add")]
    Add(AddOptions),
    #[structopt(about = "list all items", name = "list")]
    List(ListOptions),
    #[structopt(about = "remove an item", name = "remove")]
    Remove(RemoveOptions),
}
#[derive(StructOpt)]
struct AddOptions {
    #[structopt(long)]
    item: String,
}

#[derive(StructOpt)]
struct RemoveOptions {
    #[structopt(long)]
    item: String,
}

#[derive(StructOpt)]
struct ListOptions {
    #[structopt(long)]
    sorted: bool,
}

fn add(folder: &str, args: &AddOptions) -> Result<()> {
    let path = build_path_from_item(folder, &args.item);
    if Path::new(&path).exists() {
        Err(Error::new(
            ErrorKind::AlreadyExists,
            "an item with the same name already exists",
        ))
    } else {
        File::create(path)?;
        Ok(())
    }
}

fn remove(folder: &str, args: &RemoveOptions) -> Result<()> {
    let path = build_path_from_item(folder, &args.item);
    if !Path::new(&path).exists() {
        Err(Error::new(ErrorKind::NotFound, "could not find the item"))
    } else {
        remove_file(path)?;
        Ok(())
    }
}

fn list(folder: &str, args: &ListOptions) -> Result<()> {
    let mut paths: Vec<_> = read_dir(folder)?.filter_map(Result::ok).collect();
    if args.sorted {
        paths.sort_by_key(|dir| dir.file_name());
    }
    for path in paths {
        if let Ok(filename) = path.path().strip_prefix(folder) {
            println!("{}", filename.display());
        }
    }
    Ok(())
}

fn build_path_from_item(folder: &str, item: &str) -> String {
    folder.to_string() + item
}
