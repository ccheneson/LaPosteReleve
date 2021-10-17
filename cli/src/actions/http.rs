use warp::hyper::Method;
use std::sync::{Arc, Mutex};
use crate::actions::handlers::{get_activities, get_balance, get_tags, get_stats_tag_per_month};
use crate::db::DBActions;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use warp::Filter;
use filters::{path_activities, path_balance, path_tags, path_stats_tag_per_month};
use self::filters::{QueryParam, path_tags_pattern};
use super::handlers::get_tags_pattern;

/**
 * The definitions of APIs
 */
mod filters {
    use crate::db::{ArcMutDB, DBActions};
    use warp::{Filter, Rejection, path};
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct QueryParam {        
        pub value: String
    }

    impl QueryParam {
        pub fn tokenize(&self) -> Vec<String> {
            self.value.split(",").map(str::to_string).collect()
        }
    }


    pub fn path_activities<T>(arc_db: ArcMutDB<T>) -> impl Filter<Extract = (ArcMutDB<T>,), Error = Rejection> 
    where T: DBActions + Send 
    {
        let with_db = warp::any().map( move || arc_db.clone());

        warp::get()
         .and(path!("api"/"activities"))
         .and(path::end())         
         .and(with_db)
    }

    pub fn path_balance<T>(arc_db: ArcMutDB<T>) -> impl Filter<Extract = (ArcMutDB<T>,), Error = Rejection> 
    where T: DBActions + Send 
    {
        let with_db = warp::any().map( move || arc_db.clone());

        warp::get()
         .and(path!("api"/"balance"))
         .and(path::end())
         .and(with_db)
    }

    pub fn path_tags<T>(arc_db: ArcMutDB<T>) -> impl Filter<Extract = (ArcMutDB<T>,), Error = Rejection>
    where T: DBActions + Send 
    {
        let with_db = warp::any().map( move || arc_db.clone());

        warp::get()
         .and(path!("api"/"tags"))
         .and(path::end())
         .and(with_db)
    }

    pub fn path_stats_tag_per_month<T>(arc_db: ArcMutDB<T>) -> impl Filter<Extract = (QueryParam, ArcMutDB<T>,), Error = Rejection> 
    where T: DBActions + Send 
    {
        let with_db = warp::any().map( move || arc_db.clone());

        warp::get()
         .and(path!("api"/"stats"/ "per_month" / "tag"))
         .and(warp::query::<QueryParam>())
         .and(with_db)
    }

    pub fn path_tags_pattern<T>(arc_db: ArcMutDB<T>) -> impl Filter<Extract = (ArcMutDB<T>,), Error = Rejection>
    where T: DBActions + Send 
    {
        let with_db = warp::any().map( move || arc_db.clone());

        warp::get()
         .and(path!("api"/"tags"/ "pattern"))
         .and(with_db)
    }
}


pub async fn http_server<T>(www_dir: String, arc_db : Arc<Mutex<T>>) -> anyhow::Result<()> 
where T: DBActions + Send + 'static 
{

    let cors = warp::cors().allow_any_origin().allow_header("content-type").allow_methods(&[Method::GET]);
        

    let www_root = warp::get().and(warp::fs::dir(www_dir));

    let api_activities =  
        path_activities(Arc::clone(&arc_db))
        .and_then( move |db: Arc<Mutex<T>>| {
            get_activities(db)
        });

    
    let api_balance = 
        path_balance(Arc::clone(&arc_db))
        .and_then( move |db: Arc<Mutex<T>>| {
           get_balance(db)
        });
    
    let api_tags = 
        path_tags(Arc::clone(&arc_db))
        .and_then( move |db: Arc<Mutex<T>>| {
           get_tags(db)
        });

    let api_stats_tag_per_month = 
        path_stats_tag_per_month(Arc::clone(&arc_db))
        .and_then( move |query_param: QueryParam, db: Arc<Mutex<T>>| {
            get_stats_tag_per_month(db, query_param.tokenize())
        });

    let api_tags_pattern = 
        path_tags_pattern(Arc::clone(&arc_db))
        .and_then( move |db: Arc<Mutex<T>>| {
            get_tags_pattern(db)
        });

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");


    let route =  
        www_root
        .or(api_activities.boxed())
        .or(api_balance.boxed())
        .or(api_tags.boxed())
        .or(api_stats_tag_per_month.boxed())
        .or(api_tags_pattern.boxed())
        .with(cors);

    Ok(warp::serve(route).run(([127, 0, 0, 1], 3030)).await)
}
