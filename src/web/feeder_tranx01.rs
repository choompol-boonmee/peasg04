use crate::sg::{ldp, ldp::base};
use crate::sg::{wk4, wk5};
use askama::Template;
use askama_axum;
use axum::extract::{Path, Query};
use serde::Deserialize;
use std::sync::Arc;
use thousands::Separable;
use tokio::sync::RwLock;

#[derive(Template, Debug, Default, Clone)]
#[template(path = "pg2/feeder_tranx01.html")]
pub struct FeederTranxInfo {
    pub ssid: String,
    pub fdid: String,
    pub trans: Vec<ldp::FeederTranx>,
}

impl FeederTranxInfo {
    async fn new(wk5prc: Arc<RwLock<wk5::Wk5Proc>>, ssid: String, fdid: String) -> Self {
        let wk5prc = wk5prc.read().await;
        let mut trans = Vec::new();
        for ss in &wk5prc.ssv {
            if ss.ssid == ssid {
                for fd in &ss.feeders {
                    if fd.fdid == fdid {
                        trans = fd.trans.clone();
                    }
                }
            }
        }
        FeederTranxInfo { ssid, fdid, trans }
    }
}
pub async fn handler(Path((ssid, fdid)): Path<(String, String)>) -> FeederTranxInfo {
    FeederTranxInfo::new(base().wk5prc.clone(), ssid, fdid).await
}
