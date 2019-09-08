use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[cfg(feature = "totp")]
#[derive(Deserialize, Debug)]
pub struct TotpCode {
    pub code: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum AuthRequest {
    #[serde(rename = "user")]
    Credentials(Credentials),

    #[cfg(feature = "totp")]
    #[serde(rename = "totp")]
    TotpCode(TotpCode),
}
