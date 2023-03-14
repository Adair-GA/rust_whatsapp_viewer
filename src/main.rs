pub mod database;

fn main() {
    let db = database::WaDatabase::new("msgstore.db").unwrap();
    if let Err(res) = db.validate().map_err(|e| e.to_string()){
        panic!("{}", res)
    }
    let chats = db.get_chats().unwrap();
    for c in chats {
        println!("{:?}", c)
    }
}
