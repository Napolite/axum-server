use crate::error::{Error, Result};
use crate::web::AUTH_COOKIES;
use axum::{middleware::Next, http::Request, response::Response};
use tower_cookies::Cookies;

pub async fn mw_require_auth<B>(cookies:Cookies, req: Request<B>, next: Next<B>) -> Result<Response> {
    
    let auth_token = cookies.get(AUTH_COOKIES).map(|c| c.value().to_string());

    println!("{:?}", cookies);

    auth_token.ok_or(Error::AuthTokenNotFound)?;

    Ok(next.run(req).await)
}
