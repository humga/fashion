use chrono::{Duration, Utc};
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json},
    Error, Request, Result,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::auth::claims::Claims;

pub mod claims;
pub mod jwt_middleware;

#[derive(Debug, Deserialize, sqlx::FromRow)]
struct LoginInfo {
    pub name: String,
    pub password: String,
}

#[handler]
pub async fn login(info: Json<LoginInfo>, state: Data<&PgPool>) -> Result<String> {
    let user = sqlx::query_as::<_, LoginInfo>(
        "select  username as name,password from \"user\" where username=$1",
    )
    .bind(&info.name)
    .fetch_one(state.0)
    .await;
    // .map_err(BadRequest)?;
    // if info.password.eq(&user.password) {
    //     claims::create_jwt(claims::Claims::new(info.name.clone()))
    // } else {
    //     Err(Error::from_string(
    //         "user don't exist!",
    //         StatusCode::BAD_REQUEST,
    //     ))
    // }
    match user {
        Ok(user) => {
            if user.password.eq(&info.password) {
                claims::create_jwt(claims::Claims::new(info.name.clone()))
            } else {
                Err(Error::from_string(
                    "password not match!",
                    StatusCode::BAD_REQUEST,
                ))
            }
        }
        Err(sqlx::Error::RowNotFound) => Err(Error::from_string(
            "username not found",
            StatusCode::BAD_REQUEST,
        )),

        Err(e) => Err(Error::from_string(e.to_string(), StatusCode::BAD_REQUEST)),
    }
}

#[handler]
pub async fn register(info: Json<LoginInfo>, state: Data<&PgPool>) -> Result<String> {
    let ids =  sqlx::query("INSERT INTO \"user\" (username,password,create_time,status)  VALUES ( $1, $2, now(), TRUE) returning id; ")
                    .bind(&info.name)
                    .bind(&info.password)
                    .fetch_one(state.0)
                    .await
                    ;
    match ids {
        Ok(_) => Ok(String::from("success")),
        Err(sqlx::Error::Database(e)) => match e.is_unique_violation() {
            true => Err(Error::from_string("user exits", StatusCode::CONFLICT)),
            false => Err(Error::from_string(
                "database error",
                StatusCode::BAD_REQUEST,
            )),
        },
        _ => Err(Error::from_string("unknow error", StatusCode::BAD_REQUEST)),
    }
}

#[handler]
pub fn refresh_token(req: &Request) -> poem::Result<String> {
    let res = req.extensions().get::<Claims>().unwrap();
    let mut claim: Claims = Claims::new(res.username.clone());
    claim.exp =
        (Utc::now() + Duration::try_hours(claims::JWT_EXPIRATION_HOURS).unwrap()).timestamp();
    claims::create_jwt(claim)
}
