use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

//actix-web uses a type-map to represent its application state: a HashMap that stores arbitrary data
//(using the Any type) against their unique type identifier (obtained via TypeId::of).
//web::Data, when a new request comes in, computes the TypeId of the type you specified in the signature (in our case PgConnection) and checks if there is a record corresponding to it in the type- map.
//If there is one, it casts the retrieved Any value to the type you specified (TypeId is unique, nothing to worry about) and passes it to your handler.
//It is an interesting technique to perform what in other language ecosystems might be referred to as dependency injection.
#[tracing::instrument(
    name = "Adding a new subscriber", skip(form, pool),
    fields(
    request_id = %Uuid::new_v4(), subscriber_email = %form.email, subscriber_name= %form.name
    ) )]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(())
}
