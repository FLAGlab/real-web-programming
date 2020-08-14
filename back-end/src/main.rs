#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/api/thopics")]
fn get_thopics() -> content::Json<&'static str> {
    content::Json(
        "[
            { 
                'name': \"Mariño, Olga\", 
                'topics': [\"Componentes educativos a ser integrados en ambientes de aprendizaje activo\", \"Predicción de deserción académica\"] 
            }, 
            { 
                'name': \"Duitama Jorge\", 
                'topics': [\"Visualización de alineamientos de genomas\", \"Predicción de estructura de proteinas\", \"Análisis automatizado de imágenes de geles de poliacrilamida\"] 
            }
        ]",
    )
}

#[get("/users")]
fn get_all_users() -> &'static str {
    "GET All Users"
}

#[get("/users/<id>")]
fn get_user(id: String) -> String {
    format!("GET user id = {}", id)
}

#[put("/users/<id>")]
fn put_user(id: String) -> String {
    format!("PUT user id = {}", id)
}

#[post("/users")]
fn post_user() -> &'static str {
    "POST new user"
}

#[delete("/users/<id>")]
fn delete_user(id: String) -> String {
    format!("DELETE user id = {}", id)
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                get_thopics,
                get_all_users,
                get_user,
                put_user,
                post_user,
                delete_user
            ],
        )
        .launch();
}
