use std::io::Cursor;

pub fn fetch_url(f: &str, file_name: String) -> Result<(), String> {
    let url = get_url(f);
    // log::info!("start download = {}, path = {}", &url, file_name);

    let path = std::path::Path::new(&file_name);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let client = reqwest::blocking::Client::new();

    let response = client.get(url).send().map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("请求失败! code = {}", response.status()));
    }
    let bytes = response.bytes().map_err(|e| e.to_string())?;
    if bytes.len() < 1024 {
        return Err("文件大小不对".to_string());
    }
    let mut content = Cursor::new(bytes);
    let mut file = std::fs::File::create(file_name).map_err(|e| e.to_string())?;
    std::io::copy(&mut content, &mut file).map_err(|e| e.to_string())?;
    Ok(())
}

#[inline]
fn get_url(f: &str) -> String {
    format!(
        "http://hulab.rxnfinder.org/smi2img/{}",
        urlencoding::encode(f)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download() {
        crate::config::init_config();
        let r = fetch_url(
            "CC(CS(=O)(=O)C1=CC=C(C=C1)F)(C(=O)NC2=CC(=C(C=C2)C#N)C(F)(F)F)O",
            "123.png".to_string(),
        );
        log::info!("result = {:?}", r);
    }
}
