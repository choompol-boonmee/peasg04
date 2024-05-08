use crate::sg::ldp::base;
use crate::sg::{dcl, wk3};
use askama::Template;
use askama_axum;
use axum::extract::{Path, Query};
use serde::Deserialize;

#[derive(Template, Debug, Default, Clone)]
#[template(path = "pg2/ssfd2.html")]
pub struct FeederLoadProfile {
    pub ssid: String,
    pub ssnm: String,
    pub dayloads: Vec<DayLoad>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct DayLoad {
    pub day: usize,
    pub peak: dcl::LoadProfVal,
    pub avg: dcl::LoadProfVal,
    pub load_factor: f64,
    pub load_cnt: usize,
    pub load_null: usize,
    pub load_none: usize,
    pub load: Vec<dcl::LoadProfVal>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Param {
    pub a: Option<String>,
}

pub async fn handler(Path(fid): Path<String>, opt: Option<Query<Param>>) -> FeederLoadProfile {
    let base = base();
    let lpl = base.wk3_subst.read().await;
    let mut web = FeederLoadProfile::default();
    let parts = fid.split("_");
    let wds = parts.collect::<Vec<&str>>();
    web.ssid = wds[0].to_string();
    web.ssnm = wds[1].to_string();
    for ss in &*lpl {
        if ss.sbst == web.ssid {
            for ff in &ss.feed {
                let fid2 = ff.feed.trim();
                if fid2 == fid {
                    for d in 0..365 {
                        let mut dayload = DayLoad::default();
                        let d1 = d * 48;
                        for d2 in d1..d1 + 48 {
                            dayload.load.push(ff.time_r[d2].clone());
                        }
                        dayload.day = d + 1;
                        web.dayloads.push(dayload);
                    }
                }
            }
        }
    }
    web
}
