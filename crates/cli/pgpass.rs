use std::path::Path;

use postgres_secrets::PgPass;

pub fn lookup_password_from_file(
    pgpass_file: &Path,
    host: &str,
    port: u16,
    database: &str,
    user: &str,
) -> Option<String> {
    let pgpass = PgPass::open(pgpass_file).ok()?;

    let creds = pgpass
        .query()
        .hostname(host)
        .ok()?
        .port(port)
        .ok()?
        .database(database)
        .ok()?
        .username(user)
        .ok()?
        .find()
        .ok()??;

    Some(creds.password)
}

pub fn lookup_password(host: &str, port: u16, database: &str, user: &str) -> Option<String> {
    let pgpass = PgPass::load().ok()?;

    let creds = pgpass
        .query()
        .hostname(host)
        .ok()?
        .port(port)
        .ok()?
        .database(database)
        .ok()?
        .username(user)
        .ok()?
        .find()
        .ok()??;

    Some(creds.password)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_lookup_password_from_file_matches_first() {
        let dir = env::temp_dir();
        let file = dir.join("minerva_cli_pgpass_test");

        fs::write(
            &file,
            "# comment\nlocalhost:5432:db:user:pw1\n*:5432:db:user:pw2\n",
        )
        .unwrap();

        let pw = lookup_password_from_file(&file, "localhost", 5432, "db", "user").unwrap();
        assert_eq!(pw, "pw1");

        let _ = fs::remove_file(file);
    }

    #[test]
    fn test_lookup_password_from_file_wildcards() {
        let dir = env::temp_dir();
        let file = dir.join("minerva_cli_pgpass_test2");

        fs::write(&file, "*:5432:*:user:pw\n").unwrap();

        let pw = lookup_password_from_file(&file, "anyhost", 5432, "anydb", "user").unwrap();
        assert_eq!(pw, "pw");

        let _ = fs::remove_file(file);
    }
}
