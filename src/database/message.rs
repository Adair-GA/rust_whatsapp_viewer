use rusqlite::{Row, Error};

#[derive(Debug)]
struct Location {
    lat: f64,
    lon: f64
}

#[derive(Debug)]
struct Media {}

#[derive(Debug)]
pub struct Message<'a>{
    id: String,
    from_me: bool,
    text_data: Option<String>,
    timestamp: i64,
    quoted: Option<&'a Message<'a>>,
    location: Option<Location>,
    media: Option<Media>
}


impl<'a> Message<'a> {
    pub fn from_row(r: &Row, already_existing: &'a Vec<Message>) -> Result<Message<'a>,Error> {
        let id = r.get::<usize,String>(0)?;
        let from_me = r.get::<usize,i32>(1)? == 1;
        let text_data = r.get::<usize,Option<String>>(2)?;
        let timestamp = r.get::<usize,i64>(3)?;

        let location: Option<Location>;

        if let Some(lat) = r.get::<usize,Option<f64>>(4)? {
            let lon = r.get::<usize,f64>(5)?;
            location = Some(Location{
                lat,
                lon
            });
        }
        else {
            location = None;
        }

        let quoted = r.get::<usize,Option<String>>(7)?;

        let quoted = if let Some(quoted) = quoted {
            if let Some(quoted) = Message::get_quoted_message(quoted, already_existing) {
                Some(quoted)
            }
            else{
                None
            }
        }
        else {
            None
        };

        let media = Message::build_media(r);

        Ok(Message{
            id,
            from_me,
            text_data,
            timestamp,
            quoted,
            location,
            media
        })
    }
    

    fn get_quoted_message(quoted_id: String, already_existing: &'a Vec<Message>) -> Option<&'a Message<'a>> {
        let mut it = already_existing.iter().rev();
        while let Some(ms) = it.next_back() {
            if ms.id == quoted_id {
                return Some(&ms)
            }
        }
        None
    }

    fn build_media(r: &Row) -> Option<Media> {
        None
    }
}