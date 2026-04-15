use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::auth::AuthUser,
    models::study::{CreateStudyRequest, JoinStudyRequest, Study, StudyEnrollment, StudiesQuery},
    AppState,
};

pub async fn list_studies(
    State(state): State<AppState>,
    Query(query): Query<StudiesQuery>,
) -> Result<Json<Vec<Study>>, (StatusCode, String)> {
    let status = query.status.unwrap_or("recruiting".into());
    let page = query.page.unwrap_or(0);

    let studies = sqlx::query_as!(
        Study,
        r#"SELECT id, creator_id, title, hypothesis, protocol, target_n, enrolled_n,
                  duration_days, status, dua_template_id, result_doi, arbiter_id,
                  created_at, starts_at, ends_at
           FROM studies
           WHERE status = $1
           ORDER BY created_at DESC
           LIMIT 20 OFFSET $2"#,
        status,
        page * 20,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(studies))
}

pub async fn get_study(
    State(state): State<AppState>,
    Path(study_id): Path<Uuid>,
) -> Result<Json<Study>, (StatusCode, String)> {
    let study = sqlx::query_as!(
        Study,
        r#"SELECT id, creator_id, title, hypothesis, protocol, target_n, enrolled_n,
                  duration_days, status, dua_template_id, result_doi, arbiter_id,
                  created_at, starts_at, ends_at
           FROM studies WHERE id = $1"#,
        study_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "Study not found".into()))?;

    Ok(Json(study))
}

pub async fn create_study(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<CreateStudyRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    req.validate()
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;

    let id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO studies (id, creator_id, title, hypothesis, protocol, target_n, duration_days, dua_template_id)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
        id,
        auth_user.id,
        req.title,
        req.hypothesis,
        req.protocol,
        req.target_n,
        req.duration_days,
        req.dua_template_id,
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({ "id": id })))
}

pub async fn join_study(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(study_id): Path<Uuid>,
    Json(req): Json<JoinStudyRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Verify study is recruiting
    let study = sqlx::query!(
        "SELECT status, enrolled_n, target_n FROM studies WHERE id = $1",
        study_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "Study not found".into()))?;

    if study.status.as_deref() != Some("recruiting") {
        return Err((StatusCode::CONFLICT, "Study is not recruiting".into()));
    }
    if study.enrolled_n.unwrap_or(0) >= study.target_n {
        return Err((StatusCode::CONFLICT, "Study has reached target enrollment".into()));
    }

    let enrollment_id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO study_enrollments (id, study_id, user_id, consent_text)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (study_id, user_id) DO NOTHING"#,
        enrollment_id,
        study_id,
        auth_user.id,
        req.consent_text,
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query!(
        "UPDATE studies SET enrolled_n = enrolled_n + 1 WHERE id = $1",
        study_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "enrollment_id": enrollment_id,
        "message": "Successfully enrolled. Your consent has been recorded."
    })))
}

pub async fn leave_study(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(study_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let rows = sqlx::query!(
        "UPDATE study_enrollments SET status = 'withdrawn' WHERE study_id = $1 AND user_id = $2 AND status = 'active'",
        study_id,
        auth_user.id,
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .rows_affected();

    if rows == 0 {
        return Err((StatusCode::NOT_FOUND, "Enrollment not found".into()));
    }

    sqlx::query!(
        "UPDATE studies SET enrolled_n = GREATEST(0, enrolled_n - 1) WHERE id = $1",
        study_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
