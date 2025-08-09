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

#[derive(Serialize)]
pub struct SalaVotacion {
    pub id: i32,
    pub nombre: String,
    pub descripcion: String,
    pub recurrente: bool,
    pub privada: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtro_dominio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codigo_acceso: Option<String>,
    pub max_participantes: i32,
    pub fecha_inicio: Option<chrono::NaiveDate>, 
    pub hora_inicio: Option<chrono::NaiveTime>,  
    pub hora_cierre: Option<chrono::NaiveTime>,
    pub creador_id: i32,
    pub creado_en: chrono::NaiveDateTime,
    pub activa: bool,
}