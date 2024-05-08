use crate::sg::{dcl, dcl::DaVa, ldp, ldp::base, wk5};
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Repo1 {
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
}

pub async fn make_repo(wk5prc: &mut wk5::Wk5Proc, acfg: Arc<RwLock<dcl::Config>>) {
    let cfg = acfg.read().await;
    let sol = cfg.criteria.solar_energy_ratio;
    let max = cfg.criteria.bess_energy_max;
    let sot = cfg.criteria.solar_time_window;
    let tt = [
        "NO", "SSID", "NAME", "PROV", "FDID", "YR-ENG", "PEAK", "AVG", "ENG0", "TXNO", "txPEA",
        "txCUS", "METER1", "METER3", "OUT", "ENERGY", "POWER", "STORE",
    ];
    for t in tt {
        wk5prc.wk5a.cols.push(t.to_string());
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
            if fd.year_load.power_quality.pos_energy > 0.0 && fd.tx.tx_no > 0 {
                wk5prc.wk5a.rows.push(rw);
            }
        }
    }
    wk5prc.wk5a.rows.sort_by(|a, b| {
        let a1 = wk5prc.ssv[a.s].feeders[a.f].para1.energy;
        let b1 = wk5prc.ssv[b.s].feeders[b.f].para1.energy;
        b1.partial_cmp(&a1).unwrap()
    });
    /*
    for r in 1..wk5prc.repo1.rows.len() {
        wk5prc.repo1.rows[r].acc += wk5prc.repo1.rows[r - 1].acc + wk5prc.repo1.rows[r].cap;
    }
    */
}

impl Repo1 {
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
            6 => DaVa::F32(fd.year_load.power_quality.pos_peak),
            7 => DaVa::F32(fd.year_load.power_quality.pos_avg),
            8 => DaVa::F32(fd.para1.energy),
            9 => DaVa::USZ(fd.tx.tx_no),
            10 => DaVa::USZ(fd.tx.tx_pea),
            11 => DaVa::USZ(fd.tx.tx_cus),
            12 => DaVa::USZ(fd.tx.mt1_no),
            13 => DaVa::USZ(fd.tx.mt3_no),
            14 => DaVa::F64(fd.outage_hour),
            15 => DaVa::F32(self.rows[r].solar_energy),
            16 => DaVa::F32(self.rows[r].solar_power),
            17 => DaVa::F32(self.rows[r].solar_store),
            n => DaVa::USZ(n),
        }
    }
}
use askama::Template;
use askama_axum;
use axum::extract::{Path, Query};
use thousands::Separable;
use tokio::sync::{OwnedRwLockReadGuard, RwLockReadGuard};

#[derive(Template, Debug)]
#[template(path = "pg2/wk5a.html")]

pub struct Wk5a {
    pub wk: OwnedRwLockReadGuard<wk5::Wk5Proc>,
}

impl Wk5a {
    async fn new(wk5prc: Arc<RwLock<wk5::Wk5Proc>>) -> Self {
        let wk = wk5prc.read_owned().await;
        Wk5a { wk }
    }
    pub fn cell(&self, r: &usize, c: &usize) -> String {
        let ce = self.wk.wk5a.dava(&self.wk.ssv, *r, *c);
        match ce {
            DaVa::Text(s) => s,
            DaVa::F32(f) => ((f * 100.0).floor() / 100.0).separate_with_commas(),
            DaVa::F64(f) => ((f * 100.0).floor() / 100.0).separate_with_commas(),
            DaVa::USZ(u) => format!("{}", u),
            d => format!("{:?}", d),
        }
    }
}
pub async fn handler() -> Wk5a {
    Wk5a::new(base().wk5prc.clone()).await
}
