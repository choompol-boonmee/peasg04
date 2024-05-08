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
    &wk5prc.wk5t9
}
fn sp(wk5prc: &mut wk5::Wk5Proc, rp: Report) {
    wk5prc.wk5t9 = rp;
}

impl ReportTemp {
    pub fn repo(&self) -> &Report {
        &self.wk.wk5t9
    }
    async fn new(wk5prc: Arc<RwLock<wk5::Wk5Proc>>) -> Self {
        let wk = wk5prc.read_owned().await;
        let title = "FINANCIAL BENEFIT PROJECTION : WK5T";
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
                    v.form()
                ));
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
    pub prov: String,
    pub dtx: usize,
    pub m1p: usize,
    pub m3p: usize,
    pub cost: f32,
    pub fina: f32,
    pub econ: f32,
    pub firr: f32,
    pub eirr: f32,
	pub ener: f32,
}

/*
pub prov: String,
pub dtx: usize,
pub m1p: usize,
pub m3p: usize,
pub cost: f32,
pub fina: f32,
pub econ: f32,
pub firr: f32,
pub eirr: f32,
    */
const TT: [&str; 10] = [
    "NO", "PROV", "DTX", "M1P", "M3P", "COST", "FINA", "ECON", "FIRR", "ENER",
];

pub async fn make_repo(wk5prc: &mut wk5::Wk5Proc, acfg: Arc<RwLock<dcl::Config>>) {
    let mut repo = rp(wk5prc).clone();

    let cfg = acfg.read().await;
    for t in TT {
        repo.cols.push(t.to_string());
        repo.sums.push(DaVa::None);
    }

    let mut pvs = Vec::new();
    let mut pvm = HashMap::<String, Vec<usize>>::new();
    for (si, ss) in wk5prc.ssv.iter().enumerate() {
        if let Some(siv) = pvm.get_mut(&ss.prov) {
            siv.push(si);
        } else {
            pvm.insert(ss.prov.to_string(), vec![si]);
            pvs.push(ss.prov.to_string());
        }
    }

	let mut e0 = 0f32;
    for (si, ss) in wk5prc.ssv.iter().enumerate() {
		for fi in 0..wk5prc.ssv[si].feeders.len() {
			let fd = &wk5prc.ssv[si].feeders[fi];
			e0 += fd.year_load.power_quality.pos_energy;
		}
	}
	print!("e0: {}\n", e0);
	
	let mut sia = 0;
    for (pi, pv) in pvs.iter().enumerate() {
        if let Some(siv) = pvm.get(pv) {
			sia += siv.len();
//print!("pv:{} siv:{}\n", pv, siv.len());
            let mut rw = RepoRow1::default();
            rw.prov = pv.to_string();
            rw.dtx = 0;
            rw.m1p = 0;
            rw.m3p = 0;
            rw.cost = 0f32;
            rw.fina = 0f32;
            rw.econ = 0f32;
            rw.firr = 0f32;
            rw.eirr = 0f32;
			rw.ener = 0f32;
			//rw.ener = siv.len() as f32;
            //print!("{}\n", pv);
            for si in siv {
                let ss = &wk5prc.ssv[*si];
                for fi in 0..wk5prc.ssv[*si].feeders.len() {
                    let fd = &wk5prc.ssv[*si].feeders[fi];
                    //print!("  {} {} {}\n", fd.prov, fd.ssid, fd.fdid);
					let mut ok = false;
					if PRV1.contains(&fd.prov.as_str()) {
						//if fd.firr >= cfg.criteria.expect_min_firr {
							ok = true;
						//	}
					} else if PRV2.contains(&fd.prov.as_str()) {
						//if fd.firr >= -0.12 {
							ok = true;
						//}
					}
					if ok {
						rw.dtx += fd.tx.tx_no;
						rw.m1p += fd.tx.mt1_no;
						rw.m3p += fd.tx.mt3_no;
						rw.cost += fd.total_cost_npv;
						rw.fina += fd.financial_benefit_npv;
						rw.econ += fd.economic_benefit_npv;
						if !fd.firr.is_nan() {
							rw.firr += fd.firr;
						}
						rw.eirr += fd.ev_car_series[14];
						rw.ener += fd.year_load.power_quality.pos_energy;
						/*
						if !fd.eirr.is_nan() {
							rw.eirr += fd.eirr;
						}
						*/
					}
                }
                let flen = wk5prc.ssv[*si].feeders.len() as f32;
                rw.firr /= flen;
                //rw.eirr /= flen;
            }
            if rw.firr > 0f32 {
                repo.rows.push(rw);
            }
        }
    }
	print!("ALL feeders: {}\n", sia);
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
            1 => DaVa::Text(self.rows[r].prov.to_string()),
            2 => DaVa::USZ(self.rows[r].dtx),
            3 => DaVa::USZ(self.rows[r].m1p),
            4 => DaVa::USZ(self.rows[r].m3p),
            5 => DaVa::F32(self.rows[r].cost),
            6 => DaVa::F32(self.rows[r].fina),
            7 => DaVa::F32(self.rows[r].econ),
            8 => DaVa::F32(self.rows[r].firr * 100f32),
            9 => DaVa::F32(self.rows[r].ener),
            // ========
            n => DaVa::F32(fd.financial_benefit_series[n - 4]),
        }
    }
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

const PRV2: [&str; 9] = [
"ราชบุรี",
"ขอนแก่น",
"นครปฐม",
"สงขลา",
"นครราชสีมา",
"สุราษฎร์ธานี",
"กาญจนบุรี",
"นครสวรรค์",
"ตราด",
];

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
