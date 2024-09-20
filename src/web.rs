#[macro_use] extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::status;
use txt::{reverse, is_palindrome, remove_vowels, rle_encode, to_case, TxtToCase};

#[derive(Deserialize)]
struct CaseInput {
    text: String,
    case_type: Option<String>,  // case_type devient une option
}

#[derive(Serialize)]
struct ApiResponse {
    result: String,
}

#[get("/reverse?<arg>")]
fn reverse_text(arg: Option<&str>) -> Result<Json<ApiResponse>, status::BadRequest<String>> {
    match arg {
        Some(text) if !text.is_empty() => Ok(Json(ApiResponse { result: reverse(text) })),
        _ => Err(status::BadRequest("Missing or empty argument".into())),
    }
}

#[get("/is_palindrome?<arg>")]
fn check_palindrome(arg: Option<&str>) -> Result<Json<ApiResponse>, status::BadRequest<String>> {
    match arg {
        Some(text) if !text.is_empty() => {
            let result = if is_palindrome(text) {
                "true".to_string()
            } else {
                "false".to_string()
            };
            Ok(Json(ApiResponse { result }))
        }
        _ => Err(status::BadRequest("Missing or empty argument".into())),
    }
}

#[get("/remove_vowels?<arg>")]
fn remove_vowels_text(arg: Option<&str>) -> Result<Json<ApiResponse>, status::BadRequest<String>> {
    match arg {
        Some(text) if !text.is_empty() => Ok(Json(ApiResponse { result: remove_vowels(text) })),
        _ => Err(status::BadRequest("Missing or empty argument".into())),
    }
}

#[get("/rle_encode?<arg>")]
fn rle_encode_text(arg: Option<&str>) -> Result<Json<ApiResponse>, status::BadRequest<String>> {
    match arg {
        Some(text) if !text.is_empty() => Ok(Json(ApiResponse { result: rle_encode(text) })),
        _ => Err(status::BadRequest("Missing or empty argument".into())),
    }
}

#[post("/to_case", format = "json", data = "<case_input>")]
fn to_case_format(case_input: Json<CaseInput>) -> Result<Json<ApiResponse>, status::BadRequest<String>> {
    // Si case_type est absent, on utilise "snake" comme défaut
    let case_type = match case_input.case_type.as_deref() {
        Some("snake") | None => TxtToCase::SnakeCase,
        Some("camel") => TxtToCase::CamelCase,
        Some("kebab") => TxtToCase::KebabCase,
        _ => return Err(status::BadRequest("Invalid case_type: choose between snake, camel, or kebab".into())),
    };

    let result = to_case(&case_input.text, case_type);
    Ok(Json(ApiResponse { result }))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        reverse_text,
        check_palindrome,
        remove_vowels_text,
        rle_encode_text,
        to_case_format
    ])
}
