use std::{env, path::PathBuf};

mod rebuild;

fn main() {
    let paths: Vec<PathBuf> = env::args()
        .skip(1)
        .map(|arg| PathBuf::from(arg))
        .collect();

    let src = paths.get(0).unwrap();
    let dest = paths.get(1).unwrap();

    rebuild::do_rebuild(src, dest);
}
