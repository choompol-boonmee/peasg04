use crate::sg::ldp::base;
use crate::sg::{wk4, wk5};
use askama::Template;
use askama_axum;
use axum::extract::{Path, Query};
use serde::Deserialize;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::sync::Arc;
use thousands::Separable;
use tokio::sync::RwLock;
use tokio::sync::{OwnedRwLockReadGuard, RwLockReadGuard};

#[derive(Template, Debug)]
#[template(path = "pg2/city_calc01.html")]
pub struct FeederPower03 {
    pub wk: OwnedRwLockReadGuard<wk5::Wk5Proc>,
    pub rw: Vec<Row>,
}
#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd)]
pub struct Row {
    s: usize,
    f: usize,
}

impl FeederPower03 {
    async fn new(wk5prc: Arc<RwLock<wk5::Wk5Proc>>) -> Self {
        let mut rw = Vec::new();
        let wk = wk5prc.read_owned().await;
        {
            for (s, ss) in wk.ssv.iter().enumerate() {
                for (f, fd) in ss.feeders.iter().enumerate() {
                    if fd.year_load.power_quality.pos_energy > 0f32 && fd.tx.tx_no > 0 {
                        rw.push(Row { s, f });
                    }
                }
            }
            rw.sort_by(|a, b| {
                let ea = wk.ssv[a.s].feeders[a.f].para1.energy;
                let eb = wk.ssv[b.s].feeders[b.f].para1.energy;
                let ta = wk.ssv[a.s].feeders[a.f].tx.tx_no;
                let tb = wk.ssv[b.s].feeders[b.f].tx.tx_no;
                let rs = if ea == eb {
                    tb.partial_cmp(&ta).unwrap()
                } else {
                    eb.partial_cmp(&ea).unwrap()
                };
                rs
            });
        }
        FeederPower03 { wk, rw }
    }
}
pub async fn handler() -> FeederPower03 {
    FeederPower03::new(base().wk5prc.clone()).await
}
