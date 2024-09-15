use std::sync::Arc;

use redis::{Client, Commands};
use serde::{de::DeserializeOwned, Serialize};

pub async fn save_to_redis_as_json<T: Serialize>(
    client: Arc<Client>, 
    key: String, 
    obj: &T
)  {
    // Get an async Redis connection
    let mut redis_conn = client.get_connection().expect("Failed to get redis connection");
    let stringified_val = serde_json::to_string(obj).expect("Failed to serialize movie to JSON");
    let _: () = redis::cmd("SET")
    .arg(key)
    .arg(stringified_val)
    .query(&mut redis_conn)
    .expect("failed to execute SET for 'foo'");
}



pub async fn get_from_redis_as_json<T:DeserializeOwned>(client: Arc<Client>, key: &String) -> redis::RedisResult<T> {
  let mut recis_conn = client.get_connection().expect("Failed to get redis connection");
  let movie_json: String = recis_conn.get(key)?;
  let res =  serde_json::from_str(&movie_json).expect("Failed to deserialize JSON");
  Ok(res)
}
