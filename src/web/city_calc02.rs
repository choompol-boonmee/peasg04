use crate::sg::dcl::DaVa;
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
#[template(path = "pg2/city_calc02.html")]
pub struct CityCalc02 {
    pub wk: OwnedRwLockReadGuard<wk5::Wk5Proc>,
}

impl CityCalc02 {
    async fn new(wk5prc: Arc<RwLock<wk5::Wk5Proc>>) -> Self {
        let wk = wk5prc.read_owned().await;
        CityCalc02 { wk }
    }
    pub fn cell(&self, r: &usize, c: &usize) -> String {
        let ce = self.wk.repo1.dava(&self.wk.ssv, *r, *c);
        match ce {
            DaVa::Text(s) => s,
            DaVa::F32(f) => ((f * 100.0).floor() / 100.0).separate_with_commas(),
            DaVa::F64(f) => ((f * 100.0).floor() / 100.0).separate_with_commas(),
            DaVa::USZ(u) => format!("{}", u),
            d => format!("{:?}", d),
        }
    }
}
pub async fn handler() -> CityCalc02 {
    CityCalc02::new(base().wk5prc.clone()).await
}
