use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn split_pgpass_line(line: &str) -> Option<[String; 5]> {
    let mut fields: Vec<String> = Vec::with_capacity(5);
    let mut current = String::new();
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                if let Some(next) = chars.next() {
                    current.push(next);
                } else {
                    current.push('\\');
                }
            }
            ':' => {
                fields.push(std::mem::take(&mut current));
                if fields.len() > 5 {
                    return None;
                }
            }
            _ => current.push(ch),
        }
    }

    fields.push(current);

    if fields.len() != 5 {
        return None;
    }

    Some([
        fields[0].clone(),
        fields[1].clone(),
        fields[2].clone(),
        fields[3].clone(),
        fields[4].clone(),
    ])
}

fn default_pgpass_path() -> Option<PathBuf> {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .ok()?;

    Some(PathBuf::from(home).join(".pgpass"))
}

fn pgpass_path_from_env() -> Option<PathBuf> {
    if let Ok(v) = env::var("PGPASSFILE") {
        if v.trim().is_empty() {
            None
        } else {
            Some(PathBuf::from(v))
        }
    } else {
        default_pgpass_path()
    }
}

fn pgpass_field_matches(pattern: &str, value: &str) -> bool {
    pattern == "*" || pattern == value
}

pub fn lookup_password_from_file(
    pgpass_file: &Path,
    host: &str,
    port: u16,
    database: &str,
    user: &str,
) -> Option<String> {
    let content = fs::read_to_string(pgpass_file).ok()?;

    for raw_line in content.lines() {
        let line = raw_line.trim_end_matches(['\n', '\r']);
        let trimmed = line.trim_start();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let Some([h, p, d, u, pw]) = split_pgpass_line(trimmed) else {
            continue;
        };

        let port_str = port.to_string();

        if pgpass_field_matches(&h, host)
            && pgpass_field_matches(&p, &port_str)
            && pgpass_field_matches(&d, database)
            && pgpass_field_matches(&u, user)
        {
            return Some(pw);
        }
    }

    None
}

pub fn lookup_password(host: &str, port: u16, database: &str, user: &str) -> Option<String> {
    let pgpass_file = pgpass_path_from_env()?;
    lookup_password_from_file(&pgpass_file, host, port, database, user)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_pgpass_line() {
        let parsed = split_pgpass_line(r"localhost:5432:mydb:me:p\:a\\ss").unwrap();
        assert_eq!(parsed[0], "localhost");
        assert_eq!(parsed[1], "5432");
        assert_eq!(parsed[2], "mydb");
        assert_eq!(parsed[3], "me");
        assert_eq!(parsed[4], "p:ass");
    }

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
