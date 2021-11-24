#[macro_use]
extern crate diesel;

mod actions;
mod csv;
mod db;
mod db_schema;
mod errors;
mod models;

use crate::{
    actions::tagging::tagging,
    db::{postgres::Postgres, DBActions},
};
use actions::csv2db::csv2db;
use actions::http::http_server;
use serde::{Deserialize, Serialize};
use std::{
    env,
    sync::{Arc, Mutex},
};

#[derive(Default, Debug, Serialize, Deserialize)]
struct AppConfig {
    db_path: String,
    csv_source: String,
    root_www: String,
    port_www: u16,
    init_db_path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let switch = args.get(1).map(|e| e.as_str());

    let cfg: AppConfig = confy::load_path("./config.toml")?;

    match switch {
        Some("--http") => {
            let postgres_db = Postgres::from_config(db::DBConfig::rdbms_with_url(
                "postgres://postgres:mysecretpassword@172.17.0.2:5432/postgres".to_string(),
            ));
            let arc_db = Arc::new(Mutex::new(postgres_db));
            http_server(cfg.root_www, cfg.port_www, arc_db).await
        }
        Some("--db") => {
            let postgres_db = Postgres::from_config(db::DBConfig::rdbms_with_url(
                "postgres://postgres:mysecretpassword@172.17.0.2:5432/postgres".to_string(),
            ));
            let postgres_db = postgres_db.with_init_db_script(cfg.init_db_path);
            let arc_db = Arc::new(Mutex::new(postgres_db));
            csv2db(cfg.csv_source, arc_db.clone())?;
            tagging(arc_db).map(|_| ())
        }
        Some(arg) => Err(anyhow::anyhow!(format!("Invalid argument '{}'", arg))),
        _ => Err(anyhow::anyhow!("Missing argument")),
    }
}
