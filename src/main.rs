use std::{env, path::PathBuf};

mod recache;
mod stub;

fn main() {
    let paths: Vec<PathBuf> = env::args()
        .skip(1)
        .map(|arg| PathBuf::from(arg))
        .collect();

    let base_dir = paths.get(0).unwrap();

    unsafe { recache::do_recache(
        base_dir,
        "stargate,staking,terra",
        6000,
        6000,
    ) }
}
