#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    // Connection string field
    pub connection_string: Option<String>,
    // Individual fields (used if connection_string is not provided)
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub dbname: Option<String>,
    pub ssl_mode: Option<String>,
    pub connect_timeout_secs: Option<u64>,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> Result<String, &'static str> {
        if let Some(ref conn_str) = self.connection_string {
            Ok(conn_str.clone())
        } else {
            // Ensure required fields are present
            let host = self.host.as_ref().ok_or("Host is required")?;
            let username = self.username.as_ref().ok_or("Username is required")?;
            let dbname = self.dbname.as_ref().ok_or("Database name is required")?;

            let mut conn_str = format!("postgresql://{}@{}", username, host);

            if let Some(port) = self.port {
                conn_str.push_str(&format!(":{}", port));
            }
            if let Some(ref password) = self.password {
                conn_str.push_str(&format!(":{}", password));
            }
            conn_str.push_str(&format!("/{}", dbname));

            if let Some(ref ssl_mode) = self.ssl_mode {
                conn_str.push_str(&format!("?sslmode={}", ssl_mode));
            }

            Ok(conn_str)
        }
    }
}
