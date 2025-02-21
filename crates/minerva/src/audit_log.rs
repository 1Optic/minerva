pub enum AuditLogError {}

pub struct AuditLog {}

impl AuditLog {
    pub fn log_query() -> Result<(), AuditLogError> {
        Ok(())
    }
}
