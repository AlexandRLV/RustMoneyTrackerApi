use std::fs;
use std::error::Error;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DbInfo {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: String,
}

const DB_INFO_FILE_PATH: &str = "db-info.json";

impl DbInfo {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let file_content = fs::read_to_string(DB_INFO_FILE_PATH)
            .map_err(|e| format!("Can't read file {}, {}", Path::new(DB_INFO_FILE_PATH).to_str().unwrap(), e))?;

        let db_info: DbInfo = serde_json::from_str(&file_content)
            .map_err(|e| format!("Can't parse json {}, {}", file_content, e))?;

        Ok(db_info)
    }

    pub fn connection_string(&self) -> String {
        format!("host={} port={} user={} password={} dbname={}", self.host, self.port, self.user, self.password, self.database)
    }
}