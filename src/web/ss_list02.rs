use crate::sg::ldp::base;
use crate::sg::wk4;
use askama::Template;
use askama_axum;
use axum::extract::{Path, Query};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Template, Debug, Default, Clone)]
#[template(path = "pg2/ss_list01.html")]
pub struct SSList {
    pub ssv: Vec<Substation>,
}
#[derive(Debug, Default, Clone)]
pub struct Substation {
    ssid: String,
    name: String,
    prov: String,
    fdno: usize,
    fdno_valid: usize,
    data_quality: wk4::DataQuality,
    data_cleaned: wk4::DataQuality,
}
impl SSList {
    async fn new(wk4prc: Arc<RwLock<wk4::Wk4Proc>>) -> SSList {
        let wk4prc = wk4prc.read().await;
        let mut ssv = Vec::new();
        for s in &wk4prc.ssv {
            let ssid = s.sbst.to_string();
            let name = s.name.to_string();
            let prov = s.prov.to_string();
            let fdno = s.feeders.len();
            let mut fdno_valid = 0;
            for f in &s.feeders {
                if f.year_load.data_quality.good > 0 {
                    fdno_valid += 1;
                }
            }
            let data_cleaned = s.year_load.data_cleaned.clone();
            let data_quality = s.year_load.data_quality.clone();
            ssv.push(Substation {
                ssid,
                name,
                prov,
                fdno,
                fdno_valid,
                data_quality,
                data_cleaned,
            })
        }
        ssv.sort_by(|a, b| a.prov.cmp(&b.prov));
        SSList { ssv }
    }
}

pub async fn handler() -> SSList {
    let bs = base();
    let wk4proc = bs.wk4_ssv.clone();
    SSList::new(wk4proc).await
}
