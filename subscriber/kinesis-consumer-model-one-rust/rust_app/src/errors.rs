use momento::MomentoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Error communicating with Momento")]
    Communication(#[from] MomentoError),
    #[error("failed to parse response into a user: {0}")]
    SerdeError(serde_dynamo::Error),
    #[error("aws_sdk_dynamodb error: {0}")]
    DynamoError(aws_sdk_dynamodb::Error),
    #[error("aws_sdk_dynamodb::error:: error: {0}")]
    DynamoSdkError(String),
    #[error("item not found")]
    NotFound,
}

impl From<aws_sdk_dynamodb::Error> for CacheError {
    fn from(err: aws_sdk_dynamodb::Error) -> Self {
        CacheError::DynamoError(err)
    }
}

impl From<serde_dynamo::Error> for CacheError {
    fn from(err: serde_dynamo::Error) -> Self {
        CacheError::SerdeError(err)
    }
}

impl<E, R> From<aws_sdk_dynamodb::error::SdkError<E, R>> for CacheError
where
    E: std::fmt::Debug,
    R: std::fmt::Debug,
{
    fn from(err: aws_sdk_dynamodb::error::SdkError<E, R>) -> Self {
        CacheError::DynamoSdkError(std::format!("{:?}", err))
    }
}
