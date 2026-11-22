use befli::PathGroup;
use color_print::{cformat, cprintln};
use rustyline::{Cmd, Editor, EventHandler, KeyCode, KeyEvent, Modifiers, history::DefaultHistory};
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::Duration;

fn open(path: &PathBuf) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn writes(path: &PathBuf, contents: &str) -> Result<(), io::Error> {
    fs::write(path, contents)
}

fn to_path(path: &String) -> Result<PathBuf, io::Error> {
    let p = PathBuf::from(path);
    if p.exists() {
        Ok(p)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }
}

fn pop(mut v: String) -> String {
    let txt = if v.ends_with("\n") {
        v.pop();
        v
    } else {
        v
    };

    txt
}

pub fn ls(args: &mut PathGroup) {
    if args.len() == 0 {
        args.push(env::current_dir().unwrap());
    }

    for i in args {
        let name = match i.to_str().unwrap() {
            "/" => "/",
            _ => i.file_name().unwrap().to_str().unwrap(),
        };
        cprintln!("<r>{}</>:", &name);

        for f in fs::read_dir(&i).unwrap() {
            let file = f.unwrap();
            let tmp = file.file_name();
            let name = tmp.to_str().unwrap();

            match file.metadata().unwrap() {
                x if x.is_file() => println!("{}{}", " ".repeat(3), &name),
                x if x.is_dir() => cprintln!("{}<r>{}</>/", " ".repeat(3), &name),
                x if x.is_symlink() => cprintln!("{}<m>{}</>*", " ".repeat(3), &name),
                _ => {}
            }
        }
    }
}

pub fn read(args: &PathGroup) {
    for i in args {
        match open(i) {
            Ok(v) => {
                cprintln!("<r>{}</>:", i.file_name().unwrap().to_str().unwrap());
                for i in pop(v).lines() {
                    println!("{}{}", " ".repeat(3), &i)
                }
            }
            Err(e) => cprintln!("<r>{}</>", e),
        }
    }
}

pub fn lines(args: &PathGroup) {
    for i in args {
        match open(i) {
            Ok(v) => {
                cprintln!("<r>{}:</>", i.file_name().unwrap().to_str().unwrap());
                for (i, v) in pop(v).lines().enumerate() {
                    cprintln!("{}<r>[{}]</> {}", " ".repeat(3), i + 1, &v);
                }
            }
            Err(e) => cprintln!("<r>{}</>", e),
        }
    }
}

pub fn write(args: &String) {
    let file = to_path(args);
    match file {
        Ok(v) => {
            let f = open(&v);
            match f {
                Ok(val) => {
                    let mut x: Editor<(), DefaultHistory> = Editor::new().unwrap();
                    x.bind_sequence(
                        KeyEvent(KeyCode::Enter, Modifiers::NONE),
                        EventHandler::Simple(Cmd::Newline),
                    );
                    x.bind_sequence(
                        KeyEvent(KeyCode::Char('S'), Modifiers::CTRL),
                        EventHandler::Simple(Cmd::AcceptLine),
                    );
                    x.bind_sequence(
                        KeyEvent(KeyCode::Tab, Modifiers::NONE),
                        EventHandler::Simple(Cmd::Insert(1, " ".to_string().repeat(4))),
                    );
                    let res = match x.readline_with_initial("", (&val, "")) {
                        Ok(v) => v,
                        Err(_) => return (),
                    };

                    match writes(&v, &res) {
                        Ok(_) => {}
                        Err(e) => cprintln!("<r>{}</>", e),
                    };
                }
                Err(e) => cprintln!("<r>{}</>", e),
            }
        }
        Err(e) => cprintln!("<r>{}</>", e),
    }
}

pub fn find(find: &str, args: &PathGroup) {
    for i in args {
        match open(i) {
            Ok(val) => {
                cprintln!("<r>{}:</>", i.file_name().unwrap().to_str().unwrap());
                for (i, v) in val.lines().enumerate() {
                    if v.to_lowercase().contains(&find.to_lowercase()) {
                        let lower = v.to_lowercase();
                        let mut str = v.to_string();

                        let x: Vec<(usize, &str)> =
                            lower.match_indices(&find.to_lowercase()).collect();

                        for id in &x {
                            let ids = id.0..id.0 + find.len();
                            str.replace_range(ids.clone(), &cformat!("<r>{}</>", &v[ids.clone()]));
                        }
                        cprintln!("{}<r>[{}]</> {}", " ".repeat(3), i + 1, str);
                    }
                }
            }
            Err(e) => cprintln!("<r>{}</>", e),
        }
    }
}

