use rusqlite;
pub mod chat;
pub mod message;

pub struct WaDatabase {
    connection: rusqlite::Connection,
}

impl WaDatabase {
    pub fn new(path: &str) -> Result<WaDatabase, String> {
        let connection = rusqlite::Connection::open(path).map_err(|e| e.to_string())?;
        Ok(WaDatabase { 
            connection
         }
        )
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

    pub fn get_chats(&self) -> Result<Vec<chat::Chat>, String> {
        let mut res: Vec<chat::Chat> = Vec::new();
        
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
            chat::Chat::from_row(row, &mut message_count_stm).map(|chat| res.push(chat)).map_err(|e| e.to_string())?;
        }
        Ok(res)
        
    
    }

}