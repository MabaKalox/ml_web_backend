extern crate anyhow;
use std::convert::Infallible;
use warp::{self, http::StatusCode};

use crate::text_manipulations::models::TextManipulation;
use rust_bert::bart::{
    BartConfigResources, BartMergesResources, BartModelResources, BartVocabResources,
};
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
use rust_bert::resources::{RemoteResource, Resource};
use tch::Device;
use tokio::task::spawn_blocking;

pub async fn text_summarization(
    mut given_task: TextManipulation,
) -> Result<Box<dyn warp::Reply>, Infallible> {
    match spawn_blocking(move || {
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

        return if let Ok(model) = SummarizationModel::new(summarization_config) {
            given_task.text = model.summarize(&given_task.text);
            Ok(given_task)
        } else {
            Err(())
        };
    })
    .await
    {
        Ok(r) => Ok(Box::new(warp::reply::json(&r))),
        Err(_) => Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}
