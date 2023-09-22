use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::extract::State;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tokio::time;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MyStruct {
    name: String,
    age: i64,
}

fn helper() -> Result<(), Box<dyn Error>> {
    let a = MyStruct {
        name: "Nurlan".to_string(),
        age: 22
    };
    let b = serde_json::to_string(&a).unwrap() + "";
    println!("{}", b);

    let c: Result<MyStruct, _ > = serde_json::from_str(&b);
    // match &c {
    //     Ok(c) => { println!("{}", c.name)}
    //     Err(e) => {println!("7848: {}", e)}
    // }
    //
    // if let Ok(c) = &c {
    //     println!("{}", c.age)
    // }
    println!("{}", c?.age);
    Ok(())
}
#[derive(Clone)]
struct AppState {
    users: Arc<RwLock<Vec<MyStruct>>>
}

#[tokio::main]
async fn main() {

    let app_state = AppState {
        users: Arc::new(RwLock::new(vec![
            MyStruct{
                age: 22,
                name: "Nurlan".to_string()
            },
            MyStruct{
                age: 22,
                name: "Elgun".to_string()
            }
        ]))
    };

    // let a = helper();
    // println!("{:?}", a);
    // time::sleep(Duration::from_secs(5)).await;

    let app = Router::new()
        .route("/", get(root))
        .route("/api/rust/users", get(get_users))
        .route("/api/rust/users", post(AddUser))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3005));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
async fn get_users(State(state): State<AppState>) -> impl IntoResponse {
    let guard = state.users.read().await;
    let users = Vec::clone(guard.as_ref());
    return Json(users);
}

async fn AddUser(State(mut state): State<AppState>, Json(dto): Json<MyStruct>) -> impl IntoResponse {
    let mut guard = state.users.write().await;
    guard.push(dto);
    return "salam";
}
