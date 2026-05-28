use axum::{
    routing::{get, post},
    Router, 
    Json, 
    extract::{Path, State}, 
    http::StatusCode,
};
use serde::{Deserialize, Serialize}; //Сериализатор
use std::collections::HashMap;  // Стандартная библиоека, подключение штатных структур данных, HashMap 
use std::sync::Arc;
use tokio::sync::Mutex; 

//Sgared application state
type UserStore = Arc<Mutex<HashMap<u32, User>>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u32, 
    name: String, 
    email: String,
}

// Health check and protection of ""
async fn health_check() -> &'static str {
    "OK"
}

// Crreate user

async fn create_user(
    State(store): State<UserStore>,
    Json(payload): Json<User>, 
) -> (StatusCode, Json<User>){
    let mut users = store.lock().await;
    let id = users.keys().max().cloned().unwrap_or(0) + 1;
    let user = User {id, ..payload};
    users.insert(id, user.clone());
    (StatusCode::CREATED, Json(user))
}

// Get user by ID 
async fn get_user(
    State(store): State<UserStore>,
    Path(id): Path<u32>,
) -> Result<Json<User>, StatusCode> {
    let users = store.lock().await;
    users
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[tokio::main]
async fn main() {
    // Include shared store
    let user_store: UserStore = Arc::new(Mutex::new(HashMap::new()));

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .with_state(user_store);

    //start server 
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        println!("server running on http://0.0.0.0:3000");
        axum::serve(listener, app).await.unwrap();

}
