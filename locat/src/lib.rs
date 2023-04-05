use std::net::IpAddr;

use rusqlite::Result;

struct Db {
    path: String,
}

impl Db {
    fn list(&self) -> Result<Vec<u64>, rusqlite::Error> {
        let conn = self.get_conn()?;
        let mut stmt = conn.prepare("SELECT count FROM analytics")?;
        let rows = stmt.query_map([], |row| row.get(0)?);
        let mut analytics = Vec::new();
        for row in rows {
            println!("{:?}", row?);
            //analytics.push(count);
        }
        Ok(analytics)
    }

    fn increment(&self) -> Result<(), rusqlite::Error> {
        let conn = self.get_conn().unwrap();
        conn.execute("UPDATE analytics SET count = 43", ())?;
        Ok(())
    }

    fn get_conn(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        let conn = rusqlite::Connection::open(&self.path).unwrap();
        self.migrate(&conn)?;
        Ok(conn)
    }

    fn migrate(&self, conn: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        // create analytics table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS analytics (
                    id INTEGER PRIMARY KEY NOT NULL,
                    count INTEGER NOT NULL DEFAULT 0
            );
             UPDATE analytics SET count = 0
            ",
            (),
        )?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("rusqlite error: {0}")]
    Rusqlite(#[from] rusqlite::Error),
}

pub struct Locat {
    analytics: Db,
}

impl Locat {
    pub fn new(_analytics_db_path: &str) -> Self {
        Self {
            analytics: Db {
                path: _analytics_db_path.to_string(),
            },
        }
    }

    /// Converts an address to an ISO 3166-1 alpha-2 country code
    pub fn ip_to_iso_code(&self, _addr: IpAddr) -> Option<&str> {
        None
    }

    /// Returns a map of country codes to number of requests
    pub fn get_analytics(&self) -> Vec<(String, u64)> {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::Db;

    struct RemoveOnDrop {
        path: String,
    }

    impl Drop for RemoveOnDrop {
        fn drop(&mut self) {
            _ = std::fs::remove_file(&self.path);
        }
    }

    #[test]
    fn test_db() {
        let db = Db {
            path: "/tmp/locat-test.db".to_string(),
        };
        //let _remove_on_drop = RemoveOnDrop {
        //    path: db.path.clone(),
        //};

        let analytics = db.list().unwrap();
        assert_eq!(analytics.len(), 0);

        db.increment().unwrap();
        db.increment().unwrap();
        let analytics = db.list().unwrap();
        assert_eq!(analytics.len(), 1);

        // db.increment().unwrap();
        // db.increment().unwrap();
        // let analytics = db.list().unwrap();
        // assert_eq!(analytics.len(), 2);
        // // contains US at count 2
        // assert!(analytics.contains(&("US".to_string(), 2)));
        // // contains FR at count 1
        // assert!(analytics.contains(&("FR".to_string(), 1)));
        // // doesn't contain DE
        // assert!(!analytics.contains(&("DE".to_string(), 0)));
    }
}
