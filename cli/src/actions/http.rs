use self::filters::{QueryParam, filter_generic};
use super::handlers::get_tags_pattern;
use crate::actions::handlers::{get_activities, get_balance, get_stats_tag_per_month, get_tags};
use crate::db::ArcMutDB;
use crate::db::DBActions;
use std::sync::Arc;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use warp::hyper::Method;
use warp::Filter;
/**
 * The definitions of APIs
 */
mod filters {
    use crate::{
        actions::utils::path_from_str,
        db::{ArcMutDB, DBActions},
    };
    use serde::Deserialize;
    use warp::{Filter, Rejection};

    #[derive(Deserialize)]
    pub struct QueryParam {        
        pub value: String
    }

    impl QueryParam {
        pub fn tokenize(&self) -> Vec<String> {
            self.value.split(",").map(str::to_string).collect()
        }
    }


    pub fn filter_generic<T>(
        path: &str,
        arc_db: ArcMutDB<T>,
    ) -> impl Filter<Extract = (ArcMutDB<T>,), Error = Rejection>
    where
        T: DBActions + Send,
    {
        let with_db = warp::any().map(move || arc_db.clone());

        let path_filter = path_from_str(path);

        warp::get()
            .and(path_filter)             
            .and(warp::path::end())
            .and(with_db)
    }
}


pub async fn http_server<T>(www_dir: String, www_port: u16, arc_db : ArcMutDB<T>) -> anyhow::Result<()> 
where 
    T: DBActions + Send + 'static
{
    let extract_param = warp::query::<QueryParam>();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::GET]);

    let www_root = warp::get().and(warp::fs::dir(www_dir));

    let api_activities = 
        filter_generic("api/activities", arc_db.clone())
        .and_then(get_activities);

    let api_balance = 
        filter_generic("api/balance", arc_db.clone())
        .and_then(get_balance);

    let api_tags = 
        filter_generic("api/tags", arc_db.clone())
        .and_then(get_tags);

    let api_stats_tag_per_month = 
        filter_generic("api/stats/per_month/tag", arc_db.clone())
        .and(extract_param)
        .and_then( move |arc_db : ArcMutDB<T>, param : QueryParam|  {
            get_stats_tag_per_month(arc_db, param.tokenize())
        });

    let api_tags_pattern = 
        filter_generic("api/tags/pattern", arc_db.clone())
        .and_then(get_tags_pattern);

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let route = www_root
        .or(api_activities.boxed())
        .or(api_balance.boxed())
        .or(api_tags.boxed())
        .or(api_stats_tag_per_month.boxed())
        .or(api_tags_pattern.boxed())
        .with(cors);

    Ok(warp::serve(route).run(([127, 0, 0, 1], www_port)).await)
}
