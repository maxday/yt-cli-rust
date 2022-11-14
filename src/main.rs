use std::fs::{read_dir, remove_file, File};
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use structopt::StructOpt;

fn main() -> Result<()> {
    let args = TodoCli::from_args();
    let folder = "./items/";
    match args.cmd {
        TodoCommand::Add(args) => add(folder, &args),
        TodoCommand::List(args) => list(folder, &args, &mut ConsoleLogger),
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

trait Logger {
    fn print(&mut self, value: String);
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn print(&mut self, value: String) {
        println!("{}", value);
    }
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

fn list(folder: &str, args: &ListOptions, logger: &mut dyn Logger) -> Result<()> {
    let mut paths: Vec<_> = read_dir(folder)?.filter_map(Result::ok).collect();
    if args.sorted {
        paths.sort_by_key(|dir| dir.file_name());
    }
    for path in paths {
        if let Ok(filename) = path.path().strip_prefix(folder) {
            logger.print(filename.display().to_string());
        }
    }
    Ok(())
}

fn build_path_from_item(folder: &str, item: &str) -> String {
    folder.to_string() + item
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;

    #[derive(Default)]
    struct TestLogger(Vec<String>);
    impl Logger for TestLogger {
        fn print(&mut self, value: String) {
            self.0.push(value);
        }
    }

    #[test]
    fn build_path_from_item_test() {
        assert_eq!("folder/item", build_path_from_item("folder/", "item"));
    }

    #[test]
    fn add_test() {
        let folder = "./test_add/";
        fs::create_dir(folder).unwrap();
        let mut test_logger = TestLogger::default();
        let test_add_options = AddOptions {
            item: String::from("abc"),
        };
        let test_list_options = ListOptions { sorted: true };
        add(folder, &test_add_options).unwrap();
        list(folder, &test_list_options, &mut test_logger).unwrap();
        fs::remove_dir_all(folder).unwrap();
        assert_eq!(1, test_logger.0.len());
        assert_eq!("abc", test_logger.0[0]);
    }

    #[test]
    fn list_test() {
        let folder = "./test_list/";
        fs::create_dir(folder).unwrap();
        File::create(build_path_from_item(folder, "abc")).unwrap();
        File::create(build_path_from_item(folder, "zzz")).unwrap();
        File::create(build_path_from_item(folder, "ccc")).unwrap();
        let test_options = ListOptions { sorted: true };
        let mut test_logger = TestLogger::default();
        list(folder, &test_options, &mut test_logger).unwrap();
        fs::remove_dir_all(folder).unwrap();
        assert_eq!(3, test_logger.0.len());
        assert_eq!("abc", test_logger.0[0]);
        assert_eq!("ccc", test_logger.0[1]);
        assert_eq!("zzz", test_logger.0[2]);
    }

    #[test]
    fn remove_test() {
        let folder = "./test_remove/";
        fs::create_dir(folder).unwrap();
        File::create(build_path_from_item(folder, "abc")).unwrap();
        let mut test_logger = TestLogger::default();
        let test_remove_options = RemoveOptions {
            item: String::from("abc"),
        };
        let test_list_options = ListOptions { sorted: true };
        remove(folder, &test_remove_options).unwrap();
        list(folder, &test_list_options, &mut test_logger).unwrap();
        fs::remove_dir_all(folder).unwrap();
        assert_eq!(0, test_logger.0.len());
    }
}
