use crate::sg::ldp::base;
use crate::sg::{wk4, wk5};
use askama::Template;
use askama_axum;
use axum::extract::{Path, Query};
use serde::Deserialize;
use std::sync::Arc;
use thousands::Separable;
use tokio::sync::RwLock;

#[derive(Template, Debug, Default, Clone)]
#[template(path = "pg2/feeder_power02.html")]
pub struct FeederList {
    pub ssid: String,
    pub feeders: Vec<Feeder>,
}

#[derive(Debug, Default, Clone)]
pub struct Feeder {
    pub fdid: String,
    pub power_quality: wk4::PowerQuality,
    pub tx_no: usize,
    pub tx_pea: usize,
    pub tx_cus: usize,
    pub mt1_no: usize,
    pub mt3_no: usize,
    pub outage: f64,
}

impl FeederList {
    async fn new(wk5prc: Arc<RwLock<wk5::Wk5Proc>>, ssid: String) -> Self {
        let wk5prc = wk5prc.read().await;
        let mut feeders = Vec::new();
        for ss in &wk5prc.ssv {
            if ss.ssid == ssid {
                for fd in &ss.feeders {
                    if fd.year_load.data_quality.good == 0 {
                        continue;
                    }
                    let fdid = fd.fdid.to_string();
                    let power_quality = fd.year_load.power_quality.clone();
                    let outage = fd.outage_hour;
                    let (mut tx_no, mut mt1_no, mut mt3_no, mut tx_pea, mut tx_cus) =
                        (0, 0, 0, 0, 0);
                    for tx in &fd.trans {
                        tx_no += 1;
                        if tx.tx_own == "P" {
                            tx_pea += 1;
                        } else {
                            tx_cus += 1;
                        }
                        mt1_no += tx.mt_1_ph;
                        mt3_no += tx.mt_3_ph;
                    }
                    let fd = Feeder {
                        fdid,
                        power_quality,
                        tx_no,
                        tx_pea,
                        tx_cus,
                        mt1_no,
                        mt3_no,
                        outage,
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
    FeederList::new(base().wk5prc.clone(), ssid).await
}
