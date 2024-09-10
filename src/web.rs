use mogulid::{mogulid_allow_update, mogulid_merge, MogulidError, Mogulid};
use rocket::{fs::FileServer, serde::json::Json};
use rocket::response::content::RawHtml;

#[macro_use] extern crate rocket;

#[get("/get")]
async fn get_new_mogulid() -> Json<Mogulid> {
  let mogulid = Mogulid::new_start();
  Json(mogulid)
}

#[post("/update", format = "json", data = "<mogulid>")]
async fn update_mogulid(mut mogulid: Json<Mogulid>) -> Json<Mogulid> {
  mogulid.define_new_state();
  mogulid
}

#[post("/allow_merge", format = "json", data="<mogulids>")]
async fn allow_merge(mogulids: Json<[Mogulid; 2]>) -> Json<Result<(), MogulidError>> {
  Json(mogulid_allow_update(&mogulids[0], &mogulids[1]))
}

#[put("/merge/<option>", format = "json", data="<mogulids>")]
async fn merge(option: Option<String>, mogulids: Json<[Mogulid; 2]>) -> Json<Result<Mogulid, MogulidError>> {
    if option.as_deref() != Some("force") {
        match mogulid_allow_update(&mogulids[0], &mogulids[1]) {
            Ok(_) => {}
            Err(e) => return Json(Err(e)),
        }
    }

    Json(Ok(mogulid_merge(&mogulids[0], &mogulids[1])))
}

#[get("/test")]
fn test() -> RawHtml<String> {
    let title = "Bienvenue";
    let body = format!("<h1>{}</h1><p>Ceci est du HTML généré dynamiquement.</p>", title);
    RawHtml(body)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
      .mount("/", FileServer::from("./static"))
      .mount("/test",routes![test])
      .mount("/api",routes![get_new_mogulid, update_mogulid, allow_merge, merge])
}
