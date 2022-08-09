use rusqlite::{Connection, OptionalExtension, Result, Row};

use crate::model::{Feed, Link};

pub fn insert(conn: &Connection, feed: Feed) -> Result<()> {
    conn.execute(
        "insert into feed (id, links, title) values (?1, ?2, ?3)",
        (
            &feed.id,
            serde_json::to_string(&feed.links).unwrap(),
            &feed.title,
        ),
    )?;

    Ok(())
}

pub fn select_all(conn: &Connection) -> Result<Vec<Feed>> {
    let mut stmt = conn.prepare("select id, links, title from feed")?;
    let rows = stmt.query_map([], feed_mapper())?;
    Ok(rows.map(|it| it.unwrap()).collect())
}

pub fn select_by_id(conn: &Connection, id: String) -> Result<Option<Feed>> {
    let mut stmt = conn.prepare("select id, links, title from feed where id = ?")?;
    stmt.query_row([id], feed_mapper()).optional()
}

fn feed_mapper() -> fn(&Row) -> Result<Feed> {
    |row: &Row| -> Result<Feed> {
        let links: String = row.get(1)?;
        let links: Vec<Link> = serde_json::from_str(links.as_str()).unwrap();

        Ok(Feed {
            id: row.get(0)?,
            links,
            title: row.get(2)?,
        })
    }
}
