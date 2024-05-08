use crate::sg::{dcl, dcl::DaVa, ldp, ldp::base, uty::NumForm, wk5};
use askama::Template;
use askama_axum;
use axum::extract::{Path, Query};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
//use thousands::Separable;
use tokio::sync::RwLock;
use tokio::sync::{OwnedRwLockReadGuard, RwLockReadGuard};

#[derive(Template, Debug)]
#[template(path = "pg2/wk5f.html", escape = "none")]
pub struct ReportTemp {
    pub title: String,
    pub wk: OwnedRwLockReadGuard<wk5::Wk5Proc>,
}

fn rp(wk5prc: &wk5::Wk5Proc) -> &Report {
    &wk5prc.wk5t6
}
fn sp(wk5prc: &mut wk5::Wk5Proc, rp: Report) {
    wk5prc.wk5t6 = rp;
}

impl ReportTemp {
    pub fn repo(&self) -> &Report {
        &self.wk.wk5t6
    }
    async fn new(wk5prc: Arc<RwLock<wk5::Wk5Proc>>) -> Self {
        let wk = wk5prc.read_owned().await;
        let title = "INTERNAL RETURN RATE : WK5T";
        let title = title.to_string();

        ReportTemp { wk, title }
    }
    pub fn sum(&self, c: &usize) -> String {
        if *c == 0 {
            return format!("");
        }
        match rp(&self.wk).sums[*c] {
            DaVa::F32(v) => v.form(),
            DaVa::F64(v) => v.form(),
            DaVa::USZ(v) => v.form(),
            DaVa::I32(v) => v.form(),
            DaVa::I64(v) => v.form(),
            _ => format!(""),
        }
    }
    pub fn cell(&self, r: &usize, c: &usize) -> String {
        let mut ce = rp(&self.wk).dava(&self.wk.ssv, *r, *c);
        if *c == 1 {
            if let DaVa::Text(s) = ce {
                let s = rp(&self.wk).rows[*r].s;
                let p = &self.wk.ssv[s].prov.to_string();

                ce = DaVa::Text(format!("<a href='/wk5u1/{}'>{}</a>", p, p));
            }
        }
        match ce {
            DaVa::Text(s) => s,
            DaVa::F32(f) => f.form(),
            DaVa::F64(f) => f.form(),
            DaVa::USZ(u) => format!("{}", u),
            d => format!("{:?}", d),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Report {
    pub rows: Vec<RepoRow1>,
    pub cols: Vec<String>,
    pub sums: Vec<DaVa>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepoRow1 {
    pub s: usize, // substation
    pub f: usize, // feeder
}

const TT: [&str; 13] = [
    "NO", "PROV", "SSID", "SSNAME", "FDID", "DTX", "M1P", "M3P", "COST", "FR", "ER", "FIRR", "EIRR",
];

pub async fn make_repo(wk5prc: &mut wk5::Wk5Proc, acfg: Arc<RwLock<dcl::Config>>) {
    let mut repo = rp(wk5prc).clone();

    let cfg = acfg.read().await;
    for t in TT {
        repo.cols.push(t.to_string());
        repo.sums.push(DaVa::None);
    }
    let cfg = base().config.read().await;

    /*
    let syf = cfg.criteria.start_year_from_2022;
    let imy = cfg.criteria.implement_year;
    let opy = cfg.criteria.operate_year;
    let yrl = syf + imy + opy;
    let yrl = yrl as usize;
    for i in 0..yrl {
        let yr = 2022 + i + 1;
        repo.cols.push(format!("{}:B", yr));
        repo.sums.push(DaVa::None);
    }
    */
    //let re = Regex::new(r"[A-Z]{3}_[0-9][0-9][VY].*").unwrap();
    //let re = Regex::new(r"[A-Z]{3}_[0-9][0-9][VY].*").unwrap();
    let re = Regex::new(r"..._[0-9][0-9].+").unwrap();
    for s in 0..wk5prc.ssv.len() {
        for f in 0..wk5prc.ssv[s].feeders.len() {
            let mut rw = RepoRow1::default();
            rw.s = s;
            rw.f = f;
            let fd = &wk5prc.ssv[s].feeders[f];
            if re.is_match(fd.fdid.as_str()) {
                //if &fd.fdid[5..6] == "V" {
                //if fd.firr>=0.12 {
                //if fd.firr >= cfg.criteria.expect_min_firr {
				if PRV1.contains(&fd.prov.as_str()) {
					if fd.firr >= cfg.criteria.expect_min_firr {
						repo.rows.push(rw);
					}
				} else if PRV2.contains(&fd.prov.as_str()) {
					if fd.firr >= 0.0 {
						repo.rows.push(rw);
					}
				}
				/*
                if fd.firr >= 0.0 {
                    //if fd.tx.tx_no + fd.tx.mt1_no+ fd.tx.mt3_no > 5 {
                    //if fd.firr>=cfg.criteria.pea_min_firr {
                    //if fd.firr>=cfg.criteria.pea_min_firr {
                    //if fd.firr>=cfg.criteria.pea_min_firr && fd.eirr>=cfg.criteria.pea_min_eirr {
                    //if fd.total_cost>0.0 {
                    repo.rows.push(rw);
                    //}
                }
				*/
            }
        }
    }
    repo.rows.sort_by(|a, b| {
        let p1 = &wk5prc.ssv[a.s].feeders[a.f].prov;
        let p2 = &wk5prc.ssv[b.s].feeders[b.f].prov;
        let f1 = &wk5prc.ssv[a.s].feeders[a.f].fdid;
        let f2 = &wk5prc.ssv[b.s].feeders[b.f].fdid;
        if p1 == p2 {
            f1.cmp(&f2)
        } else {
            p1.cmp(&p2)
        }
        /*
        let a1 = &wk5prc.ssv[a.s].feeders[a.f].firr;
        let b1 = &wk5prc.ssv[b.s].feeders[b.f].firr;
        b1.partial_cmp(a1).unwrap()
        */
    });

    sum(&mut repo, &wk5prc.ssv);

    sp(wk5prc, repo);
}

const PRV1: [&str; 19] = [
"ระยอง",
"ชลบุรี",
"กระบี่",
"สระแก้ว",
"พระนครศรีอยุธยา",
"ฉะเชิงเทรา",
"สมุทรสาคร",
"ปทุมธานี",
"ตาก",
"บุรีรัมย์",
"ปราจีนบุรี",
"เพชรบุรี",
"ลพบุรี",
"เชียงใหม่",
"สระบุรี",
"ภูเก็ต",
"พิษณุโลก",
"ระนอง",
"สมุทรสงคราม",
];

const PRV2: [&str; 6] = [
"ราชบุรี",
"ขอนแก่น",
"นครปฐม",
"สงขลา",
"นครราชสีมา",
"สุราษฎร์ธานี",
];


impl Report {
    pub fn dava(&self, ssv: &Vec<wk5::Substation>, r: usize, c: usize) -> dcl::DaVa {
        let s = self.rows[r].s;
        let f = self.rows[r].f;
        let ss = &ssv[s];
        let fd = &ssv[s].feeders[f];
        match c {
            0 => DaVa::USZ(r + 1),
            1 => DaVa::Text(ss.prov.to_string()),
            2 => DaVa::Text(ss.ssid.to_string()),
            3 => DaVa::Text(ss.name.to_string()),
            4 => DaVa::Text(fd.fdid5.to_string()),
            5 => DaVa::USZ(fd.tx.tx_no),
            6 => DaVa::USZ(fd.tx.mt1_no),
            7 => DaVa::USZ(fd.tx.mt3_no),
            8 => DaVa::F32(fd.total_cost_npv),
            9 => DaVa::F32(fd.financial_benefit_npv),
            10 => DaVa::F32(fd.economic_benefit_npv),
            11 => DaVa::F32(fd.firr * 100.0),
            12 => DaVa::F32(fd.eirr * 100.0),
            n => DaVa::USZ(n),
        }
    }
}

pub async fn handler() -> ReportTemp {
    ReportTemp::new(base().wk5prc.clone()).await
}

fn sum(repo: &mut Report, ssv: &Vec<wk5::Substation>) {
    if repo.rows.len() > 0 {
        repo.sums[0] = DaVa::None;
        for ci in 1..repo.cols.len() {
            repo.sums[ci] = match repo.dava(ssv, 0, ci) {
                DaVa::F32(_) => DaVa::F32(0.0),
                DaVa::F64(_) => DaVa::F64(0.0),
                DaVa::I32(_) => DaVa::I32(0),
                DaVa::I64(_) => DaVa::I64(0),
                DaVa::USZ(_) => DaVa::USZ(0),
                _ => DaVa::None,
            };
        }
        let mut txno = 0;
        for (ri, rr) in repo.rows.iter().enumerate() {
            if let DaVa::USZ(v) = repo.dava(ssv, ri, 5) {
                txno += v;
            }

            for ci in 0..repo.cols.len() {
                repo.sums[ci] = match repo.dava(ssv, ri, ci) {
                    DaVa::F32(v1) => {
                        if let DaVa::F32(v2) = repo.sums[ci] {
                            DaVa::F32(v1 + v2)
                        } else {
                            DaVa::F32(0.0)
                        }
                    }
                    DaVa::F64(v1) => {
                        if let DaVa::F64(v2) = repo.sums[ci] {
                            DaVa::F64(v1 + v2)
                        } else {
                            DaVa::F64(0.0)
                        }
                    }
                    DaVa::I32(v1) => {
                        if let DaVa::I32(v2) = repo.sums[ci] {
                            DaVa::I32(v1 + v2)
                        } else {
                            DaVa::I32(0)
                        }
                    }
                    DaVa::I64(v1) => {
                        if let DaVa::I64(v2) = repo.sums[ci] {
                            DaVa::I64(v1 + v2)
                        } else {
                            DaVa::I64(0)
                        }
                    }
                    DaVa::USZ(v1) => {
                        if let DaVa::USZ(v2) = repo.sums[ci] {
                            DaVa::USZ(v1 + v2)
                        } else {
                            DaVa::USZ(0)
                        }
                    }
                    _ => DaVa::None,
                };
            }
        }
    }
}
