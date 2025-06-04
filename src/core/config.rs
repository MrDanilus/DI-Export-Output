use std::fs;
use std::path::PathBuf;
use std::{env::var, path::Path};

pub fn resolve_path(file: &str) -> Result<PathBuf, String>{
    #[cfg(target_family="unix")]
    let local_app_data = match var("HOME"){
        Ok(res) => res,
        Err(err) => return Err(err.to_string())
    };
    #[cfg(target_family="unix")]
    let path = Path::new(&local_app_data);
    #[cfg(target_family="unix")]
    return Ok(path.join(format!("~/.airfish/sdimg-params/{file}")));

    #[cfg(target_family="windows")]
    let local_app_data = match var("LocalAppData"){
        Ok(res) => res,
        Err(err) => return Err(err.to_string())
    };
    #[cfg(target_family="windows")]
    let path = Path::new(&local_app_data);
    #[cfg(target_family="windows")]
    return Ok(path.join(format!(".airfish\\sdimg-params\\{file}")));
}

pub fn init_theme(){
    match resolve_path(""){
        Ok(path) => {
            let _ = fs::create_dir_all(&path);
            let _ = fs::write(path, b"light");
        },
        Err(_) => {}
    }
}

pub fn get_theme() -> String{
    match resolve_path("theme.txt"){
        Ok(path) => match fs::read_to_string(&path){
            Ok(res) => match res.as_str() {
                "light" | "dark" => res,
                _ => {
                    let _ = fs::write(path, b"light");
                    "light".to_string()
                }
            },
            Err(_) => {
                let _ = fs::write(path, b"light");
                "light".to_string()
            }
        },
        Err(_) => "light".to_string()
    }
}

pub fn write_theme(theme: String){
    match resolve_path("theme.txt"){
        Ok(path) => {
            let _ = fs::write(path, theme.as_bytes());
        },
        Err(_) => {}
    }
}