pub fn count(find: &str, args: &PathGroup) {
    for i in args {
        match open(i) {
            Ok(val) => {
                let mut count: usize = 0;
                let ch: Vec<char> = val.chars().collect();

                let searchr: Vec<char> = find.chars().collect();
                let mut change: usize = 0;

                let lens = searchr.len();

                if lens != 0 {
                    for i in 0..ch.len() {
                        for v in 0..lens {
                            if i + (lens - 1) < ch.len() && ch[i + v] == searchr[v] {
                                change += 1;
                            }
                        }
                        if change == lens {
                            count += 1;
                        }
                        change = 0;
                    }
                }
                cprintln!(
                    "<r>{}:</> {}",
                    i.file_name().unwrap().to_str().unwrap(),
                    count
                );
            }
            Err(e) => cprintln!("<r>{}</>", e),
        }
    }
}

pub fn len(args: &PathGroup) {
    for p in args {
        match open(p) {
            Ok(val) => {
                cprintln!(
                    "<r>{}:</> {}",
                    p.file_name().unwrap().to_str().unwrap(),
                    val.chars().count()
                );
            }
            Err(e) => cprintln!("<r>{}</>", e),
        }
    }
}

pub fn lenline(args: &PathGroup) {
    for p in args {
        match open(p) {
            Ok(val) => {
                cprintln!(
                    "<r>{}:</> {}",
                    p.file_name().unwrap().to_str().unwrap(),
                    pop(val).lines().count()
                );
            }
            Err(e) => cprintln!("<r>{}</>", e),
        }
    }
}

pub fn mkdir(args: &PathGroup) {
    for i in args {
        match fs::create_dir_all(i) {
            Ok(_) => cprintln!("<r>{} created</>", i.to_str().unwrap()),
            Err(e) => cprintln!("<r>{}</>", e),
        };
    }
}

pub fn mkfile(args: &PathGroup) {
    for i in args {
        match fs::File::create(i) {
            Ok(_) => cprintln!("<r>{} created</>", i.to_str().unwrap()),
            Err(e) => cprintln!("<r>{}</>", e),
        };
    }
}

pub fn del(args: &PathGroup) {
    for i in args {
        match fs::metadata(i) {
            Ok(v) => match v {
                x if x.is_file() || x.is_symlink() => {
                    fs::remove_file(i).unwrap();
                    cprintln!("<r>Delete file: {}</>", i.to_str().unwrap());
                }
                x if x.is_dir() => {
                    fs::remove_dir_all(i).unwrap();
                    cprintln!("<r>Delete dir: {}</>", i.to_str().unwrap());
                }
                _ => {}
            },
            Err(e) => cprintln!("<r>{}</>", e),
        };
    }
}

pub fn slice(find: &str, args: &PathGroup) {
    for i in args {
        let rng: Vec<&str> = find.split("..").collect();
        let s1: usize = rng[0].trim().parse().expect("Not a number");
        let s2: usize = rng[1].trim().parse().expect("Not a number");

        let mut result = String::new();

        match open(i) {
            Ok(val) => {
                let count = val.chars().count();
                if s1 > count || s2 > count {
                    cprintln!("<r>Failed slicing:</> max is <r>{}</>", count)
                } else {
                    for (i, v) in val.chars().enumerate() {
                        if (s1..=s2).contains(&i) {
                            result.push(v);
                        } else if i == s2 {
                            break;
                        }
                    }

                    cprintln!("<r>{}:</>", i.file_name().unwrap().to_str().unwrap());
                    println!("{}{}", " ".repeat(3), result);
                }
            }
            Err(e) => cprintln!("<r>{}</>", e),
        }
    }
}

pub fn reverse(args: &PathGroup) {
    for i in args {
        match open(i) {
            Ok(val) => {
                cprintln!("<r>{}</>", i.file_name().unwrap().to_str().unwrap());
                for v in val.lines() {
                    let mut str: String = String::new();
                    let string: Vec<char> = v.chars().collect();
                    for c in string.iter().rev() {
                        str.push(*c);
                    }
                    println!("{}{}", " ".repeat(3), str)
                }
            }
            Err(e) => cprintln!("<r>{}</>", e),
        }
    }
}

