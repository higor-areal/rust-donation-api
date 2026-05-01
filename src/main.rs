use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone)]
struct Donation{
    donor: String,
    amount: f64,
}

impl Donation{
    fn valid(self: &Self) -> Result<(), String>{
        match self {
            x if x.amount < 0.0 => {
                Err("amount invalido!".to_string())
            }
            x if x.donor.is_empty() => {
                Err("Donor invalido!".to_string())
            }
            _ => {
                Ok(())
            }
        }
    }
}
struct AppState{
    total: f64,
    donations: Vec<Donation>,
}


#[tokio::main]
async fn main() {

    let state = AppState{
        total: 0.0,
        donations: vec![]
    };

    let shared = std::sync::Arc::new(std::sync::Mutex::new(state));

    let app = axum::Router::new()
    .route("/", axum::routing::get(home))
    .route("/total", axum::routing::get(get_total))
    .route("/donate", axum::routing::post(donate))
    .route("/donations", axum::routing::get(get_donates))
    .with_state(shared);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> String{
    "Sejá bem vindo a API do higor".to_string()
}

async fn get_total(axum::extract::State(state): axum::extract::State<std::sync::Arc<std::sync::Mutex<AppState>>>) -> String{
    let data = state.lock().unwrap();
    format!("Total: {}", data.total)
}

async fn donate(
    axum::extract::State(state): 
    axum::extract::State<std::sync::Arc<std::sync::Mutex<AppState>>>, 
    axum::Json(x): 
    axum::Json<Donation>)
     -> Result<String, String>{
    let _ = x.valid()?;

    let mut data = state.lock().unwrap();
    data.total += x.amount;
    let res = format!("Adicionando R$ {} de {}", x.amount, x.donor);
    data.donations.push(x);
    Ok(res)
}

async fn get_donates(axum::extract::State(state): axum::extract::State<std::sync::Arc<std::sync::Mutex<AppState>>>) ->axum::Json<Vec<Donation>>{
    let data = state.lock().unwrap();
    axum::Json(data.donations.clone())
}
