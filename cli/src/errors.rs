
use warp::reject::Reject;

#[derive(Debug)]
pub enum Errors {
    DBError(anyhow::Error)
}

impl Reject for Errors {}