pub fn reverse_lines(args: &PathGroup) {
    for i in args {
        match open(i) {
            Ok(val) => {
                cprintln!("<r>{}</>", i.file_name().unwrap().to_str().unwrap());
                for v in val.lines().rev() {
                    println!("{}{}", " ".repeat(3), v)
                }
            }
            Err(e) => cprintln!("<r>{}</>", e),
        }
    }
}

pub fn copy(args: &Vec<String>) {
    let path1 = match to_path(&args[0]) {
        Ok(v) => v,
        Err(e) => {
            cprintln!("<r>{}</>", e);
            return;
        }
    };

    if path1.is_file() || path1.is_symlink() {
        match fs::copy(path1, &args[1]) {
            Ok(_) => cprintln!("<r>Copy {} to {}</>", args[0], args[1]),
            Err(e) => cprintln!("<r>{}</>", e),
        }
    } else if path1.is_dir() {
        for v in path1.read_dir().unwrap() {
            let x = v.unwrap();
            let name = format!("{}{}", &args[1], x.file_name().to_str().unwrap());
            match fs::copy(x.path(), &name) {
                Ok(_) => cprintln!("<r>Copy {} to {}</>", x.path().to_str().unwrap(), &name),
                Err(e) => cprintln!("<r>{}</>", e),
            }
        }
    }
}

pub fn mv(args: &Vec<String>) {
    let path1 = match to_path(&args[0]) {
        Ok(v) => v,
        Err(e) => {
            cprintln!("<r>{}</>", e);
            return;
        }
    };

    if path1.is_file() || path1.is_symlink() {
        match fs::rename(path1, &args[1]) {
            Ok(_) => cprintln!("<r>Move {} to {}</>", args[0], args[1]),
            Err(e) => cprintln!("<r>{}</>", e),
        }
    } else if path1.is_dir() {
        for v in path1.read_dir().unwrap() {
            let x = v.unwrap();
            let name = format!("{}{}", &args[1], x.file_name().to_str().unwrap());
            match fs::rename(x.path(), &name) {
                Ok(_) => cprintln!("<r>Move {} to {}</>", x.path().to_str().unwrap(), &name),
                Err(e) => cprintln!("<r>{}</>", e),
            }
        }
    }
}

pub fn mask(mask: &str, args: &PathGroup) {
    for i in args {
        match open(i) {
            Ok(val) => {
                let maska = "*".repeat(mask.chars().count());

                match writes(i, &val.replace(mask, &maska)) {
                    Ok(_) => {
                        cprintln!("<r>{}:</>", i.to_str().unwrap());
                        print!("{}", " ".repeat(3));
                        find(&maska, &vec![i.clone()]);
                    }
                    Err(e) => cprintln!("<r>{}</>", e),
                }
            }
            Err(e) => cprintln!("<r>{}</>", e),
        }
    }
}

#[derive(Clone)]
pub enum WatchFN {
    Note,
    Read,
    Lines,
    Len,
    LenLines,
    Reverse,
    ReverseLines,
}

pub fn watch(arg: &String, cmd: &WatchFN) {
    let path = match to_path(arg) {
        Ok(v) => v,
        Err(_) => return,
    };

    let x = open(&path).expect("");
    let flag = Arc::new(AtomicBool::new(true));

    let fl = Arc::clone(&flag);
    ctrlc::set_handler(move || {
        fl.store(false, std::sync::atomic::Ordering::Relaxed);
    })
    .unwrap();

    let handle = thread::spawn({
        let mut x = x.clone();
        let enums = cmd.clone();

        move || {
            while flag.load(std::sync::atomic::Ordering::Relaxed) {
                if open(&path).expect("") != x {
                    let vec = vec![path.clone()];
                    match enums {
                        WatchFN::Note => {
                            println!(
                                "\x07File changed: {}",
                                path.file_name().unwrap().to_str().unwrap()
                            );
                        }
                        WatchFN::Read => read(&vec),
                        WatchFN::Lines => lines(&vec),
                        WatchFN::Len => len(&vec),
                        WatchFN::LenLines => lenline(&vec),
                        WatchFN::Reverse => reverse(&vec),
                        WatchFN::ReverseLines => reverse_lines(&vec),
                    }
                    x = open(&path).expect("");
                }
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    handle.join().unwrap();
}
