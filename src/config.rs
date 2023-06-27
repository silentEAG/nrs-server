use hmac::Hmac;
use jwt::VerifyWithKey;
use poem::Request;
use poem_openapi::{auth::ApiKey, SecurityScheme};
use sha2::Sha256;

use crate::common::object::user::UserSign;

pub const SALT: &[u8; 14] = b"Nekopara114514";
pub const SERVER_KEY: &[u8] = b"0237jfH#f3h289f3j0";
pub const API_KEY: &str = "SNn0TR#*N0f#JDMWsdmiwan3dj2d2k3d";

pub type ServerKey = Hmac<Sha256>;

/// ApiKey authorization
#[derive(SecurityScheme)]
#[oai(
    type = "api_key",
    key_name = "NRS-TOKEN",
    in = "header",
    checker = "api_checker"
)]
pub struct AppAuthorization(pub UserSign);

async fn api_checker(req: &Request, api_key: ApiKey) -> Option<UserSign> {
    let server_key = req.data::<ServerKey>().unwrap();
    VerifyWithKey::<UserSign>::verify_with_key(api_key.key.as_str(), server_key).ok()
}
