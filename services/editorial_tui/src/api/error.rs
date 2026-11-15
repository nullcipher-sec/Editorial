use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ApiError {
    #[error("No autorizado (401): Sesión no válida o expirada")]
    Unauthorized,

    #[error("Acceso denegado (403): Permisos insuficientes para esta área")]
    Forbidden,

    #[error("Recurso no encontrado (404)")]
    NotFound,

    #[error("Operación no implementada en Fase 1 (501): {0}")]
    NotImplemented(String),

    #[error("Error de validación: {0}")]
    Validation(String),

    #[error("Error de comunicación con la API: {0}")]
    Network(String),

    #[error("Error al procesar respuesta del servidor: {0}")]
    Serialization(String),
}
