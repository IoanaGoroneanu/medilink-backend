use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

use crate::router::routes::clinic_signup::clinic_signup;
use crate::router::routes::list_doctors::list_doctors;
use crate::router::routes::patient_signup::patient_signup;
use crate::router::routes::clinic_login::clinic_login;
use crate::router::routes::add_doctor::add_doctor;
use crate::router::routes::patient_login::patient_login;
use crate::router::routes::book_appointment::book_appointment;
use crate::router::routes::get_appointments::get_appointments;
use crate::router::routes::cancel_appointment::cancel_appointment_handler;
use crate::ApiContext;

pub fn router() -> Router<Arc<ApiContext>> {
    Router::new()
        .route("/", get(|| async { "hello world" }))
        .route("/clinic_signup", post(clinic_signup))
        .route("/clinic_login", post(clinic_login))
        .route("/clinic/doctors", post(add_doctor))
        .route("/patient_signup", post(patient_signup))
        .route("/patient_login", post(patient_login))
        .route("/appointments", post(book_appointment))
        .route("/clinics/{slug}/doctors", get(list_doctors))
        .route("/appointments", get(get_appointments))
        .route("/clinics/{slug}/login", post(patient_login))
        .route("/clinics/{slug}/patients/register", post(patient_signup))
        .route("/appointments/{id}/cancel", post(cancel_appointment_handler))
}