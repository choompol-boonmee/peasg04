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
#[template(path = "pg2/ss_day01.html")]
pub struct SubstLoadOneDay {
    pub ssid: String,
    pub day: usize,
    pub points: Vec<DataPoint>,
}
#[derive(Debug, Default, Clone)]
pub struct DataPoint {
    pub time: f32,
    pub value: f32,
    pub data: String,
    pub null: String,
    pub none: String,
}
impl SubstLoadOneDay {
    async fn new(wk4prc: Arc<RwLock<wk4::Wk4Proc>>, ssid: String, day: usize) -> Self {
        let wk4prc = wk4prc.read().await;
        let mut points = Vec::new();
        for ss in &wk4prc.ssv {
            if ss.sbst == ssid {
                for li in 0..ss.year_load.loads.len() {
                    if (li + 1) == day {
                        let load = &ss.year_load.loads[li].load;
                        for di in 0..load.len() {
                            let mut dp = DataPoint::default();
                            dp.time = di as f32 / 2f32;
                            match load[di] {
                                dcl::LoadProfVal::Value(va) => {
                                    dp.value = va;
                                    dp.data = format!("{}", va);
                                }
                                dcl::LoadProfVal::Null => {
                                    dp.null = "Y".to_string();
                                }
                                dcl::LoadProfVal::None => {
                                    dp.none = "Y".to_string();
                                } //_ => {}
                            }
                            points.push(dp);
                        }
                    }
                }
            }
        }
        SubstLoadOneDay { ssid, day, points }
    }
}

pub async fn handler(Path((ssid, day)): Path<(String, String)>) -> SubstLoadOneDay {
    let day = day.parse::<usize>().unwrap();
    let bs = base();
    SubstLoadOneDay::new(bs.wk4_ssv.clone(), ssid, day).await
}
