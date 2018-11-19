use super::json;
use vkrs::auth::{AccessToken, OAuthError, Permission};
use vkrs::api::Client;
use std::fs::File;
use std::io::stdin;
use std::env;

static TOKEN_FILE: &'static str = "token.json";

pub fn fetch_access_token(api: &Client) -> Result<AccessToken, OAuthError> {
    let oauth = api.auth(
        env::var("VK_APP_ID").expect("VK_APP_ID env var"),
        env::var("VK_APP_SECRET").expect("VK_APP_SECRET env var"));

    let auth_uri = oauth.auth_uri(&[Permission::Audio, Permission::Video, Permission::Offline, Permission::Status]);
    println!("Go to {} and enter code below...", auth_uri);

    let inp = stdin();
    let code = {
        let mut buf = String::new();
        inp.read_line(&mut buf).unwrap();
        buf
    };

    let token = oauth.request_token(code.trim())?;
    let _ = File::create(TOKEN_FILE).ok().map(|mut f| json::to_writer(&mut f, &token).ok());
    Ok(token)
}

pub fn get_access_token(api: &Client) -> Result<AccessToken, OAuthError> {
    let token: Option<AccessToken> = File::open(TOKEN_FILE).ok().and_then(|mut f| json::from_reader(&mut f).ok());

    if let Some(token) = token {
        if token.expired() {
            fetch_access_token(api)
        } else {
            Ok(token)
        }
    } else {
        fetch_access_token(api)
    }
}

