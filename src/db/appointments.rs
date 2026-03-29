use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Appointment {
    pub id: Uuid,
    pub clinic_id: Uuid,
    pub doctor_id: Uuid,
    pub patient_id: Uuid,
    pub appointment_time: DateTime<Utc>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub struct CreateAppointmentData {
    pub clinic_id: Uuid,
    pub doctor_id: Uuid,
    pub patient_id: Uuid,
    pub appointment_time: DateTime<Utc>,
    pub notes: Option<String>,
}

pub async fn create_appointment(
    pool: &PgPool,
    data: CreateAppointmentData,
) -> Result<Uuid, sqlx::Error> {
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO appointments (clinic_id, doctor_id, patient_id, appointment_time, notes)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        data.clinic_id,
        data.doctor_id,
        data.patient_id,
        data.appointment_time,
        data.notes
    )
    .fetch_one(pool)
    .await?;

    Ok(id)
}

pub async fn get_appointments_by_patient(
    pool: &PgPool,
    patient_id: Uuid,
    clinic_id: Uuid,
) -> Result<Vec<Appointment>, sqlx::Error> {
    let appointments = sqlx::query_as!(
        Appointment,
        r#"
        SELECT id, clinic_id, doctor_id, patient_id, appointment_time, 
               status as "status: String", notes, created_at
        FROM appointments
        WHERE patient_id = $1 AND clinic_id = $2
        ORDER BY appointment_time ASC
        "#,
        patient_id,
        clinic_id
    )
    .fetch_all(pool)
    .await?;

    Ok(appointments)
}

pub async fn cancel_appointment(
    pool: &PgPool,
    appointment_id: Uuid,
    patient_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let rows_affected = sqlx::query!(
        r#"
        UPDATE appointments
        SET status = 'cancelled'
        WHERE id = $1 AND patient_id = $2 AND status = 'scheduled'
        "#,
        appointment_id,
        patient_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}