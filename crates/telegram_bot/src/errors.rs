pub enum Error {
    DBUniqueConstraintViolation,
    DBError(sqlx::Error),
}
