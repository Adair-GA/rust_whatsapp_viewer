use std::collections::HashMap;

use rusqlite::Connection;

pub struct Contact{
    pub jid: String,
    pub name: String,
    pub status: Option<String>
}

pub fn populate(db: Connection) -> Result<HashMap<String,Contact>, String> {
    let mut res: HashMap<String,Contact> = HashMap::new();
    let query = "SELECT jid, display_name, status, wa_name FROM wa_contacts";
    let mut statement = db.prepare(query).map_err(|e| e.to_string())?;
    let mut result = statement.query([]).map_err(|e| e.to_string())?;
    while let Some(row) = result.next().map_err(|e| e.to_string())? {
        let id: String = row.get(0).map_err(|e| e.to_string())?;
        let name: String;
        if let Some(display_name) = row.get::<usize,Option<String>>(1).map_err(|e| e.to_string())? {
            name = display_name;
        }else if let Some(wa_name) = row.get::<usize,Option<String>>(3).map_err(|e| e.to_string())? {
            name = wa_name;
        }else {
            continue;
        }
        let status = row.get(2).map_err(|e| e.to_string())?;
        res.insert(id.clone(), Contact{
            jid: id,
            name,
            status
        });
    }
    Ok(res)
}
