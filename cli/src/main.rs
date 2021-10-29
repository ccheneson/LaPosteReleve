mod actions;
mod csv;
mod db;
mod models;
mod errors;

use actions::csv2db::csv2db;
use actions::http::http_server;
use rusqlite::Connection;
use std::{env, sync::{Arc, Mutex}};
use serde::{Serialize, Deserialize};
use crate::{actions::tagging::tagging, db::{sqlite::SqliteDB, sqlite_connections::{self, remove_db_if_exist}}};


#[derive(Default, Debug, Serialize, Deserialize)]
struct AppConfig {
    db_path: String,
    csv_source: String,
    root_www: String,
    port_www: u16
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let args: Vec<String> = env::args().collect();    
    let switch = args.get(1).map(|e| e.as_str());

    let cfg: AppConfig = confy::load_path("./config.toml")?;
    let db_path = cfg.db_path.as_str();

    let arc_db = |conn: Connection|{        
        let sqlite_db = SqliteDB::new(conn);
        Arc::new(Mutex::new(sqlite_db))
    };
    
     match switch {
        Some("--http") => {
            let conn = sqlite_connections::from_file(db_path)?;
            let arc_db = arc_db(conn);
            http_server(cfg.root_www, cfg.port_www, arc_db).await
        },
        Some("--db")   => {
            remove_db_if_exist(db_path)?;
            let conn = sqlite_connections::from_file(db_path)?;
            let arc_db = arc_db(conn);
            csv2db(cfg.csv_source, arc_db.clone())?;
            tagging(arc_db).map(|_| ())
        },
        Some(arg) => Err(anyhow::anyhow!(format!("Invalid argument '{}'", arg))),
        _ => Err(anyhow::anyhow!("Missing argument"))
    }

}
