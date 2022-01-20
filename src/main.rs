mod db;
mod text_manipulations;
mod customers;

use warp;

#[tokio::main]
async fn main() {
    warp::serve(text_manipulations::routes::text_manipulations_route())
        .run(([127, 0, 0, 1], 3000))
        .await;
}
