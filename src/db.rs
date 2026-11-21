use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Database {
    connection: Connection,
}

impl Database {
    /// Opens (or creates) a SQLite database at `path`.
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let connection = Connection::open(path)?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS products (
                barcode TEXT PRIMARY KEY,
                name    TEXT NOT NULL
        )",
        [],
        )?;
        Ok(Self { connection })
    }
    /// Adds a product, or updates its name if the barcode already exists.
    pub fn save_product(&self, barcode: &str, name: &str) -> Result<()> {
        self.connection.execute(
            "INSERT INTO products (barcode, name)
        VALUES (?1, ?2)
        ON CONFLICT(barcode) DO UPDATE SET name = excluded.name",
                                params![barcode, name],
        )?;
        Ok(())
    }
    /// Returns the product name, or `None` if the barcode isn't in the database.
    pub fn get_name(&self, barcode: &str) -> Result<Option<String>> {
        let mut statement = self
        .connection
        .prepare("SELECT name FROM products WHERE barcode = ?1")?;
        let mut rows = statement.query([barcode])?;
        match rows.next()? {
            Some(row) => Ok(Some(row.get(0)?)),
            None => Ok(None),
        }
    }
    /// Deletes a product. Returns `true` if a row was deleted.
    pub fn delete_product(&self, barcode: &str) -> Result<bool> {
        let deleted = self
        .connection
        .execute("DELETE FROM products WHERE barcode = ?1", [barcode])?;
        Ok(deleted > 0)
    }
}
