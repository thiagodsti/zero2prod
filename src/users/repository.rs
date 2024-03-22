use sqlx::PgPool;

use crate::users::entities::new_user::NewUser;

#[tracing::instrument(name = "Saving new user details in the database", skip(new_user, pool))]
pub async fn insert_user(pool: &PgPool, new_user: &NewUser) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (name, email, password, roles)
              VALUES ($1, $2, $3, $4)
        "#,
        new_user.name,
        new_user.email,
        new_user.password,
        &new_user.roles
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
