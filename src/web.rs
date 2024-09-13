#[allow(unused_imports)]
use calc::*;
#[allow(unused_imports)]
use rocket::{serde::json::Json, response::{status,content::RawHtml}, http::Status};
use serde_json::Value;
use serde::Deserialize;

#[macro_use] extern crate rocket;

#[get("/check?<expression>")]
fn wcheck(expression: String) -> Result<Json<bool>, status::Custom<&'static str>> {
  if check(&expression) {
      Ok(Json(true))
  } else {
      Err(status::Custom(Status::BadRequest, "Invalid expression"))
  }
}

#[get("/calc?<expression>")]
async fn wcalc(expression: String) ->  Result<Json<Value>, status::Custom<&'static str>> {
  match calc(&expression) {
      Ok(result) => Ok(Json(result)),
      Err(_) => Err(status::Custom(Status::BadRequest, "Invalid expression")),
  }
}

#[post("/equal", format = "application/json", data = "<expressions>")]
async fn wequal(expressions: Json<Vec<String>>) -> Result<Json<bool>, status::Custom<&'static str>> {
  if expressions.len() != 2 {
        return Err(status::Custom(Status::BadRequest, "Need exactly two expressions"));
    }

    match are_equal(&expressions[0], &expressions[1]) {
        Ok(0) => Ok(Json(true)),
        Ok(_) => Ok(Json(false)),
        Err(_) => Err(status::Custom(Status::BadRequest, "Invalid expressions")),
    }
}

#[get("/generate24")]
async fn wgenerate24() -> Json<[u8; 4]> {
  Json(generate_24_challenge())
}

#[derive(Deserialize)]
struct Challenge {
    numbers: [u8; 4],
    expression: String,
}

#[post("/check24", format = "application/json", data = "<challenge>")]
async fn wcheck24(challenge: Json<Challenge>) -> Status {
    match check_24_challenge(&challenge.numbers, &challenge.expression) {
        Ok(()) => Status::Ok,
        Err(CalcError::ChallengeFalseItems) => Status::BadRequest,
        Err(CalcError::UnsolvedChallenge) => Status::NotFound,
        Err(_) => Status::InternalServerError,
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build()
      .mount("/",routes![wcheck,wcalc,wequal,wgenerate24,wcheck24])
}
