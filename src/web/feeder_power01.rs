use crate::sg::ldp::base;
use crate::sg::wk4;
use askama::Template;
use askama_axum;
use axum::extract::{Path, Query};
use serde::Deserialize;
use std::sync::Arc;
use thousands::Separable;
use tokio::sync::RwLock;

#[derive(Template, Debug, Default, Clone)]
#[template(path = "pg2/feeder_power01.html")]
pub struct FeederList {
    pub ssid: String,
    pub feeders: Vec<Feeder>,
}

#[derive(Debug, Default, Clone)]
pub struct Feeder {
    pub fdid: String,
    pub power_quality: wk4::PowerQuality,
    pub tx_no: usize,
    pub mt1_no: usize,
    pub mt3_no: usize,
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
                    let power_quality = fd.year_load.power_quality.clone();
                    let (mut tx_no, mut mt1_no, mut mt3_no) = (0, 0, 0);
                    for tx in &fd.trans {
                        tx_no += 1;
                        mt1_no += tx.mt_1_ph;
                        mt3_no += tx.mt_3_ph;
                    }
                    let fd = Feeder {
                        fdid,
                        power_quality,
                        tx_no,
                        mt1_no,
                        mt3_no,
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
