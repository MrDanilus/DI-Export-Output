use std::{collections::HashMap, path::PathBuf};
use copypasta::{ClipboardContext, ClipboardProvider};

use crate::core::{civitai::get_by_hash, file::check_image, params::Params, parse::{self, raw}};

pub async fn save_to_clipboard(files: Vec<PathBuf>, civitai_value: bool) -> bool{
    let mut params: Vec<(PathBuf, Option<Params>)> = files.iter()
        .map(|file| match raw::parse(
            check_image(&file).unwrap().extensions_str().get(0).unwrap(), 
            &file
        ){
            Ok(params) => match parse::extract(params){
                Ok(res) => (file.clone(), Some(res)),
                Err(_) => (file.clone(), None)
            },
            Err(_) => (file.clone(), None)
        }).collect();

    if civitai_value{
        let mut hashes = HashMap::new();
        for (_, param) in &params{
            if let Some(param) = param{
                let model = &param.model_hash;
                if !model.is_empty() && !hashes.contains_key(model){
                    hashes.insert(model.clone(), None);
                }
                let vae = &param.vae_hash;
                if !vae.is_empty() && !hashes.contains_key(vae){
                    hashes.insert(vae.clone(), None);
                }
                for lora in &param.loras{
                    if !lora.1.is_empty() && !hashes.contains_key(&lora.1){
                        hashes.insert(lora.1.clone(), None);
                    }
                }
            }
        }
        
        let client = reqwest::Client::new();
        let futures: Vec<_> = hashes.clone().into_iter()
            .map(|hash|{
                let client = client.clone();
                tokio::spawn(async move {
                    let hash = hash.clone();
                    let res = match get_by_hash(&hash.0, &client).await{
                        Ok(res) => Some(res),
                        Err(_) => None
                    };
                    (hash.0, res)
                })
            }).collect();
        for f in futures.into_iter() {
            let res = f.await.unwrap();
            *hashes.get_mut(&res.0).unwrap() = res.1;
        }

        for param in params.iter_mut(){
            if let Some(param) = &mut param.1{
                // Model
                if let Some(model) = hashes.get(&param.model_hash){
                    param.model_url = match model{
                        Some(url) => url.clone(),
                        None => "🔎❌".to_string()
                    }
                }
                // Vae
                if let Some(vae) = hashes.get(&param.vae_hash){
                    param.vae_url = match vae{
                        Some(url) => url.clone(),
                        None => "🔎❌".to_string()
                    }
                }
                // Loras
                for lora in param.loras.iter_mut(){
                    if let Some(res_lora) = hashes.get(&lora.1){
                        lora.2 = match res_lora{
                            Some(url) => url.clone(),
                            None => "🔎❌".to_string()
                        }
                    }
                }
            }
        }
    }

    let mut res = String::new();
    if params.is_empty(){
        return false;
    } else{
        for (i, param) in params.iter().enumerate(){
            if let Some(res_param) = &param.1{
                res.push_str(&format!("[{:?}]\n{}", param.0, res_param));
                if i != params.len()-1{
                    res.push_str("\n---\n");
                }
            }
        }
    };
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(res).unwrap();
    true
}