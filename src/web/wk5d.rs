use crate::sg::{dcl, dcl::DaVa, ldp, ldp::base, wk5};
use askama::Template;
use askama_axum;
use axum::extract::{Path, Query};
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use thousands::Separable;
use tokio::sync::RwLock;
use tokio::sync::{OwnedRwLockReadGuard, RwLockReadGuard};

#[derive(Template, Debug)]
#[template(path = "pg2/wk5c.html", escape = "none")]
pub struct ReportTemp {
    pub title: String,
    pub wk: OwnedRwLockReadGuard<wk5::Wk5Proc>,
}
fn rp(wk5prc: &wk5::Wk5Proc) -> &Report {
    &wk5prc.wk5d
}

fn sp(wk5prc: &mut wk5::Wk5Proc, rp: Report) {
    wk5prc.wk5d = rp;
}

impl ReportTemp {
    pub fn repo(&self) -> &Report {
        &self.wk.wk5d
    }
    async fn new(wk5prc: Arc<RwLock<wk5::Wk5Proc>>) -> Self {
        let wk = wk5prc.read_owned().await;
        let title = "SOLAR BATT CALCULATION".to_string();
        ReportTemp { wk, title }
    }
    pub fn cell(&self, r: &usize, c: &usize) -> String {
        let mut ce = rp(&self.wk).dava(&self.wk.ssv, *r, *c);
        if *c == 5 {
            if let DaVa::F32(v) = ce {
                let s = rp(&self.wk).rows[*r].s;
                let f = rp(&self.wk).rows[*r].f;
                let ss = &self.wk.ssv[s].ssid;
                let fd = &self.wk.ssv[s].feeders[f].fdid;
                ce = DaVa::Text(format!(
                    "<a href='/feeder_yrpw01/{}/{}'>{}</a>",
                    ss,
                    fd,
                    v.separate_with_commas()
                ));
            }
        }
        match ce {
            DaVa::Text(s) => s,
            DaVa::F32(f) => ((f * 100.0).floor() / 100.0).separate_with_commas(),
            DaVa::F64(f) => ((f * 100.0).floor() / 100.0).separate_with_commas(),
            DaVa::USZ(u) => format!("{}", u),
            d => format!("{:?}", d),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Report {
    pub rows: Vec<RepoRow1>,
    pub cols: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepoRow1 {
    pub s: usize, // substation
    pub f: usize, // feeder
    pub solar_energy: f32,
    pub solar_power: f32,
    pub solar_store: f32,
    pub solar_year_store: f32,
    pub solar_year_price: f32,
    pub solar_all_price: f32,
    pub solar_sum_price: f32,
}

pub async fn make_repo(wk5prc: &mut wk5::Wk5Proc, acfg: Arc<RwLock<dcl::Config>>) {
    let mut repo = rp(wk5prc).clone();

    let cfg = acfg.read().await;
    let sol = cfg.criteria.solar_energy_ratio;
    let max = cfg.criteria.bess_energy_max;
    let sot = cfg.criteria.solar_time_window;
    let yea = cfg.criteria.operate_year;
    let unp = cfg.criteria.bess_sell_per_mwh;
    let tt = [
        "NO",
        "SSID",
        "NAME",
        "PROV",
        "FDID",
        "Y.E.Mwh",
        "Y.MD.E.Mwh",
        "Y.Sol.E.Mwh",
        "Sol.P.Mw",
        "D.Sol.E.Mwh",
        "Y.Sol.E.Mwh",
        "Y.Sol.R.Bth",
        "12.Sol.R",
        "Acc Rev",
    ];
    for t in tt {
        repo.cols.push(t.to_string());
    }
    for s in 0..wk5prc.ssv.len() {
        for f in 0..wk5prc.ssv[s].feeders.len() {
            let mut rw = RepoRow1::default();
            rw.s = s;
            rw.f = f;
            let fd = &wk5prc.ssv[s].feeders[f];
            let en = fd.year_load.power_quality.pos_energy * sol as f32; // MWH = solar energy per year
            rw.solar_energy = en;
            let en = en / (365.0 * sot); // MW = solar power
            rw.solar_power = en;
            rw.solar_store = en * sot;
            rw.solar_year_store = rw.solar_store * 365.0;
            rw.solar_year_price = rw.solar_year_store * unp;
            rw.solar_all_price = rw.solar_year_price * yea;
            rw.solar_sum_price = rw.solar_all_price;
            if fd.year_load.power_quality.pos_energy > 0.0 && fd.tx.tx_no > 0 {
                repo.rows.push(rw);
            }
        }
    }
    repo.rows.sort_by(|a, b| {
        let a1 = wk5prc.ssv[a.s].feeders[a.f].para1.energy;
        let b1 = wk5prc.ssv[b.s].feeders[b.f].para1.energy;
        b1.partial_cmp(&a1).unwrap()
    });
    for r in 1..wk5prc.repo1.rows.len() {
        repo.rows[r].solar_sum_price +=
            repo.rows[r - 1].solar_sum_price + repo.rows[r].solar_all_price;
    }
    sp(wk5prc, repo);
}

impl Report {
    pub fn dava(&self, ssv: &Vec<wk5::Substation>, r: usize, c: usize) -> dcl::DaVa {
        let s = self.rows[r].s;
        let f = self.rows[r].f;
        let ss = &ssv[s];
        let fd = &ssv[s].feeders[f];
        match c {
            0 => DaVa::USZ(r + 1),
            1 => DaVa::Text(ss.ssid.to_string()),
            2 => DaVa::Text(ss.name.to_string()),
            3 => DaVa::Text(ss.prov.to_string()),
            4 => DaVa::Text(fd.fdid.to_string()),
            5 => DaVa::F32(fd.year_load.power_quality.pos_energy),
            6 => DaVa::F32(fd.year_load.power_quality.mid_day_energy),
            7 => DaVa::F32(self.rows[r].solar_energy),
            8 => DaVa::F32(self.rows[r].solar_power),
            9 => DaVa::F32(self.rows[r].solar_store),
            10 => DaVa::F32(self.rows[r].solar_year_store),
            11 => DaVa::F32(self.rows[r].solar_year_price),
            12 => DaVa::F32(self.rows[r].solar_all_price),
            13 => DaVa::F32(self.rows[r].solar_sum_price),
            n => DaVa::USZ(n),
        }
    }
}

pub async fn handler() -> ReportTemp {
    ReportTemp::new(base().wk5prc.clone()).await
}
