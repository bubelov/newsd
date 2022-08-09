use rusqlite::{Connection, Result};

pub fn init() -> Result<()> {
    let conn = connect()?;

    if schema_ver(&conn)? == 0 {
        conn.execute(
            "create table feed (
            id text not null primary key,
            links text not null,
            title text not null
        )",
            (),
        )?;

        conn.execute(
            "create table entry (
            content text,
            id text not null primary key,
            links text not null,
            published text,
            summary text,
            title text not null,
            updated text not null
        )",
            (),
        )?;

        conn.execute_batch(&format!("PRAGMA user_version={}", 1))?;
    }

    Ok(())
}

pub fn connect() -> Result<Connection> {
    Connection::open("news.db")
}

fn schema_ver(conn: &Connection) -> Result<i16> {
    Ok(
        conn.query_row("select user_version from pragma_user_version", [], |row| {
            row.get(0)
        })?,
    )
}
