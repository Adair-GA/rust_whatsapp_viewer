use std::rc::Rc;

use rusqlite::{Row, Error, Statement};

use super::message::Message;


#[derive(Debug)]
pub struct Chat{
    pub chat_row_id: i32,
    // display_name: Option<String>,
    key: String,
    subject: Option<String>,
    creation_timestamp: Option<i64>,
    last_message_timestamp: i64,
    messages_sent: i32,
    messages_received: i32,
    messages: Vec<Rc<Message>>
}


impl Chat{
    pub fn from_row(r: &Row, count_stm: &mut Statement) -> Result<Chat,Error> {
        // let display_name = r.get::<usize,String>(1)?;
        let key = r.get::<usize,String>(0)?;
        let subject = r.get::<usize,Option<String>>(1)?;
        
        let creation_timestamp: Option<i64>;

        if let Some(timestamp) = r.get::<usize,Option<i64>>(2)?{
            if timestamp == 0 {
                creation_timestamp = None;
            }else {
                creation_timestamp = Some(timestamp);
            }
        }else {
            creation_timestamp = None;
        }

        let last_message_timestamp = r.get::<usize,i64>(3)?;
        let chat_row_id = r.get::<usize,i32>(4)?;

        let messages_received = Chat::messages_count(chat_row_id,count_stm)?;
        
        Ok(Chat{
            chat_row_id,
            // display_name,
            key,
            subject,
            creation_timestamp,
            last_message_timestamp,
            messages_sent: messages_received.0,
            messages_received: messages_received.1,
            messages: vec![],
        })
    }
    
    fn messages_count(chat_row_id: i32, count_stm: &mut Statement) -> Result<(i32,i32), Error> {

        let mut message_sent = 0;
        let mut message_received = 0;

        {
            let mut own_rows = count_stm.query([chat_row_id,1])?;
            if let Some(row) = own_rows.next()? {
                message_sent = row.get(0)?;
            }
        }

        let mut other_rows = count_stm.query([chat_row_id,0])?;


        if let Some(row) = other_rows.next()? {
            message_received = row.get(0)?;
        }

        Ok((message_sent, message_received))
    }
    
    pub fn retrieve_messages(&mut self, db_connection: &rusqlite::Connection) -> Result<(), Error> {
        let query = "SELECT message_view.key_id, message_view.from_me, message_view.text_data, \
        message_view.timestamp, message_location.latitude, message_location.longitude, message_view.thumb_image, message_quoted.key_id \
        FROM message_view \
        LEFT JOIN message_quoted on message_view._id = message_quoted.message_row_id \
        LEFT JOIN message_location on message_view._id = message_location.message_row_id \
        WHERE message_view.chat_row_id = ? \
        ORDER BY message_view.timestamp asc";

        let mut stm = db_connection.prepare(query)?;
        let mut result_rows = stm.query([self.chat_row_id])?;



        while let Some(row) = result_rows.next()? {
            let message:Message;

            {message = Message::from_row(&row, &self.messages)?};
            self.messages.push(Rc::new(message));
        }
        Ok(())
    }   

    pub fn get_message_by_index(&self, index: usize) -> Option<&Rc<Message>> {
        self.messages.get(index)
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}