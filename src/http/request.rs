use super::method::Method;

pub struct Request {
    path: String,
    //option is whether none or some value of the Type specified
    query_string: Option<String>,
    method: Method,
}
