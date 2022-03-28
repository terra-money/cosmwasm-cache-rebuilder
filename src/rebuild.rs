use std::{fs::{self, File}, path::PathBuf, io::Read};
use wasmer::{Module, Singlepass, Universal, Store};

pub fn do_rebuild<'a>(src: &PathBuf, dest: &PathBuf) {
    let config = Singlepass::default();
    let engine = Universal::new(config).engine();
    let store = Store::new(&engine);

    let files = fs::read_dir(src)
        .map_err(|e| panic!("{}", e))
        .map(|res| {
            res
                .filter_map(|el| el.ok())
                .map(|el| el.path().canonicalize())
                .filter_map(|el| el.ok())
                .collect::<Vec<PathBuf>>()
        })
        .unwrap();

    println!("compiling {} target files", files.len());

    files
        .iter()
        .filter(|p| p.as_path().is_file())
        .map(|p| {
            let mut code = Vec::<u8>::new();
            let mut cached_file = PathBuf::new();
            let filename = p.file_name().unwrap();
            cached_file.push(dest);
            cached_file.push(filename);

            println!("compiling {:?} into {:?}...", filename, cached_file);

            File::open(p.as_path())
                .map(|mut blob| blob.read_to_end(&mut code))
                .map_err(|e| panic!("{}", e));

            Module::new(&store, code)
                .map(|module| module.serialize_to_file(cached_file))
                .map_err(|e| panic!("{}", e));
            
            ()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::do_rebuild;

    #[test]
    fn do_rebuild_works() {
        let mut src = PathBuf::new();
        src.push("./test");

        let mut dest = PathBuf::new();
        dest.push("./test");
        dest.push("./cached");

        do_rebuild(&src, &dest);
    }
}
