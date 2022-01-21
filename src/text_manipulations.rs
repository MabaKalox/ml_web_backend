use actix_web::web::Json;
use actix_web::{post, HttpResponse};
use actix_web::rt::task::spawn_blocking;
use rust_bert::RustBertError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mode {
    Summarization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub text_list: Vec<String>,
    pub mode: Mode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub text_list: Vec<String>,
}

// Very expensive operation
fn summarize_text(text_vec: &Vec<String>) -> Result<Vec<String>, RustBertError> {
    use rust_bert::bart::{
        BartConfigResources, BartMergesResources, BartModelResources, BartVocabResources,
    };
    use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
    use rust_bert::resources::{RemoteResource, Resource};
    use tch::Device;

    let config_resource = Resource::Remote(RemoteResource::from_pretrained(
        BartConfigResources::DISTILBART_CNN_6_6,
    ));
    let vocab_resource = Resource::Remote(RemoteResource::from_pretrained(
        BartVocabResources::DISTILBART_CNN_6_6,
    ));
    let merges_resource = Resource::Remote(RemoteResource::from_pretrained(
        BartMergesResources::DISTILBART_CNN_6_6,
    ));
    let model_resource = Resource::Remote(RemoteResource::from_pretrained(
        BartModelResources::DISTILBART_CNN_6_6,
    ));

    let summarization_config = SummarizationConfig {
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource,
        num_beams: 1,
        length_penalty: 1.0,
        min_length: 56,
        max_length: 142,
        device: Device::Cuda(0),
        ..Default::default()
    };

    let summarization_model = SummarizationModel::new(summarization_config)?;

    return Ok(summarization_model.summarize(text_vec));
}

#[post("/text_manipulation")]
pub async fn text_manipulation(request: Json<Request>) -> HttpResponse {
    return match &request.mode {
        Mode::Summarization => match spawn_blocking(move || summarize_text(&request.text_list)).await  {
            Ok(Ok(r)) =>HttpResponse::Ok()
                .content_type(mime::APPLICATION_JSON)
                .json(Response { text_list: r }),
            _ => HttpResponse::InternalServerError().finish(),
        },
    };
}
