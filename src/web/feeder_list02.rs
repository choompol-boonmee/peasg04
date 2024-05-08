use crate::sg::ldp::base;
use crate::sg::wk4;
use askama::Template;
use askama_axum;
use axum::extract::{Path, Query};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Template, Debug, Default, Clone)]
#[template(path = "pg2/feeder_list02.html")]
pub struct FeederList {
    pub ssid: String,
    pub feeders: Vec<Feeder>,
}

#[derive(Debug, Default, Clone)]
pub struct Feeder {
    pub fdid: String,
    pub year_ldlen: usize,
    pub data_quality: wk4::DataQuality,
    pub data_cleaned: wk4::DataQuality,
    pub last_year_ldlen: usize,
}

impl FeederList {
    async fn new(wk4prc: Arc<RwLock<wk4::Wk4Proc>>, ssid: String) -> Self {
        let wk4prc = wk4prc.read().await;
        let mut feeders = Vec::new();
        for ss in &wk4prc.ssv {
            if ss.sbst == ssid {
                for fd in &ss.feeders {
                    if fd.year_load.data_quality.good == 0 {
                        continue;
                    }
                    let fdid = fd.feed.to_string();
                    let year_ldlen = fd.year_load.loads.len();
                    let data_cleaned = fd.year_load.data_cleaned.clone();
                    let data_quality = fd.year_load.data_quality.clone();
                    let last_year_ldlen = fd.last_year_load.loads.len();
                    let fd = Feeder {
                        fdid,
                        year_ldlen,
                        last_year_ldlen,
                        data_quality,
                        data_cleaned,
                    };
                    feeders.push(fd);
                }
            }
        }
        feeders.sort_by(|a, b| a.fdid.cmp(&b.fdid));
        FeederList { ssid, feeders }
    }
}
pub async fn handler(Path(ssid): Path<String>) -> FeederList {
    let bs = base();
    FeederList::new(bs.wk4_ssv.clone(), ssid).await
}
