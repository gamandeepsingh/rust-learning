use super::method::Methods;
// optional as it is already imported by compiler
// pub enum Option<T> {
//     None, Some(T)
// }

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Methods,
}
