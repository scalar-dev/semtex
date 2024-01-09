use std::time::Instant;

use candle_core::{
    utils::{cuda_is_available, metal_is_available},
    DType, Device, Error, Tensor, Module,
};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::BertModel;
use candle_transformers::models::jina_bert::BertModel as JinaModel;
use candle_transformers::models::jina_bert::Config as JinaConfig;
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;

pub struct LoadedModel {
    model: JinaModel,
    tokenizer: Tokenizer,
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

/// Loads the safetensors files for a model from the hub based on a json index file.
pub fn hub_load_safetensors(
    repo: &hf_hub::api::sync::ApiRepo,
    json_file: &str,
) -> candle_core::Result<Vec<std::path::PathBuf>> {
    // let json_file = repo.get(json_file).map_err(Error::wrap)?;
    // let json_file = std::fs::File::open(json_file)?;
    // let json: serde_json::Value =
    //     serde_json::from_reader(&json_file).map_err(Error::wrap)?;
    // let weight_map = match json.get("weight_map") {
    //     None => bail!("no weight map in {json_file:?}"),
    //     Some(serde_json::Value::Object(map)) => map,
    //     Some(_) => bail!("weight map in {json_file:?} is not a map"),
    // };
    let mut safetensors_files = std::collections::HashSet::new();
    safetensors_files.insert("model.safetensors");
    // for value in weight_map.values() {
    // if let Some(file) = value.as_str() {
    // safetensors_files.insert(file.to_string());
    // }
    // }
    let safetensors_files = safetensors_files
        .iter()
        .map(|v| repo.get(v).map_err(Error::wrap))
        .collect::<candle_core::Result<Vec<_>>>()?;
    Ok(safetensors_files)
}

// pub fn build_model_and_tokenizer() -> candle_core::Result<LoadedModel> {
//     let device = device(true)?;
//     let model_id = "BAAI/bge-base-en-v1.5".to_string();
//     let revision = "main".to_string();

//     let api = Api::new().map_err(Error::wrap)?;
//     let repo = api.repo(Repo::with_revision(model_id, RepoType::Model, revision));


//     let config_filename = repo.get("config.json").unwrap();
//     let tokenizer = repo.get("tokenizer.json").unwrap();
//     let tokenizer = Tokenizer::from_file(tokenizer).unwrap();

//     let filenames = hub_load_safetensors(&repo, "model.safetensors.index.json").unwrap();
//     filenames
//         .iter()
//         .for_each(|fname| println!("File: {:?}", fname.to_str()));
//     let vb =
//         unsafe { VarBuilder::from_mmaped_safetensors(&filenames, DType::F32, &device).unwrap() };
//     let config = std::fs::read_to_string(config_filename)?;
//     let config = serde_json::from_str(&config).unwrap();

//     let model = BertModel::load(vb, &config).unwrap();
//     Ok(LoadedModel {
//         model: model,
//         tokenizer: tokenizer,
//     })
// }

pub fn build_model_and_tokenizer_jina() -> candle_core::Result<LoadedModel> {
    let device = device(true)?;
    let model_id = "jinaai/jina-embeddings-v2-small-en".to_string();

    let api = Api::new().map_err(Error::wrap)?;
    let repo = api.repo(Repo::new(model_id, RepoType::Model));

     let tokenizer = Api::new().unwrap()
                .repo(Repo::new(
                    "sentence-transformers/all-MiniLM-L6-v2".to_string(),
                    RepoType::Model,
                ))
                .get("tokenizer.json").unwrap();
    let tokenizer = Tokenizer::from_file(tokenizer).unwrap();

    let filenames = hub_load_safetensors(&repo, "model.safetensors.index.json").unwrap();
    filenames
        .iter()
        .for_each(|fname| println!("File: {:?}", fname.to_str()));
    let vb =
        unsafe { VarBuilder::from_mmaped_safetensors(&filenames, DType::F32, &device).unwrap() };

let config_filename = repo.get("config.json").unwrap();
    let config = std::fs::read_to_string(config_filename)?;
    let config = serde_json::from_str(&config).unwrap();

    let model = JinaModel::new(vb, &config).unwrap();
    Ok(LoadedModel {
        model: model,
        tokenizer: tokenizer,
    })
}



fn get_mask(size: usize, device: &Device) -> Tensor {
    let mask: Vec<_> = (0..size)
        .flat_map(|i| (0..size).map(move |j| u8::from(j > i)))
        .collect();
    Tensor::from_slice(&mask, (size, size), device).unwrap()
}

fn normalize_l2(v: &Tensor) -> candle_core::Result<Tensor> {
    v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)
}

pub fn to_vec(model: &mut LoadedModel, text: &[&str]) -> Vec<Vec<f32>> {
    let device = device(true).unwrap();
    let tokenizer = model
        .tokenizer
        .with_padding(None)
        .with_truncation(None)
        .unwrap();
    println!("Tokenizing");
    let tokens = tokenizer.encode_batch(text.to_vec(), true).unwrap();

    let token_ids = tokens
        .iter()
        .map(|tokens| {
            let tokens = tokens.get_ids().to_vec();
            Ok(Tensor::new(tokens.as_slice(), &device).unwrap())
        })
        .collect::<candle_core::Result<Vec<_>>>()
        .unwrap();
    let token_ids = Tensor::stack(&token_ids, 0).unwrap();
    // let token_type_ids = token_ids.zeros_like().unwrap();

    println!("Embedding {:?}", token_ids.shape());
    let now = Instant::now();
    let embeddings = model.model.forward(&token_ids/*, &token_type_ids*/).unwrap();
    println!("Elapsed: {}", now.elapsed().as_millis());
    println!("generated embeddings {:?}", embeddings.shape());
    let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3().unwrap();

    let embeddings = (embeddings.sum(1).unwrap() / (n_tokens as f64)).unwrap();
    normalize_l2(&embeddings).unwrap();

    println!("pooled embeddings {:?}", embeddings.shape());

    return embeddings.to_vec2().unwrap();
}
