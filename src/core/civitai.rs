use serde_json::Value;
use reqwest::{Client, ClientBuilder};

#[derive(Default)]
pub struct CivitAI{
    pub model_url: String,
    pub vae_url: String,
    pub loras_urls: Vec<(String, String)>
}

pub async fn get_all(model: &String, vae: &String, loras: &Vec<String>) -> Result<CivitAI, ()>{
    let mut res = CivitAI::default();

    let client = ClientBuilder::new()
        .build().map_err(|_| ())?;
    res.model_url = match get_by_hash(model, &client).await{
        Ok(url) => url,
        Err(_) => "❌".to_string()
    };
    if !vae.is_empty(){
        res.vae_url = match get_by_hash(vae, &client).await{
            Ok(url) => url,
            Err(_) => "❌".to_string()
        };
    }
    for lora in loras{
        res.loras_urls.push((lora.to_owned(), match get_by_hash(lora, &client).await{
            Ok(url) => url,
            Err(_) => "❌".to_string()
        }));
    }
    Ok(res)
}

pub async fn get_by_hash(hash: &String, client: &Client) -> Result<String, ()>{
    let res = client
        .get(format!("https://civitai.com/api/v1/model-versions/by-hash/{hash}"))
        .send().await.map_err(|_| ())?
        .text().await.map_err(|_| ())?;

    let json = serde_json::from_str::<Value>(&res)
        .map_err(|_| ())?;
    Ok(format!(
        "https://civitai.com/models/{}?modelVersionId={}",
        json.get("modelId").ok_or(())?
        .as_i64().ok_or(())?,
        json.get("id").ok_or(())?
        .as_i64().ok_or(())?
    ))
}