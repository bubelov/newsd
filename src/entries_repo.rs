use rusqlite::{Connection, OptionalExtension, Result, Row};

use crate::model::{Entry, Link};

pub fn insert(conn: &Connection, entry: Entry) {
    conn.execute(
        "insert into entry (content, id, links, published, summary, title, updated) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&entry.content, &entry.id, serde_json::to_string(&entry.links).unwrap(), &entry.published, &entry.summary, &entry.title, &entry.updated),
    )
    .unwrap();
}

pub fn select_all(conn: &Connection) -> Result<Vec<Entry>> {
    let query = "select content, id, links, published, summary, title, updated from entry";
    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map([], entry_mapper()).unwrap();
    Ok(rows.map(|it| it.unwrap()).collect())
}

pub fn select_by_id(conn: &Connection, id: String) -> Result<Option<Entry>> {
    let query =
        "select content, id, links, published, summary, title, updated from entry where id = ?";
    let mut stmt = conn.prepare(query)?;
    stmt.query_row([id], entry_mapper()).optional()
}

fn entry_mapper() -> fn(&Row) -> Result<Entry> {
    |row: &Row| -> Result<Entry> {
        let links: String = row.get(2)?;
        let links: Vec<Link> = serde_json::from_str(links.as_str()).unwrap();

        Ok(Entry {
            content: row.get(0)?,
            id: row.get(1)?,
            links,
            published: row.get(3)?,
            summary: row.get(4)?,
            title: row.get(5)?,
            updated: row.get(6)?,
        })
    }
}
