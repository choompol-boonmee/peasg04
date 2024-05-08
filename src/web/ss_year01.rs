use crate::sg::dcl;
use crate::sg::ldp::base;
use crate::sg::wk4;
use askama::Template;
use askama_axum;
use axum::extract::{Path, Query};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Template, Debug, Default, Clone)]
#[template(path = "pg2/ss_year01.html")]
pub struct SubstYearLoad {
    pub ssid: String,
    pub year_load: wk4::YearLoad,
}
impl SubstYearLoad {
    async fn new(wk4prc: Arc<RwLock<wk4::Wk4Proc>>, ssid: String) -> Self {
        let wk4prc = wk4prc.read().await;
        let mut year_load = wk4::YearLoad::default();
        for ss in &wk4prc.ssv {
            if ss.sbst == ssid {
                year_load = ss.year_load.clone();
            }
        }
        SubstYearLoad { ssid, year_load }
    }
}

pub async fn handler(Path((ssid)): Path<(String)>) -> SubstYearLoad {
    let bs = base();
    SubstYearLoad::new(bs.wk4_ssv.clone(), ssid).await
}
