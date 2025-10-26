// src-tauri/src/db.rs
use reqwest::Client;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use tauri::command;

#[derive(Serialize)]
pub struct SignUserResponse{
    pub answer: String,
    pub token: String,
}

#[derive(Serialize)]
pub struct PostUserParams<'a> {
    name: &'a str,
    email: &'a str,
    password: &'a str,
}
#[command]
pub async fn post_user(name: &str, email: &str, password: &str) -> Result<SignUserResponse, String> {
    let client = Client::new();

    let user: PostUserParams<'_> = PostUserParams {
        name,
        email,
        password,
    };

    let response = client
        .post("http://localhost:3000/signUp")
        .json(&user)
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let answer = response.text().await.map_err(|e| e.to_string())?;
    let token = generate_token(email.to_string())?;

    Ok(SignUserResponse { answer, token })
}


#[derive(Serialize)]
pub struct LogUserParams<'a> {
    email: &'a str,
    password: &'a str,
}
#[command]
pub async fn login_user(email: &str, password: &str) -> Result<SignUserResponse, String>{
    let client = Client::new();

    let user: LogUserParams<'_> = LogUserParams {
        email,
        password,
        };

    let response = client
        .post("http://localhost:3000/find")
        .json(&user)
        .send()
        .await
        .map_err(|error| error.to_string())?;    

    let answer = response.text().await.map_err(|e| e.to_string())?;
    let token = generate_token(email.to_string())?;

    Ok(SignUserResponse { answer, token })
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_identification: String,       // ID ou email do usuário
    token_expiration_time: usize,        // timestamp de expiração
}
fn generate_token(email: String) -> Result<String, String> {
    // Aqui você validaria o email e senha com o banco de dados
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        user_identification: email.clone(),
        token_expiration_time: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("sua_chave_secreta".as_ref()),
    ).map_err(|error| error.to_string())?;

    Ok(token)
}
