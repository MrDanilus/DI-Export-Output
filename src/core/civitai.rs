use serde_json::Value;
use ureq::Agent;

#[derive(Default)]
pub struct CivitAI{
    pub model_url: String,
    pub vae_url: String,
    pub loras_urls: Vec<(String, String)>
}

pub fn get_all(model: &String, vae: &String, loras: &Vec<String>) -> Result<CivitAI, ()>{
    let mut res = CivitAI::default();

    let agent = ureq::agent();
    res.model_url = match get_by_hash(model, &agent){
        Ok(url) => url,
        Err(_) => "❌".to_string()
    };
    if !vae.is_empty(){
        res.vae_url = match get_by_hash(vae, &agent){
            Ok(url) => url,
            Err(_) => "❌".to_string()
        };
    }
    for lora in loras{
        res.loras_urls.push((lora.to_owned(), match get_by_hash(lora, &agent){
            Ok(url) => url,
            Err(_) => "❌".to_string()
        }));
    }
    Ok(res)
}

fn get_by_hash(hash: &String, agent: &Agent) -> Result<String, ()>{
    let res = agent
        .get(format!("https://civitai.com/api/v1/model-versions/by-hash/{hash}"))
        .call().map_err(|_| ())?
        .body_mut()
        .read_to_string().map_err(|_| ())?;

    let json = serde_json::from_str::<Value>(&res)
        .map_err(|_| ())?;
    Ok(format!(
        "https://civitai.com/models/{}",
        json.get("modelId").ok_or(())?
        .as_i64().ok_or(())?
    ))
}