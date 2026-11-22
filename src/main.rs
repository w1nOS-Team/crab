use befli::{self, Core, ErrorsTypes, Value};

use crate::core::WatchFN;
mod core;

fn main() {
    let mut fl = befli::new();

    fl.add("ls");
    fl.get("ls").set_bio("Print a list of files");

    fl.add("copy");
    fl.get("copy")
        .set_bio("Copy file to a new path")
        .set_value(2);

    fl.add("slice");
    fl.get("slice")
        .set_bio("Print a slice of file text")
        .set_value(1);

    fl.add("move");
    fl.get("move")
        .set_bio("Move a file to a new path")
        .set_value(2);

    fl.add("read");
    fl.get("read").set_bio("Read file/files");

    fl.add("del");
    fl.get("del").set_bio("Delete files/symlinks/dirs");

    fl.add("mkdir");
    fl.get("mkdir").set_bio("Create dir/dirs");

    fl.add("mkfile");
    fl.get("mkfile").set_bio("Create file/files");

    fl.add("lines");
    fl.get("lines")
        .set_bio("Print file content with line numbers");

    fl.add("write");
    fl.get("write").set_bio("Overwrite file").set_value(1);

    fl.add("find");
    fl.get("find").set_bio("Find text in the file").set_value(1);

    fl.add("count");
    fl.get("count")
        .set_bio("Count occurrences of text in a file")
        .set_value(1);

    fl.add("mask");
    fl.get("mask").set_bio("Mask text in file").set_value(1);

    fl.add("len");
    fl.get("len").set_bio("Print length of file text");

    fl.add("len-lines");
    fl.get("len-lines")
        .set_bio("Print number of lines in a file");

    fl.add("reverse");
    fl.get("reverse").set_bio("Print reversed text of file");

    fl.add("reverse-lines");
    fl.get("reverse-lines")
        .set_bio("Print reversed lines of file");

    fl.add("watch");
    fl.get("watch")
        .set_bio("Watch file changes: <mode> <file>\n  -n: bell notification\n  -r: read\n  -rl: lines\n  -l: len\n  -ll: len-lines\n  -rv: reverse\n  -rvl: reverse-lines")
        .set_value(2);

    if fl.get_status("ls") {
        core::ls(&mut fl.search_subpath("ls"));
    }

    if fl.get_status("del") {
        core::del(&fl.search_subpath("del"));
    }

    if fl.get_status("read") {
        core::read(&fl.search_subpath("read"));
    }

    if fl.get_status("mkdir") {
        core::mkdir(&fl.search_subarg("mkdir"));
    }

    if fl.get_status("mkfile") {
        core::mkfile(&fl.search_subarg("mkfile"));
    }

    if fl.get_status("lines") {
        core::lines(&fl.search_subpath("lines"));
    }

    if fl.get_status("reverse") {
        core::reverse(&fl.search_subpath("reverse"));
    }

    if fl.get_status("reverse-lines") {
        core::reverse_lines(&fl.search_subpath("reverse-lines"));
    }

    match fl.parse("write") {
        Ok(v) => core::write(&v[0]),
        Err(e) => match e.types {
            ErrorsTypes::None => {}
            _ => println!("{}", e.value),
        },
    }

    match fl.parse("slice") {
        Ok(v) => core::slice(&v[0], &fl.search_subpath("slice")),
        Err(e) => match e.types {
            ErrorsTypes::None => {}
            _ => println!("{}", e.value),
        },
    }

    match fl.parse("watch") {
        Ok(v) => {
            let cmd = match v[0].as_str() {
                "-n" => WatchFN::Note,
                "-r" => WatchFN::Read,
                "-rl" => WatchFN::Lines,
                "-l" => WatchFN::Len,
                "-ll" => WatchFN::LenLines,
                "-rv" => WatchFN::Reverse,
                "-rvl" => WatchFN::ReverseLines,
                _ => return,
            };

            core::watch(&v[1], &cmd);
        }
        Err(e) => match e.types {
            ErrorsTypes::None => {}
            _ => println!("{}", e.value),
        },
    }

    match fl.parse("mask") {
        Ok(v) => core::mask(&v[0], &fl.search_subpath("mask")),
        Err(e) => match e.types {
            ErrorsTypes::None => {}
            _ => println!("{}", e.value),
        },
    }

    match fl.parse("find") {
        Ok(v) => {
            core::find(&v[0], &fl.search_subpath("find"));
        }
        Err(e) => match e.types {
            ErrorsTypes::None => {}
            _ => println!("{}", e.value),
        },
    }

    match fl.parse("move") {
        Ok(v) => {
            core::mv(&v);
        }
        Err(e) => match e.types {
            ErrorsTypes::None => {}
            _ => println!("{}", e.value),
        },
    }

    match fl.parse("copy") {
        Ok(v) => {
            core::copy(&v);
        }
        Err(e) => match e.types {
            ErrorsTypes::None => {}
            _ => println!("{}", e.value),
        },
    }

    match fl.parse("count") {
        Ok(v) => {
            core::count(&v[0], &fl.search_subpath("count"));
        }
        Err(e) => match e.types {
            ErrorsTypes::None => {}
            _ => println!("{}", e.value),
        },
    }

    if fl.get_status("len") {
        core::len(&fl.search_subpath("len"));
    }

    if fl.get_status("len-lines") {
        core::lenline(&fl.search_subpath("len-lines"));
    }

    fl.gen_help(befli::Colors::Red);
}
