pub mod csv2db;
pub mod http;
pub mod handlers;
pub mod tagging;

pub mod utils {
    use std::collections::HashMap;
    use std::hash::Hash;

    use warp::filters::BoxedFilter;
    use warp::Filter;

    pub fn group_by<T, K, V, I, J>(input: &[T], key_fn: K, value_fn: V) -> HashMap<&I, Vec<&J>>
    where
        K: Fn(&T) -> &I,
        V: Fn(&T) -> &J,
        I: Eq + Hash + ?Sized,
        J: ?Sized,
    {
        let mut grouped: HashMap<&I, Vec<&J>> = HashMap::new();
        for e in input.iter() {
            grouped
                .entry(key_fn(e))
                .or_insert_with(Vec::new)
                .push(value_fn(e));
        }
        grouped
    }

    /**
     * Build path filter dynamically from a &str represetnting a URL path
     */
    pub fn path_from_str(path_str: &str) -> BoxedFilter<()> {
        path_str
            .split("/")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|t| warp::path(t.to_string()).boxed())            
            .fold(warp::any().boxed(), |acc, p| acc.and(p).boxed())
    }
}