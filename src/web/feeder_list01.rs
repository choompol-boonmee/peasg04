use crate::sg::ldp::base;
use crate::sg::wk4;
use askama::Template;
use askama_axum;
use axum::extract::{Path, Query};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Template, Debug, Default, Clone)]
#[template(path = "pg2/feeder_list01.html")]
pub struct FeederList {
    pub ssid: String,
    pub feeders: Vec<Feeder>,
}

#[derive(Debug, Default, Clone)]
pub struct Feeder {
    pub fdid: String,
    pub year_ldlen: usize,
    pub year_ldgood: usize,
    pub year_ldnull: usize,
    pub year_ldnone: usize,
    pub last_year_ldlen: usize,
}

impl FeederList {
    async fn new(wk4prc: Arc<RwLock<wk4::Wk4Proc>>, ssid: String) -> Self {
        let wk4prc = wk4prc.read().await;
        let mut feeders = Vec::new();
        for ss in &wk4prc.ssv {
            if ss.sbst == ssid {
                for fd in &ss.feeders {
                    let fdid = fd.feed.to_string();
                    let year_ldlen = fd.year_load.loads.len();
                    let year_ldgood = fd.year_load.data_quality.good;
                    let year_ldnull = fd.year_load.data_quality.null;
                    let year_ldnone = fd.year_load.data_quality.none;
                    let last_year_ldlen = fd.last_year_load.loads.len();
                    let fd = Feeder {
                        fdid,
                        year_ldlen,
                        year_ldgood,
                        year_ldnull,
                        year_ldnone,
                        last_year_ldlen,
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
