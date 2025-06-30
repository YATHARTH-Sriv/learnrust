use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};

use axum::{
    extract::{Json, Path, Query, State}, http::StatusCode, response::Json as AxumJson, routing::{delete, get, post, put}, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize,Clone)]
struct DATABASE_USER{
    id:String,
    username:String
}

type Userdb=Arc<Mutex<HashMap<String,DATABASE_USER>>>;

#[derive(Clone)]
struct AppState{
    users:Userdb
}


#[tokio::main]
async fn main() {
   
   let users:Userdb=Arc::new(Mutex::new(HashMap::new()));
   let shared_App_state= AppState{
    users
   };

   let app=Router::new()
   .route("/", get(root_handler))
   .route("/user", get(user_handler))
   .route("/greet", get(greet))
   .route("/echo", post(echo))
   .route("/greet/:name", get(dynamic_handler))
   .route("/sharedstate", get(root_state_app_handler))
   .route("/create-user",post(create_user))
   .route("/getuser/:id", get(get_user))
   .route("/users", get(get_all_users))
   .route("/update-user/:id", put(update_user))
   .route("/delete-user/:id", delete(delete_user))
   .with_state(shared_App_state);

   let port: u16 = std::env::var("PORT")
    .unwrap_or_else(|_| "3000".into())
    .parse()
    .expect("PORT must be a number");

    // Define address to run our server on (localhost:3000)
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server running at http://{}", addr);

    // Start the server
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler()->&'static str{
    "Hello World"
}

#[derive(Serialize)]
struct User {
  id:u32,
  username:String
}

async fn user_handler()-> Json<User>{
    let user= User{
        id:1,
        username:String::from("yatharth")
    };

    Json(user)
}


#[derive(Deserialize,Debug)]
struct GreetParams{
    name:String
}

async fn greet(Query(params):Query<GreetParams>)-> String{
   format!("Greetings from the axum server {}",params.name)
}

#[derive(Debug,Serialize,Deserialize)]
struct Message{
    message:String
}

async fn echo(Json(payload): Json<Message>)->AxumJson<Message>{
    println!("{:?}",payload);
    // let c=format!("hello {}", payload.message);
    let d = Message{
        message:format!("This is some added response coming from server {}", payload.message)
    };
    // println!("{}",c);
   AxumJson(d)
}


async fn dynamic_handler(Path(name): Path<String>)-> String{
    format!("{}",name)
}


async fn root_state_app_handler(State(state): State<AppState>)->String{
    let number_of_users= state.users.lock().unwrap().len();
    format!(" Total Users in  memory {}",number_of_users)
}

async fn create_user(State(state):State<AppState>,Json(user):Json<DATABASE_USER>)->AxumJson<Message>{
    state.users.lock().unwrap().insert(user.id.clone(), user.clone());
    let msg=Message{
        message:"User Created".to_string()
    };
    AxumJson(msg)
}


struct ApiRes{
    value:DATABASE_USER
}
async fn get_user(State(state):State<AppState>,Path(id): Path<String>)->Result<AxumJson<DATABASE_USER>, StatusCode>{
    let users = state.users.lock().unwrap();

    if let Some(user) = users.get(&id) {
        Ok(AxumJson(user.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }

}

async fn get_all_users(
    State(state): State<AppState>
) -> AxumJson<Vec<DATABASE_USER>> {
    let users_map = state.users.lock().unwrap();

    let all_users: Vec<DATABASE_USER> = users_map
        .values()
        .cloned()
        .collect(); // .cloned() because DATABASE_USER doesn't implement Copy

    AxumJson(all_users)
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<DATABASE_USER>,
) -> Result<AxumJson<Message>, StatusCode> {
    let mut users = state.users.lock().unwrap();

    if users.contains_key(&id) {
        users.insert(id.clone(), payload.clone());

        Ok(AxumJson(Message {
            message: format!("User with id {} updated", id),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<AxumJson<Message>, StatusCode> {
    let mut users = state.users.lock().unwrap();

    if users.remove(&id).is_some() {
        Ok(AxumJson(Message {
            message: format!("User with id {} deleted", id),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}


