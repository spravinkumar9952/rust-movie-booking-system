use std::{fmt, sync::Arc};

use axum::{extract::Query, http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use redis::{Client, Commands, Connection, ToRedisArgs};
use sqlx::PgPool;

use crate::common::{self, error::throw500, redis_utils::{get_from_redis_as_json, save_to_redis_as_json}};


#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetTheatresReq {
  limit : Option<i64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetMoviesListRes {
  id : i32,
  name : String,
  address : String,
  no_of_screens : i32,
}

impl fmt::Display for GetMoviesListRes {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "ID: {}, Name: {}, Address: {}, Number of Screens: {}", 
          self.id, self.name, self.address, self.no_of_screens)
  }
}

impl IntoResponse for GetMoviesListRes {
  fn into_response(self) -> Response {
      (StatusCode::OK, Json(self)).into_response()
  }
}

pub async fn get_theatres(
  Query(query): Query<GetTheatresReq>,
  Extension(db_pool): Extension<Arc<PgPool>>,
  Extension(rs_conn): Extension<Arc<Client>>
) -> Result<Json<Vec<GetMoviesListRes>>, common::types::ErrorResponse> {
    // code here 
    let redis_key = get_theatres_key(query.limit);
    println!("Redis Key: {}", redis_key);
    let resp = get_from_redis_as_json::<Vec<GetMoviesListRes>>(rs_conn.clone(), &redis_key).await;
    println!("Redis Resp: {}", resp.is_ok());

    match get_from_redis_as_json::<Vec<GetMoviesListRes>>(rs_conn.clone(), &redis_key).await {
        Ok(resp) => {
            println!("From Redis");
            return Ok(Json(resp))
        }
        Err(e) => {
          let movies  = sqlx::query!("SELECT * FROM theatre LIMIT $1", query.limit) 
          .fetch_all(&*db_pool)
          .await;
      
          match movies {
              Ok(movies) => {
                  let mut resp:Vec<GetMoviesListRes> = vec![];
                  for movie in movies {
                      resp.push(GetMoviesListRes {
                          id: movie.id,
                          name: movie.name,
                          address: movie.address,
                          no_of_screens: movie.no_of_screens,
                      });
                  }
                  save_to_redis_as_json(rs_conn.clone(), get_theatres_key(query.limit), &resp).await;
                  println!("From DB");
                  return Ok(Json(resp))
              }
              Err(e) => {
                return throw500("NOT_ABLE_TO_FETCH_MOVIE", e.to_string().as_str());
              }
          }
        }
    }
}


fn get_theatres_key(limit : Option<i64>) -> String {
  match limit {
    Some(limit) => format!("get_theatres_{}", limit),
    None => "get_theatres".to_string()
  }
}