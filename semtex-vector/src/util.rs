use candle_core::{
    bail,
    utils::{cuda_is_available, metal_is_available},
    Device, Error, Tensor,
};
use tokenizers::Encoding;

use crate::embedding::TokenizedOutput;

pub fn hub_load_safetensors_files(
    repo: &hf_hub::api::sync::ApiRepo,
    file_names: &[&str],
) -> candle_core::Result<Vec<std::path::PathBuf>> {
    let safetensors_files = file_names
        .iter()
        .map(|v| repo.get(v).map_err(Error::wrap))
        .collect::<candle_core::Result<Vec<_>>>()?;
    Ok(safetensors_files)
}

/// Loads the safetensors files for a model from the hub based on a json index file.
pub fn hub_load_safetensors(
    repo: &hf_hub::api::sync::ApiRepo,
    json_file: &str,
) -> candle_core::Result<Vec<std::path::PathBuf>> {
    let json_file = repo.get(json_file).map_err(Error::wrap)?;
    let json_file = std::fs::File::open(json_file)?;
    let json: serde_json::Value = serde_json::from_reader(&json_file).map_err(Error::wrap)?;
    let weight_map = match json.get("weight_map") {
        None => bail!("no weight map in {json_file:?}"),
        Some(serde_json::Value::Object(map)) => map,
        Some(_) => bail!("weight map in {json_file:?} is not a map"),
    };
    let mut safetensors_files = std::collections::HashSet::new();
    for value in weight_map.values() {
        if let Some(file) = value.as_str() {
            safetensors_files.insert(file.to_string());
        }
    }
    let safetensors_files = safetensors_files
        .iter()
        .map(|v| repo.get(v).map_err(Error::wrap))
        .collect::<candle_core::Result<Vec<_>>>()?;
    Ok(safetensors_files)
}

pub fn device(cpu: bool) -> candle_core::Result<Device> {
    if cpu {
        Ok(Device::Cpu)
    } else if cuda_is_available() {
        Ok(Device::new_cuda(0)?)
    } else if metal_is_available() {
        Ok(Device::new_metal(0)?)
    } else {
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        {
            println!(
                "Running on CPU, to run on GPU(metal), build this example with `--features metal`"
            );
        }
        #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
        {
            println!("Running on CPU, to run on GPU, build this example with `--features cuda`");
        }
        Ok(Device::Cpu)
    }
}

pub fn get_mask(size: usize, device: &Device) -> Tensor {
    let mask: Vec<_> = (0..size)
        .flat_map(|i| (0..size).map(move |j| u8::from(j > i)))
        .collect();
    Tensor::from_slice(&mask, (size, size), device).unwrap()
}

pub struct BertTokens {
    pub encoding: Encoding
}

pub fn token_ids(bert_tokens: &Vec<BertTokens>, device: &Device) -> Tensor {
    let token_ids = bert_tokens
        .iter()
        .map(|tokens| {
            let tokens = tokens.encoding.get_ids().to_vec();
            Ok(Tensor::new(tokens.as_slice(), device).unwrap())
        })
        .collect::<candle_core::Result<Vec<_>>>()
        .unwrap();
    Tensor::stack(&token_ids, 0).unwrap()
}


impl TokenizedOutput for BertTokens {}
