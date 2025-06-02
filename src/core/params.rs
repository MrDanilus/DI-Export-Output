use std::{fmt, path::PathBuf};

use copypasta::{ClipboardContext, ClipboardProvider};

use crate::core::exif;

#[derive(Default, Debug)]
pub struct Params{
    pub pos_prompt: String,
    pub neg_prompt: String,

    pub steps: String,
    pub sampler: String,
    pub size: String,

    pub model: String,
    pub model_hash: String,
    pub model_url: String,

    pub vae: String,
    pub vae_hash: String,
    pub vae_url: String,

    pub loras: Vec<(String, String, String)>,

    pub a_detailer: bool,
    pub hi_res: bool,
    pub forge_couple: bool,

    pub cfg_scale: String,
    pub seed: String,
}
impl fmt::Display for Params {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut params = format!(
            "➕: {}\n",
            self.pos_prompt.trim()
        );

        if self.neg_prompt.trim().len() > 1{
            params.push_str(&format!(
                "➖: {}\n",
                self.neg_prompt.trim()
            ));
        };
        params.push_str(&format!(
            "- 👣: {}; 🖌️: {}; 📏: {}\n\
            - 🎨: {}: <{}>",
            self.steps, self.sampler, self.size,
            self.model, self.model_hash
        ));

        if !self.model_url.is_empty(){
            params.push_str(&format!(
                "; 🎨🔗: {}",
                self.model_url
            ));
        }

        if !self.vae.is_empty(){
            let mut res_vae = format!(
                "\n- 🔤: {}: <{}>",
                self.vae, self.vae_hash
            );
            if !self.vae_url.is_empty(){
                res_vae.push_str(&format!(
                    "; 🔤🔗: {}",
                    self.vae_url
                ));
            }
            params.push_str(&res_vae);
        };

        let mut loras = Vec::new();
        for lora in &self.loras{
            let mut res_lora = format!(
                "- 📖: {}: <{}>",
                lora.0, lora.1
            );
            if !lora.2.is_empty(){
                res_lora.push_str(&format!(
                    "; 📖🔗: {}",
                    lora.2
                ));
            }
            loras.push(res_lora);
        }

        write!(f,
            "{}",
            if !loras.is_empty(){
                [params, loras.join("\n")].join("\n")
            } else{
                params
            }
        )
    }
}

pub fn save_to_clipboard(files: Vec<PathBuf>, civitai_value: bool){
    let mut params = Vec::new();
    for file in files{
        match exif::parse_image(&file, civitai_value){
            Ok(res) => params.push(
                format!("[{file:?}]\n{}", res)
            ),
            Err(_) => {},
        }
    }
    let res = if params.is_empty(){
        "Нет параметров".to_string()
    } else{
        params.join("\n---\n")
    };
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(res).unwrap();
}