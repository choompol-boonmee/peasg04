pub async fn run() {
    Task::default().ana().await
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
}

#[derive(Debug, Clone, Default)]
pub struct Task {
    //ssv: Vec<String>,
    //ssm: HashMap<String, Vec<Box<FeederLoad>>>,
}

impl Task {
    pub async fn ana(&mut self) {
        let base = base();
        let sbvc1 = base.sbvc_2022.read().await;
        let sbmp1 = base.sbmp_2022.read().await;
        let mut ssv = Vec::new();
        let (mut c1, mut c2) = (0, 0);
        let (mut d1, mut d2) = (0, 0);
        for sb in &*sbvc1 {
            let mut ss = Substation::default();
            ss.sbst = sb.clone();
            ss.feed = Vec::new();
            d1 += 1;
            let sb = sb.clone();
            let vfd = sbmp1.get(&sb).unwrap();
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
            let mut wk1_load_prof_list = base.wk1_load_prof_list.write().await;
            *wk1_load_prof_list = load_prof_list;
        }
    }
}
