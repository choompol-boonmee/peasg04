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
#[template(path = "pg2/feeder_day01.html")]
pub struct FeederLoadOneDay {
    pub ssid: String,
    pub fdid: String,
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
impl FeederLoadOneDay {
    async fn new(
        wk4prc: Arc<RwLock<wk4::Wk4Proc>>,
        ssid: String,
        fdid: String,
        day: usize,
    ) -> Self {
        let wk4prc = wk4prc.read().await;
        let mut points = Vec::new();
        for ss in &wk4prc.ssv {
            if ss.sbst == ssid {
                for fd in &ss.feeders {
                    if fd.feed == fdid {
                        for li in 0..fd.year_load.loads.len() {
                            if (li + 1) == day {
                                let load = &fd.year_load.loads[li].load;
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
            }
        }
        FeederLoadOneDay {
            ssid,
            fdid,
            day,
            points,
        }
    }
}

pub async fn handler(Path((ssid, fdid, day)): Path<(String, String, String)>) -> FeederLoadOneDay {
    let day = day.parse::<usize>().unwrap();
    let bs = base();
    FeederLoadOneDay::new(bs.wk4_ssv.clone(), ssid, fdid, day).await
}
