use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct Clinic {
    pub id: Uuid,
    pub clinic_name: String,
    pub slug: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

pub struct CreateClinicData {
    pub clinic_name: String,
    pub slug: String,
    pub email: String,
    pub password_hash: String,
}

pub async fn create_clinic(pool: &PgPool, data: CreateClinicData) -> Result<Uuid, sqlx::Error> {
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO clinics (clinic_name, slug, email, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        data.clinic_name,
        data.slug,
        data.email,
        data.password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(id)
}

pub async fn get_clinic_by_email(pool: &PgPool, email: &str) -> Result<Option<Clinic>, sqlx::Error> {
    let clinic = sqlx::query_as!(
        Clinic,
        r#"SELECT * FROM clinics WHERE email = $1"#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(clinic)
}

pub async fn get_clinic_by_slug(
    pool: &PgPool,
    slug: &str,
) -> Result<Option<Clinic>, sqlx::Error> {
    let clinic = sqlx::query_as!(
        Clinic,
        r#"SELECT * FROM clinics WHERE slug = $1"#,
        slug
    )
    .fetch_optional(pool)
    .await?;

    Ok(clinic)
}