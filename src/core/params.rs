use std::fmt;

#[derive(Default, Debug)]
pub struct Params{
    pub pos_prompt: String,
    pub neg_prompt: String,

    pub steps: String,
    pub sampler: String,
    pub size: String,

    pub model: String,
    pub model_hash: String,

    pub vae: String,
    pub vae_hash: String,
    pub loras: Vec<(String, String)>,

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
            params.push_str(format!(
                "➖: {}\n",
                self.neg_prompt.trim()
            ).as_str());
        };
        params.push_str(format!(
            "- 👣: {}; 🖌️: {}; 📏: {}\n\
            - 🎨: {}; 🎨💾: {}",
            self.steps, self.sampler, self.size,
            self.model, self.model_hash
        ).as_str());

        if self.vae.len() > 1{
            params.push_str(format!(
                "\n- 🔤: {}; 🔤💾: {}",
                self.vae, self.vae_hash
            ).as_str());
        };

        let mut loras = Vec::new();
        for lora in &self.loras{
            loras.push(
                format!(
                    "- 📖: {}; 📖💾: {}",
                    lora.0, lora.1
                )
            );
        }

        write!(f,
            "{}",
            if loras.len() > 0{
                [params, loras.join("\n")].join("\n")
            } else{
                params
            }
        )
    }
}