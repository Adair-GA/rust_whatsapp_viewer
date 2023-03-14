use std::{ io, };
pub mod database;

fn main() {
    let mut db = database::WaDatabase::new("msgstore.db").unwrap();
    db.populate_contacts("wa.db").unwrap();
    if let Err(res) = db.validate().map_err(|e| e.to_string()){
        panic!("{}", res)
    }
    let mut chats = db.get_chats().unwrap();
    
    //get a number from keyboard
    let mut input = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap();

    //get the chat with the given id
    let chat = chats.get_mut(&input).unwrap();
    println!("{:?}", chat);

    //get the messages of the chat
    db.get_messages_of_chat(chat).unwrap();
    let msg_cnt = chat.message_count();
    println!("Last message: {:?}", chat.get_message_by_index(msg_cnt - 1));

}
