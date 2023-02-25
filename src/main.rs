use rocksdb::{DB, Options};

fn main() {

    let path = "/home/hatio/Workspace/Rust/rockdb-experiment/db";

    let mut options = Options::default();

    let db = DB::open_default(path).unwrap();
    db.put(b"my key", b"my value").unwrap();



    match db.get(b"my key") {
        Ok(Some(value)) => println!("retrieved value {}", 122),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }
    db.delete(b"my key").unwrap();

    println!("Hello, world!");
}
