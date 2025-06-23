use crate::core::{civitai::get_all, params::Params};

pub mod raw;

pub fn extract(params: String) -> Result<Params, String>{
    let mut res = Params::default();
    // Get prompts
    let params_clone = params.clone();
    let prompts_split = params_clone.split("Steps:").nth(0)
        .ok_or(format!("Не удалось найти промпт"))?;
    let mut prompts_split = prompts_split.split("Negative prompt: ");
    res.pos_prompt = prompts_split.next()
        .ok_or(format!("Не удалось найти позитивный промпт"))?
        .to_string();
    res.neg_prompt = prompts_split.next()
        .unwrap_or_default()
        .to_string();

    // Get params
    let params_clone = params.clone();
    let params_split = params_clone.split("Steps:").nth(1)
        .ok_or(format!("Не удалось найти промпт"))?;
    for param in params_split.split(", "){
        match param{
            p if p.starts_with(" ") => 
                res.steps = p.split(" ").nth(1).unwrap().to_string(),
            p if p.starts_with("Sampler:") => 
                res.sampler = p.split(": ").nth(1).unwrap().to_string(),
            p if p.starts_with("CFG scale:") => 
                res.cfg_scale = p.split(": ").nth(1).unwrap().to_string(),
            p if p.starts_with("Seed:") => 
                res.seed = p.split(": ").nth(1).unwrap().to_string(),
            p if p.starts_with("Size:") => 
                res.size = p.split(": ").nth(1).unwrap().to_string(),
            p if p.starts_with("Model hash:") => 
                res.model_hash = p.split(": ").nth(1).unwrap().to_string(),
            p if p.starts_with("Model:") => 
                res.model = p.split(": ").nth(1).unwrap().to_string(),
            p if p.starts_with("VAE hash:") => 
                res.vae_hash = p.split(": ").nth(1).unwrap().to_string(),
            p if p.starts_with("VAE:") => 
                res.vae = p.split(": ").nth(1).unwrap().to_string(),
            p if p.starts_with("ADetailer") => 
                res.a_detailer = true,
            p if p.starts_with("Hires") || p.starts_with("Denoising strength:") => 
                res.hi_res = true,
            p if p.starts_with("forge_couple") => 
                res.forge_couple = true,
            p if p.starts_with("Clip skip:") ||
                p.starts_with("Schedule type:") ||
                p.starts_with("Version:") => {}
            _ => {}
        }
    }

    // Get loras
    let loras_raw = params_clone.split("Lora hashes: \"").nth(1)
        .unwrap_or_default();
    let loras_raw = loras_raw.split("\"").nth(0)
        .unwrap_or_default();
    for lora in loras_raw.split(", "){
        if lora.len() > 1{
            let mut lora_split = lora.split(": ");
            let lora = lora_split.next()
                .unwrap_or_default();
            let lora_hash = lora_split.next()
                .unwrap_or_default();
            res.loras.push((lora.to_string(), lora_hash.to_string(), String::new()));
        }
    }

    Ok(res)
}

pub async fn civitai_request(mut params: Params) -> Params{
    let req_loras = params.loras.iter().map(|lora|
        lora.1.to_owned()
    ).collect::<Vec<_>>();
    if let Ok(civit_ai) = get_all(&params.model_hash, &params.vae_hash, &req_loras).await{
        params.model_url = civit_ai.model_url;
        params.vae_url = civit_ai.vae_url;
        let def = (String::new(), String::new());
        for lora in params.loras.iter_mut(){
            let lora_url = civit_ai.loras_urls.iter().filter(|v| *v.0 == lora.1)
                .next().unwrap_or(&def);
            lora.2 = lora_url.1.clone();

        }
    };
    params
}

fn clean_text_bytes(data: &[u8]) -> Vec<u8> {
    data.iter()
        .copied()
        .filter(|b| {
            match b {
                0x09 |       // \t
                0x0A |       // \n
                0x0D |       // \r
                0x20..=0x7E  // ASCII-symbols
                => true,
                _ => false,
            }
        })
        .collect()
}