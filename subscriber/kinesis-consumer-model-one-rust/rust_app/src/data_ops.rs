use crate::{errors::CacheError, models::CacheModel};
use momento::{cache::GetResponse, CacheClient};

/// Fetch item coordinates this workflow
///     Fetch from Cache
///         If not found, go to DynamoDB
///             If found in DDB then persist in Momento with a Set Dictionary
///         Return Empty
pub async fn fetch_item(
    ddb_client: &aws_sdk_dynamodb::Client,
    cache_client: &CacheClient,
    location: String,
) -> Result<Option<CacheModel>, CacheError> {
    tracing::info!(location = location, "Fetching from cache");
    // use the client to execute a Get
    match cache_client
        .get("sample-a".to_string(), location.clone())
        .await
    {
        Ok(r) => match r {
            // match on OK or Error
            GetResponse::Hit { value } => {
                // A Cache Hit
                tracing::info!("Cache HIT");
                let cached: String = value.try_into().expect("Should have been a string");
                let model = serde_json::from_str(cached.as_ref()).unwrap();
                Ok(Some(model))
            }
            GetResponse::Miss => {
                // A Cache Miss
                tracing::info!("Cache MISS, going to DDB");
                let db_fetch_result = fetch_from_db(ddb_client, location.clone()).await;
                match db_fetch_result {
                    Ok(m) => {
                        tracing::info!("Found item in DDB, setting cache");
                        set_item(cache_client, &m).await?;
                        Ok(Some(m))
                    }
                    Err(_) => {
                        tracing::info!("No item found in DDB OR Cache");
                        Ok(None)
                    }
                }
            }
        },
        Err(e) => {
            tracing::error!("(Error)={:?}", e);
            Ok(None)
        }
    }
}

/// set_item performs the put to Momento for the Cache's Dictionary Object
async fn set_item(cache_client: &CacheClient, cache_model: &CacheModel) -> Result<(), CacheError> {
    tracing::info!(
        location = cache_model.location.clone(),
        "Setting location in cache"
    );
    let s = serde_json::to_string(cache_model).unwrap();
    match cache_client
        .set("sample-a".to_string(), cache_model.location.clone(), s)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("(Error)={:?}", e);
            Ok(())
        }
    }
}

/// fetch_from_db runs the DynamoDB query
async fn fetch_from_db(
    ddb_client: &aws_sdk_dynamodb::Client,
    location: String,
) -> Result<CacheModel, CacheError> {
    tracing::info!("Fetching {:?} from DB", location.clone());
    let r = ddb_client
        .get_item()
        .key(
            "location".to_string(),
            aws_sdk_dynamodb::types::AttributeValue::S(location),
        )
        .table_name("Locations".to_string())
        .send()
        .await?;

    match r.item {
        Some(i) => {
            let m: CacheModel = serde_dynamo::from_item(i)?;
            Ok(m)
        }
        None => {
            tracing::info!("Item NOT found");
            Err(CacheError::NotFound)
        }
    }
}
