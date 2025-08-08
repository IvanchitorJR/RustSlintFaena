use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
// estructura de los usuarios en rust, posteriolmente se convierte en json
pub struct Usuario {
    pub nombre: String,
    #[serde(rename = "contrasena")]
    pub contrase_a: String,
    pub email: String,
    pub premium: bool,
}