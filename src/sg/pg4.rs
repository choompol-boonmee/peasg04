use crate::sg::ldp::base;
use crate::sg::wk2;
use askama::Template;
use askama_axum;
use axum::extract::{Path, Query};
use serde::Deserialize;

#[derive(Template, Debug, Default, Clone)]
#[template(path = "pg1/pg4.html")]
pub struct Page4 {}

#[derive(Deserialize, Debug, Default)]
pub struct Param {
    pub a: Option<String>,
    pub b: Option<String>,
}

pub async fn handler(Path(user_id): Path<String>, opt: Option<Query<Param>>) -> Page4 {
    print!("{}\n", user_id);
    let Query(p) = opt.unwrap_or_default();
    print!("{:?}\n", p);
    Page4 {}
}
