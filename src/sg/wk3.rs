pub async fn run() {
    Task::default().work().await
}
use crate::sg::dcl;
use crate::sg::ldp::base;
use askama::Template;
use askama_axum;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::OnceLock;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Debug, Clone, Default)]
pub struct Substation {
    pub sbst: String,
    pub prov: String,
    pub name: String,
    pub feed: Vec<Box<FeederLoad>>,
}

#[derive(Debug, Clone, Default)]
pub struct FeederLoad {
    pub sbst: String,
    pub feed: String,
    pub fstday: i32,
    pub good: i32,
    pub null: i32,
    pub none: i32,
    pub adj_lead: i32,
    pub adj_one: i32,
    pub time_r: Vec<dcl::LoadProfVal>,
}

#[derive(Debug, Clone, Default)]
pub struct Task {}

impl Task {
    pub async fn work(&mut self) {
        let base = base();
        let sbvc = base.sbvc_2022.read().await;
        let sbmp = base.sbmp_2022.read().await;
        //let mut sbvc = sbvc.clone();
        //let mut sbmp = sbmp.clone();
        let (mut c1, mut c2) = (0, 0);
        let (mut d1, mut d2) = (0, 0);
        let pvm = base.ss_pv_mp.read().await;
        //let mut ssm = HashMap::new();
        let mut ssv = Vec::new();
        for sb in &*sbvc {
            let mut ss = Substation::default();
            ss.sbst = sb.clone();
            ss.feed = Vec::new();
            if let Some(p) = pvm.get(sb) {
                ss.prov = p.to_string();
            }
            d1 += 1;
            let sb = sb.clone();
            let vfd = sbmp.get(&sb).unwrap();
            let mut vldp = Vec::<Box<FeederLoad>>::new();
            for f in vfd {
                ss.name = f.name.to_string();
                let mut fd = FeederLoad::default();
                fd.sbst = f.sbst.to_string();
                fd.feed = f.feed.to_string();
                let mut tr = f.time_r.clone();
                let mut tg = -1;
                let mut gd = 0;
                for d in 0..365 {
                    let mut cc = 0;
                    let ts = d * 48;
                    for tt in ts..(ts + 48) {
                        if let crate::sg::dcl::LoadProfVal::Null = tr[tt] {
                            cc += 1;
                        } else if let crate::sg::dcl::LoadProfVal::None = tr[tt] {
                            cc += 1;
                        } else {
                            gd += 1;
                        }
                    }
                    if cc == 0 {
                        tg = d as i32;
                        break;
                    }
                }
                if tg < 0 {
                    continue;
                }
                if tg > (365 - 7) {
                    tg = 365 - 7;
                }
                for d1 in (0..tg).rev() {
                    let d2 = d1 + 7;
                    let dd1 = d1 * 48;
                    let dd2 = d2 * 48;
                    for i in 0..48 {
                        let dx1 = dd1 + i;
                        let dx2 = dd2 + i;
                        let dt = tr[dx2 as usize].clone();
                        tr[dx1 as usize] = dt;
                        fd.adj_lead += 1;
                    }
                }
                for i in 1..tr.len() - 1 {
                    if let (
                        dcl::LoadProfVal::Value(lf),
                        dcl::LoadProfVal::None,
                        dcl::LoadProfVal::Value(rg),
                    ) = (&tr[i - 1], &tr[i], &tr[i + 1])
                    {
                        fd.adj_one += 1;
                        let ct = (lf + rg) / 2.0f32;
                        tr[i] = dcl::LoadProfVal::Value(ct);
                    }
                }
                let (mut gd, mut nu, mut no) = (0, 0, 0);
                for tt in &tr {
                    if let crate::sg::dcl::LoadProfVal::Null = tt {
                        nu += 1;
                    } else if let crate::sg::dcl::LoadProfVal::None = tt {
                        no += 1;
                    } else {
                        gd += 1;
                    }
                }
                fd.null = nu;
                fd.none = no;
                fd.good = gd;
                fd.time_r = tr;
                vldp.push(Box::new(fd));
            }
            ss.feed = vldp;
            ssv.push(ss);
        }
        {
            let mut wk3_subst = base.wk3_subst.write().await;
            *wk3_subst = ssv;
        }
    }
}
