use cosmwasm_vm::{features_from_csv, Cache, CacheOptions, Checksum, Size};
use cosmwasm_vm::{testing::MockApi, InstanceOptions};
use hex;
use std::{fs, path::PathBuf};

use crate::stub;

use threadpool::ThreadPool;

pub unsafe fn do_recache(
    base_dir: &PathBuf,
    supported_features: &str,
    memory_cache_size: usize,
    instance_memory_limit: usize,
) {
    let options = CacheOptions {
        base_dir: base_dir.into(),
        supported_features: features_from_csv(supported_features),
        memory_cache_size: Size::mebi(memory_cache_size),
        instance_memory_limit: Size::mebi(instance_memory_limit),
    };

    // let cache: Cache<MockApi, stub::Storage, stub::Querier> = cosmwasm_vm::Cache::new(options).unwrap();

    let mut state_dir = PathBuf::new();
    state_dir.push(base_dir);
    state_dir.push("state");
    state_dir.push("wasm");
    let files = get_files(&state_dir);
    let instance_options = InstanceOptions {
        gas_limit: 3000000,
        print_debug: true,
    };

    println!("compiling {} target files", files.len());

    let pool = ThreadPool::new(128);
    let files: Vec<PathBuf> = files
        .into_iter()
        .filter(|f| f.as_path().is_file())
        .collect::<Vec<PathBuf>>();
    for f in files {
        let options = options.clone();
        let filename = f.file_name().unwrap().to_str().unwrap().to_string();

        pool.execute(move || {
            let mut checksum: [u8; 32] = [0; 32];
            hex::decode_to_slice(filename.as_bytes(), &mut checksum)
                .map_err(|e| panic!("{}", e))
                .unwrap();
            println!("compiling {:?}", filename);

            let checksum = Checksum::from(checksum);

            let backend = cosmwasm_vm::Backend {
                api: cosmwasm_vm::testing::MockApi::default(),
                storage: stub::Storage {},
                querier: stub::Querier {},
            };

            let cache: Cache<MockApi, stub::Storage, stub::Querier> =
                cosmwasm_vm::Cache::new(options).unwrap();
            cache
                .get_instance(&checksum, backend, instance_options)
                .map_err(|e| panic!("{}", e))
                .map(|instance| instance)
                .unwrap();
            ()
        });
    }
}

fn get_files(base_dir: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(base_dir)
        .map_err(|e| panic!("{}", e))
        .map(|res| {
            res.filter_map(|el| el.ok())
                .map(|el| el.path())
                .collect::<Vec<PathBuf>>()
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::do_recache;

    #[test]
    fn do_recache_works() {
        let mut base_dir = PathBuf::new();
        base_dir.push("./test");

        unsafe { do_recache(&base_dir, "stargate,staking,terra", 6000, 6000) }
    }
}
