use crate::text_manipulations::handlers;
use crate::text_manipulations::models::TextManipulation;
use warp::Filter;

fn json_body() -> impl Filter<Extract = (TextManipulation,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn text_summarization(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("text_manipulations")
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::text_summarization)
}

pub fn text_manipulations_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    text_summarization()
}

