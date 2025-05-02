use crate::model::*;

pub async fn init_database(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user1 (
        id bigserial PRIMARY KEY,
        name Text,
        email Text,
        password Text
        )"
    )
    .execute(pool)
    .await?;
    Ok(())
}

//Create User
pub async fn create_user(pool: &sqlx::PgPool, name: &str, email: &str, password: &str) -> Result<(), sqlx::Error> {
    let query = "INSERT INTO user1 (name, email, password) VALUES ($1, $2, $3)";
    sqlx::query(query)
    .bind(name)
    .bind(email)
    .bind(password)
    .execute(pool)
    .await?;
    Ok(())
}

//User Login
pub async fn get_user(pool: &sqlx::PgPool, email: &str) -> Result<User, sqlx::Error> {
    let query = "SELECT * FROM user1 WHERE email=$1";
    let user = sqlx::query_as::<_, User>(query)
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(user)
}
