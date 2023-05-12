use {
    crate::auth,
    axum::response::IntoResponse,
    data_encoding::DecodeError,
    hyper::StatusCode,
    std::string::FromUtf8Error,
    tracing::{error, warn},
    walletconnect_sdk::rpc::domain::ClientIdDecodingError,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid event for webhook")]
    InvalidEvent,

    #[error("The provided url has invalid scheme")]
    InvalidScheme,

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Envy(#[from] envy::Error),

    #[error(transparent)]
    RpcAuth(#[from] walletconnect_sdk::rpc::auth::Error),

    #[error(transparent)]
    Trace(#[from] opentelemetry::trace::TraceError),

    #[error(transparent)]
    Metrics(#[from] opentelemetry::metrics::MetricsError),

    #[error(transparent)]
    Database(#[from] mongodb::error::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),

    #[error(transparent)]
    Hex(#[from] hex::FromHexError),

    #[error(transparent)]
    Prometheus(#[from] prometheus_core::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::error::Error),

    #[error(transparent)]
    BsonSerialization(#[from] mongodb::bson::ser::Error),

    #[error(transparent)]
    WebSocket(#[from] tungstenite::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),

    #[error("No project found associated with topic {0}")]
    NoProjectDataForTopic(String),

    #[error("No client found associated with topic {0}")]
    NoClientDataForTopic(String),

    #[error("Tried to interact with channel that's already closed")]
    ChannelClosed,

    #[error(transparent)]
    DecodeError(#[from] DecodeError),

    #[error("Missing {0}")]
    SubscriptionAuthError(String),

    #[error(transparent)]
    TryFromSliceError(#[from] std::array::TryFromSliceError),

    #[error("Invalid key length")]
    HkdfInvalidLength,

    #[error("Failed to get value from json")]
    JsonGetError,

    #[error("Cryptography failure: {0}")]
    EncryptionError(String),

    #[error("Failed to receive on websocket")]
    RecvError,

    #[error(transparent)]
    SystemTimeError(#[from] std::time::SystemTimeError),

    #[error("Failed to parse the keypair seed")]
    InvalidKeypairSeed,

    #[error("GeoIpReader Error: {0}")]
    GeoIpReader(String),

    #[error("BatchCollector Error: {0}")]
    BatchCollector(String),

    #[error(transparent)]
    JwtVerificationError(#[from] auth::AuthError),

    #[error(transparent)]
    ClientIdDecodingError(#[from] ClientIdDecodingError),

    #[error(transparent)]
    ChronoParse(#[from] chrono::ParseError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        warn!("{:?}", self);
        match self {
            Self::Database(_) => (
                StatusCode::BAD_REQUEST,
                "Client seems to already be registered for this project id",
            )
                .into_response(),
            Self::Url(_) => (StatusCode::BAD_REQUEST, "Invalid url. ").into_response(),
            Self::SerdeJson(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Serialization failure.").into_response()
            }
            Self::Hex(_) => (StatusCode::BAD_REQUEST, "Invalid symmetric key").into_response(),
            error => {
                warn!("Unhandled error: {:?}", error);
                (StatusCode::NOT_FOUND, "Not found.").into_response()
            }
        }
    }
}
