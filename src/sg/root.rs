use crate::sg::ldp::base;
use crate::sg::wk2;
use askama::Template;
use askama_axum;

#[derive(Template, Debug, Default, Clone)]
#[template(path = "pg1/root.html")]
pub struct Root {}

pub async fn handler() -> Root {
    Root {}
}
