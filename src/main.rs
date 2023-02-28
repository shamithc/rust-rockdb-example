use rocksdb::{DB, Options};
use std::collections::HashMap;
use serde::{de::DeserializeOwned, Serialize};




pub struct Ordebook {
    asks: HashMap<String, String>,
    bids: HashMap<String, String>,
}

impl Ordebook {
    pub fn new() -> Ordebook {
        Ordebook {
            asks: HashMap::new(),
            bids: HashMap::new()
        }
    }

    pub fn add_asks(&mut self) {
        self.asks.insert("abc".to_string(), "v".to_string());
    }
}


fn main() {


 
    let path = "/home/hatio/Workspace/Rust/rockdb-experiment/db";
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.set_max_open_files(10000);
    opts.set_use_fsync(false);
    opts.set_bytes_per_sync(8388608);
    // opts.set_disable_data_sync(false);
    // opts.set_block_cache_size_mb(1024);
    opts.set_table_cache_num_shard_bits(6);
    opts.set_max_write_buffer_number(32);
    opts.set_write_buffer_size(536870912);
    opts.set_target_file_size_base(1073741824);
    opts.set_min_write_buffer_number_to_merge(4);
    opts.set_level_zero_stop_writes_trigger(2000);
    opts.set_level_zero_slowdown_writes_trigger(0);
    // opts.set_compaction_style(DBUniversalCompaction);
    opts.set_max_background_compactions(4);
    opts.set_max_background_flushes(4);
    opts.set_disable_auto_compactions(true);

    let db = DB::open(&opts, path).unwrap();

    let mut ob = Ordebook::new();
    ob.add_asks();

    db.put(b"my key", ob.i).unwrap();



    match db.get(b"my key") {
        Ok(Some(value)) => println!("retrieved value {}", 122),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }
    db.delete(b"my key").unwrap();



    // let path = "/home/hatio/Workspace/Rust/rockdb-experiment/db";
    // let mut cf_opts = Options::default();
    // cf_opts.set_max_write_buffer_number(16);
    // let cf = ColumnFamilyDescriptor::new("cf1", cf_opts);

    // let mut db_opts = Options::default();
    // db_opts.create_missing_column_families(true);
    // db_opts.create_if_missing(true);
    // {
    //     let db = DB::open_cf_descriptors(&db_opts, path, vec![cf]).unwrap();
    // }
    // let _ = DB::destroy(&db_opts, path);
 
    // // let path = "/home/hatio/Workspace/Rust/rockdb-experiment/db";

    // // let mut options = Options::default();

    // DB::open(opts, path);

    // // let db = DB::open_default(path).unwrap();
    // // db.put(b"my key", b"my value").unwrap();



    // // match db.get(b"my key") {
    // //     Ok(Some(value)) => println!("retrieved value {}", 122),
    // //     Ok(None) => println!("value not found"),
    // //     Err(e) => println!("operational problem encountered: {}", e),
    // // }
    // // db.delete(b"my key").unwrap();

    println!("Hello, world!");
}
