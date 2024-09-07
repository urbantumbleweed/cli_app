use anyhow::Result;

use crate::models::DBState;

trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String,
}
