pub async fn run() {
    Task::default().work2().await
}
pub async fn save_wk4prc() {
    Task::default().save_wk4prc().await
}
pub async fn load_wk4prc() {
    Task::default().load_wk4prc().await
}
use crate::sg::dcl;
use crate::sg::ldp;
use crate::sg::ldp::base;
use askama::Template;
use askama_axum;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, OnceLock};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Wk4Proc {
    pub ssv: Vec<Substation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Substation {
    pub sbst: String,
    pub prov: String,
    pub name: String,
    pub last_year: bool,
    pub feeders: Vec<Box<FeederLoad>>,
    pub year_load: YearLoad,
    pub last_year_load: YearLoad,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FeederLoad {
    pub sbst: String,
    pub feed: String,
    pub year_load: YearLoad,
    pub last_year_load: YearLoad,
    pub trans: Vec<ldp::FeederTranx>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct YearLoad {
    pub data_quality: DataQuality,
    pub data_cleaned: DataQuality,
    pub power_quality: PowerQuality,
    pub loads: Vec<DayLoad>,
}
impl YearLoad {
    pub async fn power(&mut self, stw: f32) {
        //print!("yr pw {}\n", self.loads.len());
        self.power_quality.pos_cnt = 0;
        self.power_quality.pos_sum = 0.0;
        self.power_quality.pos_energy = 0.0;
        self.power_quality.mid_day_energy = 0.0;
        self.power_quality.neg_cnt = 0;
        self.power_quality.neg_sum = 0.0;
        self.power_quality.neg_energy = 0.0;
        for dl in &mut self.loads {
            dl.power(stw).await;
            if dl.power_quality.pos_cnt > 0 {
                if dl.power_quality.pos_peak > self.power_quality.pos_peak {
                    self.power_quality.pos_peak = dl.power_quality.pos_peak;
                }
                self.power_quality.pos_cnt += dl.power_quality.pos_cnt;
                self.power_quality.pos_sum += dl.power_quality.pos_sum;
                self.power_quality.pos_energy += dl.power_quality.pos_energy;
                self.power_quality.mid_day_energy += dl.power_quality.mid_day_energy;
            }
            if dl.power_quality.neg_cnt > 0 {
                if dl.power_quality.neg_peak > self.power_quality.neg_peak {
                    self.power_quality.neg_peak = dl.power_quality.neg_peak;
                }
                self.power_quality.neg_cnt += dl.power_quality.neg_cnt;
                self.power_quality.neg_sum += dl.power_quality.neg_sum;
                self.power_quality.neg_energy += dl.power_quality.neg_energy;
            }
        }
        if self.power_quality.pos_cnt > 0 {
            self.power_quality.pos_avg =
                self.power_quality.pos_sum / self.power_quality.pos_cnt as f32;
        }
        if self.power_quality.neg_cnt > 0 {
            self.power_quality.neg_avg =
                self.power_quality.neg_sum / self.power_quality.neg_cnt as f32;
        }
        /*
        print!(
            "year power {}-{}\n",
            self.power_quality.pos_cnt, self.power_quality.neg_cnt,
        );
        */
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DayLoad {
    pub day: usize,
    pub load: Vec<dcl::LoadProfVal>,
    pub data_quality: DataQuality,
    pub data_cleaned: DataQuality,
    pub power_quality: PowerQuality,
}
impl DayLoad {
    pub async fn power(&mut self, stw: f32) {
        let sdwd = stw as usize;
        let (wl, wr) = (24 - sdwd, 24 + sdwd);
        // print!("wl:{} wr:{}\n", wl, wr);
        self.power_quality.pos_cnt = 0;
        self.power_quality.pos_sum = 0.0;
        self.power_quality.pos_energy = 0.0;
        self.power_quality.mid_day_energy = 0.0;
        self.power_quality.neg_cnt = 0;
        self.power_quality.neg_sum = 0.0;
        self.power_quality.neg_energy = 0.0;
        for (i, dl) in self.load.iter().enumerate() {
            if let dcl::LoadProfVal::Value(va) = dl {
                let v = *va;
                if v >= 0.0f32 {
                    if v > self.power_quality.pos_peak {
                        self.power_quality.pos_peak = v;
                    }
                    self.power_quality.pos_cnt += 1;
                    self.power_quality.pos_sum += v;
                    self.power_quality.pos_energy += v;
                    if i >= wl && i < wr {
                        self.power_quality.mid_day_energy += v;
                    }
                } else {
                    let v = -v;
                    if v > self.power_quality.neg_peak {
                        self.power_quality.neg_peak = v;
                    }
                    self.power_quality.neg_cnt += 1;
                    self.power_quality.neg_sum += v;
                    self.power_quality.neg_energy += v;
                }
            }
        }
        if self.power_quality.pos_cnt > 0 {
            self.power_quality.pos_avg =
                self.power_quality.pos_sum / self.power_quality.pos_cnt as f32;
            self.power_quality.pos_energy *= 0.5f32;
            self.power_quality.mid_day_energy *= 0.5f32;
        }
        if self.power_quality.neg_cnt > 0 {
            self.power_quality.neg_avg =
                self.power_quality.neg_sum / self.power_quality.neg_cnt as f32;
            self.power_quality.neg_energy *= 0.5f32;
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PowerQuality {
    pub pos_peak: f32,
    pub pos_cnt: usize,
    pub pos_sum: f32,
    pub pos_avg: f32,
    pub pos_energy: f32,
    pub neg_peak: f32,
    pub neg_cnt: usize,
    pub neg_sum: f32,
    pub neg_avg: f32,
    pub neg_energy: f32,
    pub mid_day_energy: f32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataQuality {
    pub fstday: Option<usize>,
    pub adj_lead: usize,
    pub adj_one: usize,
    pub adj_fill: usize,
    pub adj_last: usize,
    pub good: usize,
    pub null: usize,
    pub none: usize,
}

#[derive(Debug, Default)]
pub struct Task {
    pub subst_list: Arc<RwLock<Vec<Substation>>>,
}

impl Task {
    pub async fn work2(&mut self) {
        self.read().await;
        self.calc_adjust().await;
        self.calc_recheck().await;
        self.sum_subst().await;
        self.power_quality().await;
        self.calc_trans().await;
        {
            let bs = base();
            let mut wk4ssv = bs.wk4_ssv.write().await;
            let ssv = self.subst_list.read().await.clone();
            *wk4ssv = Wk4Proc { ssv };
        }
    }

    pub async fn load_wk4prc(&mut self) {
        if let Ok(file) = File::open("data/wk4prc.bin") {
        print!("...2\n");
            let rd = BufReader::new(file);
            if let Ok(wk4prc) = bincode::deserialize_from::<BufReader<File>, Wk4Proc>(rd) {
                let bs = base();
                let mut wk4ssv = bs.wk4_ssv.write().await;
                *wk4ssv = wk4prc;
            }
        }
    }

    pub async fn save_wk4prc(&mut self) {
        self.read().await;
        self.calc_adjust().await;
        self.calc_recheck().await;
        self.sum_subst().await;
        self.power_quality().await;
        self.calc_trans().await;
        //self.calc_adjust2().await;
        let ssv = self.subst_list.read().await.clone();
        let wk4prc = Wk4Proc { ssv };
        if let Ok(se) = bincode::serialize(&wk4prc) {
            std::fs::write("data/wk4prc.bin", se).unwrap();
        }
    }

    pub async fn calc_adjust2(&mut self) {
        let mut ssv = self.subst_list.write().await;
        print!("calc adjust2\n");
        for ss in &*ssv {
            for fd in &ss.feeders {
                let (mut t1, mut t2) = (0, 0);
                for di in 0..fd.year_load.loads.len() {
                    for hi in 0..fd.year_load.loads[di].load.len() {
                        t1 += 1;
                        if let dcl::LoadProfVal::Value(v) = fd.year_load.loads[di].load[hi] {
                        } else {
                            t2 += 1;
                        }
                    }
                }
                if t2 > 0 {
                    print!("ss:{} fd:{} t1:{} t2:{}\n", ss.sbst, fd.feed, t1, t2);
                }
            }
        }
        //print!("t1:{} t2:{}\n", t1, t2);
        print!("finished\n");
    }

    pub async fn calc_trans(&mut self) {
        let base = base();
        let fd_tx_info = base.fd_tx_info.read().await;
        let mut fd_keys = HashSet::new();
        let mut ssv = self.subst_list.write().await;
        let (mut txn, mut mtn, mut fdn, mut efn, mut txo, mut mto, mut fdo) = (0, 0, 0, 0, 0, 0, 0);
        //let re = Regex::new(r"[A-Z]{3}_[0-9][0-9][VY].*").unwrap();
        //let re = Regex::new(r"..._[0-9][0-9][V.+").unwrap();
        let re = Regex::new(r"..._[0-9][0-9][VWB].+").unwrap();
        let mut fdcnt: HashMap<String, i32> = fd_tx_info
            .fdtxmp
            .iter()
            .map(|(k, v)| (k.to_string(), 0))
            .collect();
        for ss in &mut *ssv {
            for fd in &mut ss.feeders {
                if re.is_match(fd.feed.as_str()) {
                } else {
                    continue;
                }
                let fd0 = &fd.feed[0..6];
                let fd0 = fd0.to_string();
                let pw = fd.year_load.power_quality.pos_energy;
                if pw == 0f32 {
                    continue;
                }
                if let Some(txfv) = fd_tx_info.fdtxmp.get(&fd0) {
                    if let Some(c) = fdcnt.get_mut(&fd0) {
                        *c += 1;
                    }
                    if let Some(fd) = fd_keys.get(&fd0) {
                        txo += txfv.len();
                        for tx in txfv {
                            mto += (tx.mt_1_ph + tx.mt_3_ph);
                        }
                        fdo += 1;
                    } else {
                        txn += txfv.len();
                        for tx in txfv {
                            mtn += (tx.mt_1_ph + tx.mt_3_ph);
                        }
                        fdn += 1;
                        fd_keys.insert(fd0);
                    }
                    fd.trans.append(&mut txfv.clone());
                } else {
                    efn += 1;
                }
            }
        }
        print!(
            "tx:{} mt:{} fd:{}, to: tx:{} mt:{} fd:{} ef:{}\n",
            txn, mtn, fdn, txo, mto, fdo, efn
        );
        let mut bnk = vec![];
        for (k, v) in fdcnt {
            if v == 0 {
                if let Some(txfv) = fd_tx_info.fdtxmp.get(&k) {
                    bnk.push((k.to_string(), txfv.clone()));
                }
            }
        }
        bnk.sort_by(|a, b| a.0.cmp(&b.0));
        for (i, v) in bnk.iter().enumerate() {
            print!(" {}.{} - {}\n", i, v.0, v.1.len());
        }
    }

    pub async fn power_quality(&mut self) {
        let cfg = base().config.clone();
        let cfg = cfg.read().await;
        let stw = cfg.criteria.solar_time_window;
        let mut ssv = self.subst_list.write().await;
        for ss in &mut *ssv {
            for fd in &mut ss.feeders {
                fd.year_load.power(stw).await;
                fd.last_year_load.power(stw).await;
            }
            ss.year_load.power(stw).await;
            ss.last_year_load.power(stw).await;
        }
    }

    pub async fn sum_subst(&mut self) {
        let mut ssv = self.subst_list.write().await;
        for ss in &mut *ssv {
            let mut ss_val = [0f32; 365 * 48];
            for fd in &mut ss.feeders {
                for (di, dl) in fd.year_load.loads.iter().enumerate() {
                    for (hi, hl) in dl.load.iter().enumerate() {
                        let ii = di * 48 + hi;
                        if let dcl::LoadProfVal::Value(v) = hl {
                            ss_val[ii] += v;
                        } else {
                            //print!("ERR {} {} {} {}\n", ss.sbst, fd.feed, di, hi);
                        }
                    }
                }
            }
            ss.year_load = YearLoad::default();
            for di in 0..365 {
                let mut day_load = DayLoad::default();
                day_load.day = di + 1;
                for hi in 0..48 {
                    let ii = di * 48 + hi;
                    day_load.load.push(dcl::LoadProfVal::Value(ss_val[ii]));
                }
                ss.year_load.loads.push(day_load);
            }
        }
    }

    pub async fn read(&mut self) {
        let base = base();
        let sbvc = base.sbvc_2022.read().await;
        let sbmp = base.sbmp_2022.read().await;
        let sbmp0 = base.sbmp_2021.read().await;
        let pvm = base.ss_pv_mp.read().await;
        let mut ssv = Vec::new();
        for sb in &*sbvc {
            let mut ss = Substation::default();
            ss.sbst = sb.clone();
            if let Some(p) = pvm.get(sb) {
                ss.prov = p.to_string();
            }
            let vfd = sbmp.get(&sb.to_string()).unwrap();
            let vfd0 = sbmp0.get(&sb.to_string());
            ss.feeders = Vec::<Box<FeederLoad>>::new();
            if let Some(vfd0) = vfd0 {
                ss.last_year = true;
            }
            for f in vfd {
                ss.name = f.name.to_string();
                let mut fd = FeederLoad::default();
                fd.sbst = f.sbst.to_string();
                fd.feed = f.feed.trim().to_string();
                //let tr = &f.time_r;
                let mut f0: Option<&Box<dcl::FeederLoad>> = None;
                if let Some(vfd0) = vfd0 {
                    for f1 in vfd0 {
                        if f.feed == f1.feed {
                            f0 = Some(&f1);
                            break;
                        }
                    }
                }
                for d in 0..365 {
                    let mut load = DayLoad::default();
                    let mut load0 = DayLoad::default();
                    load.day = d + 1;
                    let ts = d * 48;
                    for tt in ts..(ts + 48) {
                        let va = f.time_r[tt].clone();
                        match va {
                            dcl::LoadProfVal::Value(vi) => load.data_quality.good += 1,
                            dcl::LoadProfVal::Null => load.data_quality.null += 1,
                            dcl::LoadProfVal::None => load.data_quality.none += 1,
                        }
                        load.load.push(va);
                    }
                    if let Some(f0) = f0 {
                        //print!("LAST f0 {}\n", sb);
                        for tt in ts..(ts + 48) {
                            let va = f0.time_r[tt].clone();
                            match va {
                                dcl::LoadProfVal::Value(vi) => load0.data_quality.good += 1,
                                dcl::LoadProfVal::Null => load0.data_quality.null += 1,
                                dcl::LoadProfVal::None => load0.data_quality.none += 1,
                            }
                            load0.load.push(va);
                        }
                    }
                    fd.year_load.loads.push(load);
                    fd.last_year_load.loads.push(load0);
                }
                for ld in &fd.year_load.loads {
                    fd.year_load.data_quality.good += ld.data_quality.good;
                    fd.year_load.data_quality.null += ld.data_quality.null;
                    fd.year_load.data_quality.none += ld.data_quality.none;
                    fd.year_load.data_quality.adj_lead += ld.data_quality.adj_lead;
                    fd.year_load.data_quality.adj_one += ld.data_quality.adj_one;
                }
                for ld in &fd.last_year_load.loads {
                    fd.last_year_load.data_quality.good += ld.data_quality.good;
                    fd.last_year_load.data_quality.null += ld.data_quality.null;
                    fd.last_year_load.data_quality.none += ld.data_quality.none;
                    fd.last_year_load.data_quality.adj_lead += ld.data_quality.adj_lead;
                    fd.last_year_load.data_quality.adj_one += ld.data_quality.adj_one;
                }
                ss.feeders.push(Box::new(fd));
            }
            ssv.push(ss);
        }
        {
            let mut subst_list = self.subst_list.write().await;
            *subst_list = ssv;
        }
    }

    pub async fn calc_adjust(&mut self) {
        let mut ssv = self.subst_list.write().await;
        for ss in &mut *ssv {
            for fd in &mut ss.feeders {
                Self::calc_adj_year(&ss.sbst, &fd.feed, &mut fd.year_load).await;
                if ss.last_year {
                    Self::calc_adj_year(&ss.sbst, &fd.feed, &mut fd.last_year_load).await;
                }
            }
        }
    }

    pub async fn calc_recheck(&mut self) {
        let mut ssv = self.subst_list.write().await;
        for ss in &mut *ssv {
            for fd in &mut ss.feeders {
                for dl in &mut fd.year_load.loads {
                    for hl in &dl.load {
                        match hl {
                            dcl::LoadProfVal::Value(_) => {
                                dl.data_cleaned.good += 1;
                            }
                            dcl::LoadProfVal::Null => {
                                dl.data_cleaned.null += 1;
                            }
                            dcl::LoadProfVal::None => {
                                dl.data_cleaned.none += 1;
                            }
                        }
                    }
                    fd.year_load.data_cleaned.good += dl.data_cleaned.good;
                    fd.year_load.data_cleaned.null += dl.data_cleaned.null;
                    fd.year_load.data_cleaned.none += dl.data_cleaned.none;
                }
                if fd.year_load.data_quality.good > 0 {
                    ss.year_load.data_cleaned.good += fd.year_load.data_cleaned.good;
                    ss.year_load.data_cleaned.null += fd.year_load.data_cleaned.null;
                    ss.year_load.data_cleaned.none += fd.year_load.data_cleaned.none;
                    ss.year_load.data_quality.adj_lead += fd.year_load.data_quality.adj_lead;
                    ss.year_load.data_quality.adj_one += fd.year_load.data_quality.adj_one;
                    ss.year_load.data_quality.adj_fill += fd.year_load.data_quality.adj_fill;
                    ss.year_load.data_quality.adj_last += fd.year_load.data_quality.adj_last;
                }
            }
        }
    }

    pub async fn calc_adj_year(ssid: &String, fdid: &String, mut p_year_load: &mut YearLoad) {
        for (i, load) in p_year_load.loads.iter().enumerate() {
            if load.data_quality.null + load.data_quality.none == 0 {
                p_year_load.data_quality.fstday = Some(i);
                break;
            }
        }
        if let Some(fstd) = p_year_load.data_quality.fstday {
            if fstd > 0 {
                for d1 in (0..fstd).rev() {
                    let mut d2 = d1 + 7;
                    if d2 >= p_year_load.loads.len() {
                        d2 = p_year_load.loads.len() - 1;
                    }
                    p_year_load.data_quality.adj_lead += 1;
                    p_year_load.loads[d1] = p_year_load.loads[d2].clone();
                }
            }
        }
        for ld in &mut p_year_load.loads {
            if ld.load.len() > 0 {
                for i in 1..(ld.load.len() - 1) {
                    if ld.load[i] == dcl::LoadProfVal::None || ld.load[i] == dcl::LoadProfVal::Null
                    {
                        if let (dcl::LoadProfVal::Value(d0), dcl::LoadProfVal::Value(d1)) =
                            (&ld.load[i - 1], &ld.load[i + 1])
                        {
                            p_year_load.data_quality.adj_one += 1;
                            ld.load[i] = dcl::LoadProfVal::Value((d1 + d0) / 2f32);
                        }
                    }
                }
            }
        }
        for di in 0..p_year_load.loads.len() {
            if p_year_load.loads[di].load.len() != 48 {
                continue;
            }
            let dl = &mut p_year_load.loads[di];
            let (mut mn_po_va, mut mx_po_va) = (dl.load.len() - 1, 0);
            for hi in 0..dl.load.len() {
                if let dcl::LoadProfVal::Value(_) = dl.load[hi] {
                    if hi < mn_po_va {
                        mn_po_va = hi;
                    }
                    if hi > mx_po_va {
                        mx_po_va = hi;
                    }
                }
            }
            if mn_po_va != dl.load.len() - 1 && mn_po_va > 0 {
                for hi in 0..mn_po_va {
                    dl.load[hi] = dl.load[mn_po_va].clone();
                    dl.data_quality.adj_fill += 1;
                }
            }
            if mx_po_va > 0 && mx_po_va < dl.load.len() - 1 {
                for hi in mx_po_va + 1..dl.load.len() {
                    dl.load[hi] = dl.load[mx_po_va].clone();
                    dl.data_quality.adj_fill += 1;
                }
            }
            if mx_po_va > mn_po_va {
                for h1 in 1..dl.load.len() - 1 {
                    if let dcl::LoadProfVal::Value(_) = dl.load[h1] {
                    } else {
                        dl.load[h1] = dl.load[h1 - 1].clone();
                        dl.data_quality.adj_fill += 1;
                    }
                }
                if let dcl::LoadProfVal::Value(_) = dl.load[0] {
                } else {
                    dl.load[0] = dl.load[1].clone();
                    dl.data_quality.adj_fill += 1;
                }
                let lr = dl.load.len();
                if lr > 2 {
                    if let dcl::LoadProfVal::Value(_) = dl.load[lr - 1] {
                    } else {
                        dl.load[lr - 1] = dl.load[lr - 2].clone();
                        dl.data_quality.adj_fill += 1;
                    }
                }
            }
        }

        let (mut c1, mut c2) = (0, 0);
        for di in 1..p_year_load.loads.len() {
            let mut gd = 0;
            for hl in &mut p_year_load.loads[di].load {
                if let dcl::LoadProfVal::Value(_) = hl {
                    gd += 1;
                } else {
                    //*hl = dcl::LoadProfVal::Value(0.0); // temp1
                    c1 += 1;
                }
            }
            if gd == 0 {
                p_year_load.loads[di] = p_year_load.loads[di - 1].clone();
                p_year_load.loads[di].data_quality.adj_last = p_year_load.loads[di - 1].load.len();
            }
        }
        for hl in &mut p_year_load.loads[0].load {
            if let dcl::LoadProfVal::Value(_) = hl {
            } else {
                *hl = dcl::LoadProfVal::Value(0.0); // temp2
                c2 += 1;
            }
        }

        for di in 0..p_year_load.loads.len() {
            let dl = &p_year_load.loads[di];
            p_year_load.data_quality.adj_lead += dl.data_quality.adj_lead;
            p_year_load.data_quality.adj_one += dl.data_quality.adj_one;
            p_year_load.data_quality.adj_fill += dl.data_quality.adj_fill;
            p_year_load.data_quality.adj_last += dl.data_quality.adj_last;
        }
    }
}
