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

#[derive(Template, Debug, Default, Clone)]
#[template(path = "wk1/subst.html")]
pub struct LoadProfList {
    pub ssv: Vec<Substation>,
}

#[derive(Debug, Clone, Default)]
pub struct Substation {
    pub sbst: String,
    pub prov: String,
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
    pub adj1: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Task {}

impl Task {
    pub async fn work(&mut self) {
        let base = base();
        let sbvc = base.sbvc_2022.read().await;
        let sbmp = base.sbmp_2022.read().await;
        let mut sbvc = sbvc.clone();
        let mut sbmp = sbmp.clone();
        let (mut c1, mut c2) = (0, 0);
        let (mut d1, mut d2) = (0, 0);
        for sb in &*sbvc {
            d1 += 1;
            let sb = sb.clone();
            let vfd = sbmp.get_mut(&sb).unwrap();
            for f in vfd {
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
                if tg > 0 {
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
                        }
                    }
                }
                let mut an = 0;
                for i in 1..tr.len() - 1 {
                    if let (
                        dcl::LoadProfVal::Value(lf),
                        dcl::LoadProfVal::None,
                        dcl::LoadProfVal::Value(rg),
                    ) = (&tr[i - 1], &tr[i], &tr[i + 1])
                    {
                        an += 1;
                        let ct = (lf + rg) / 2.0f32;
                        tr[i] = dcl::LoadProfVal::Value(ct);
                    }
                }
                if tg > 0 {
                    f.time_r = tr;
                }
            }
        }
        let mut ssv = Vec::new();
        //---------------
        let pvm = base.ss_pv_mp.read().await;
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
                let mut fd = FeederLoad::default();
                fd.sbst = f.sbst.to_string();
                fd.feed = f.feed.to_string();
                fd.fstday = -1;
                let tr = &f.time_r;
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
                let (mut gd, mut nu, mut no) = (0, 0, 0);
                for tt in tr {
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
                if tg >= 0 {
                    c1 += 1;
                    fd.fstday = tg;
                } else {
                    d2 += 1;
                }
                vldp.push(Box::new(fd));
            }
            ss.feed = vldp;
            ssv.push(ss);
        }
        {
            let load_prof_list = LoadProfList { ssv };
            let mut wk2_load_prof_list = base.wk2_load_prof_list.write().await;
            *wk2_load_prof_list = load_prof_list;
        }
    }
}
