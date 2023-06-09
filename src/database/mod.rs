use std::collections::HashMap;

use rusqlite;

use self::{chat::Chat, contacts::Contact};
pub mod chat;
pub mod message;
pub mod contacts;

pub struct WaDatabase {
    connection: rusqlite::Connection,
    contacts: Option<HashMap<String,Contact>>
}

impl WaDatabase {
    pub fn new(messages_path: &str) -> Result<WaDatabase, String> {
        let connection = rusqlite::Connection::open(messages_path).map_err(|e| e.to_string())?;
        
        Ok(WaDatabase { 
            connection,
            contacts: None
         }
        )
    }

    pub fn populate_contacts(&mut self, path: &str) -> Result<(), String> {
        if let Ok(conn) = rusqlite::Connection::open(path){
            let contacts = contacts::populate(conn).map_err(|e| e.to_string())?;
            self.contacts = Some(contacts);
            Ok(())
        }else {
            Err(format!("Could not open database at {}", path))
        }
    }

    fn has_table(&self, table_name: &str) -> Result<bool, String> {
        let query = "SELECT count(*) FROM sqlite_master WHERE type='table' AND name=?";
        let mut statement = self.connection.prepare(query).map_err(|e| e.to_string())?;
        let mut result = statement.query([table_name]).map_err(|e| e.to_string())?;
        let row = result.next().map_err(|e| e.to_string())?;
        if let Some(row) = row {
            let count: i32 = row.get(0).map_err(|e| e.to_string())?;
            return Ok(count > 0);
        }else {
            return Ok(false);
        }
    }

    pub fn validate(&self) -> Result<(), String>{
        let tables = vec!["message_thumbnail", "message_quoted", "message_link"];
        for table in tables {
            if !self.has_table(table)? {
                return Err(format!("Table {} not found", table));
            }
        }
        Ok(())
    }

    pub fn get_chats(&self) -> Result<HashMap<i32,Chat>, String> {
        let mut res: HashMap<i32,Chat> = HashMap::new();
        
        let query = 
        "SELECT chat_view.raw_string_jid, chat_view.subject, chat_view.created_timestamp, max(message.timestamp), chat_view._id \
        FROM chat_view \
        LEFT OUTER JOIN message on message.chat_row_id = chat_view._id \
        WHERE chat_view.hidden = 0 \
        GROUP BY chat_view.raw_string_jid, chat_view.subject, chat_view.created_timestamp \
        ORDER BY max(message.timestamp) desc
        ";

        let message_count_query = "SELECT count(_id) from message_view where chat_row_id = ? and from_me = ?";
        let mut message_count_stm = self.connection.prepare(message_count_query).map_err(|e| e.to_string())?;


        let mut result_stm = self.connection.prepare(query).map_err(|e| e.to_string())?;
        let mut result_rows = result_stm.query([]).map_err(|e| e.to_string())?;
        while let Some(row) = result_rows.next().map_err(|e| e.to_string())? {
            chat::Chat::from_row(row, &mut message_count_stm, &self.contacts).map(|chat| res.insert(chat.chat_row_id, chat)).map_err(|e| e.to_string())?;
        }
        Ok(res)
        
    
    }

    pub fn get_messages_of_chat(&self, chat: &mut Chat) -> Result<(), String>{
        chat.retrieve_messages(&self.connection).map_err(|e| e.to_string())
    }

}