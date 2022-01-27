use rusqlite::{Connection, Result};

mod cli;
mod task;
mod util;

fn main() -> Result<()> {
    let conn = Connection::open("./dev.db")?;
    task::create_table_if_not_exists(&conn)?;
    let config = util::Config { conn };

    let app = cli::build("task");
    let matches = app.get_matches();

    cli::run(&matches, &config).unwrap();

    Ok(())
}
