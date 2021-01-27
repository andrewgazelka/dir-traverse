use std::collections::VecDeque;
use std::env;
use std::path::PathBuf;

fn up(path: &PathBuf, args: &[String]) -> Option<PathBuf> {
    let path_parts: Vec<_> = path.ancestors().collect();
    path_parts.into_iter()
        .enumerate()
        .rev()
        .filter_map(|(i, path)| {
            let last = path.iter().last().unwrap().to_str().unwrap().to_lowercase();
            let valid = args.iter().all(|arg| last.contains(arg));
            if valid {
                Some(path.to_path_buf())
            } else {
                None
            }
        })
        .next()
}

fn down(path: &PathBuf, args: &[String]) -> Option<PathBuf> {
    let mut paths = VecDeque::new();
    let mut args_iter = args.iter();
    let mut arg_on = args_iter.next().unwrap();
    paths.push_front(path.to_path_buf());
    while let Some(path_on) = paths.pop_back() {
        let lowercase = path_on.file_name().unwrap().to_str().unwrap().to_lowercase();
        let matches = lowercase.contains(arg_on);
        if matches {
            match args_iter.next() {
                None => return Some(path_on),
                Some(next_arg) => {
                    arg_on = next_arg;
                    paths.clear(); // we only consider descendants
                }
            }
        }
        let sub_paths = path_on.read_dir().unwrap()
            .filter_map(|dir_entry| {
                let sub_path = dir_entry.unwrap().path();
                sub_path.is_dir().then(|| sub_path)
            });

        sub_paths.for_each(|path| paths.push_front(path))
    }
    None
}

fn main() {
    let args: Vec<_> = env::args().skip(1).map(|str| str.to_lowercase()).collect();
    let mode = args.first().expect("must include mode");
    let sub_args = &args[1..];
    let path = env::current_dir().unwrap();
    let result = if mode == "d" { // down
        down(&path, sub_args)
    } else if mode == "u" {
        up(&path, sub_args)
    } else {
        panic!("invalid mode")
    };

    match result {
        None => println!("{}", path.to_str().unwrap()),
        Some(sub_path) => {
            println!("{}", sub_path.to_str().unwrap());
        }
    };
}
