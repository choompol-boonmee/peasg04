use super::dcl::{BaseData, Config};
use crate::sg::dcl;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::sync::RwLock;
use toml;

static BASE: OnceLock<BaseData> = OnceLock::new();

pub fn base() -> &'static BaseData {
    BASE.get_or_init(base_init)
}

fn base_init() -> BaseData {
    let fnm = "config.toml";
    let dt = match std::fs::read_to_string(fnm) {
        Ok(c) => c,
        _ => panic!("read config file"),
    };
    let cfg: Config = match toml::from_str(&dt) {
        Ok(d) => d,
        Err(e) => panic!("parse config {}", e),
    };
    //let acfg = Arc::new(RwLock::new(cfg));
    BaseData {
        config: Arc::new(RwLock::new(cfg)),
        ..Default::default()
    }
}

pub async fn load_lpyd() {
    let base = base();
    let mut task = Task::default();
    let mut lpyd = task.load_lpyd().await;

    {
        let mut sbvc_2023 = base.sbvc_2023.write().await;
        let mut sbmp_2023 = base.sbmp_2023.write().await;
        let (vc, mp) = lpyd.pop().unwrap();
        *sbvc_2023 = vc;
        *sbmp_2023 = mp;
    }
    {
        let mut sbvc_2022 = base.sbvc_2022.write().await;
        let mut sbmp_2022 = base.sbmp_2022.write().await;
        let (vc, mp) = lpyd.pop().unwrap();
        *sbvc_2022 = vc;
        *sbmp_2022 = mp;
    }
    {
        let mut sbvc_2021 = base.sbvc_2021.write().await;
        let mut sbmp_2021 = base.sbmp_2021.write().await;
        let (vc, mp) = lpyd.pop().unwrap();
        *sbvc_2021 = vc;
        *sbmp_2021 = mp;
    }
}

pub async fn load_sspvmp() {
    if let Ok(file) = File::open("data/sbpvmp.bin") {
        let rd = BufReader::new(file);
        if let Ok(sbpvmp) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, String>>(rd)
        {
            let ss_pv_mp = base().ss_pv_mp.clone();
            let mut ss_pv_mp = ss_pv_mp.write().await;
            *ss_pv_mp = sbpvmp;
        }
    }
}

pub async fn load_ssfdot() {
    // get outage data
    let mut ssfdot = HashMap::<String, HashMap<String, Vec<(String, String, String)>>>::new();
    if let Ok(file) = File::open("data/sbfdot.bin") {
        let rd = BufReader::new(file);
        if let Ok(ssfd) = bincode::deserialize_from::<
            BufReader<File>,
            HashMap<String, HashMap<String, Vec<(String, String, String)>>>,
        >(rd)
        {
            let ss_fd_ot = base().ss_fd_ot.clone();
            let mut ss_fd_ot = ss_fd_ot.write().await;
            *ss_fd_ot = ssfd;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Substation {
    sbst: String,
    feed: Vec<Box<FeederLoad>>,
}

#[derive(Debug, Clone, Default)]
pub struct FeederLoad {
    sbst: String,
    feed: String,
    fstday: i32,
    good: i32,
    null: i32,
    none: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeterInfo {
    pub meter_id: String,
    pub meter_phase: String,
    pub meter_office: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranxInfo {
    pub trans_id: String,
    pub trans_power: f64,
    pub trans_feed: String,
    pub trans_sub: String,
    pub trans_type: String,
    pub trans_own: String,
    pub meters: Vec<MeterInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FeederTranxInfo {
    pub fdtxmp: HashMap<String, Vec<FeederTranx>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FeederTranx {
    pub fd_id: String,
    pub tx_id: String,
    pub tx_power: f64,
    pub tx_own: String,
    pub mt_ph_a: usize,
    pub mt_ph_b: usize,
    pub mt_ph_c: usize,
    pub mt_1_ph: usize,
    pub mt_3_ph: usize,
    pub mt_else: usize,
}

pub async fn load_txmtmp() {
    let base = base();
    if let Ok(file) = File::open("data/txmtmp.bin") {
        let rd = BufReader::new(file);
        if let Ok(txmtmp) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, TranxInfo>>(rd)
        {
            //print!("txmtmp: {}\n", txmtmp.len());

            let mut fdtxinf = FeederTranxInfo {
                fdtxmp: HashMap::<String, Vec<FeederTranx>>::new(),
            };
            for (_k, tx) in txmtmp {
                if tx.trans_feed.len() < 5 {
                    continue;
                }
                let fd0 = &tx.trans_feed[3..5];
                let fdid = format!("{}_{}", tx.trans_sub, fd0);

                let tx_id = tx.trans_id.to_string();
                let tx_power = tx.trans_power;
                let tx_own = tx.trans_own;
                let (mut mt_ph_a, mut mt_ph_b, mut mt_ph_c, mut mt_1_ph, mut mt_3_ph, mut mt_else) =
                    (0, 0, 0, 0, 0, 0);
                for mt in &tx.meters {
                    if mt.meter_phase == "A" {
                        mt_ph_a += 1;
                        mt_1_ph += 1;
                    } else if mt.meter_phase == "B" {
                        mt_ph_b += 1;
                        mt_1_ph += 1;
                    } else if mt.meter_phase == "C" {
                        mt_ph_c += 1;
                        mt_1_ph += 1;
                    } else if mt.meter_phase == "ABC" {
                        mt_3_ph += 1;
                    } else {
                        mt_else += 1;
                    }
                }
                let fdtx = FeederTranx {
                    tx_id,
                    fd_id: tx.trans_feed.to_string(),
                    tx_power,
                    tx_own,
                    mt_ph_a,
                    mt_ph_b,
                    mt_ph_c,
                    mt_1_ph,
                    mt_3_ph,
                    mt_else,
                };
                if let Some(v) = fdtxinf.fdtxmp.get_mut(&fdid) {
                    v.push(fdtx);
                } else {
                    //print!("FD {} {} {}\n", tx.trans_sub, &tx.trans_feed, fd0);
                    fdtxinf.fdtxmp.insert(fdid, vec![fdtx]);
                }
            }
            {
                let mut fd_tx_info = base.fd_tx_info.write().await;
                *fd_tx_info = fdtxinf;
            }
        } // read txmtmp.bin
    } // end open file
}

#[derive(Debug, Clone, Default)]
pub struct Task {}

impl Task {
    pub async fn load_lpyd(
        &mut self,
    ) -> Vec<(Vec<String>, HashMap<String, Vec<Box<dcl::FeederLoad>>>)> {
        if let Ok(f) = File::open("data/lpyd.bin") {
            let r = BufReader::new(f);
            if let Ok(mut lpyd) = bincode::deserialize_from::<
                BufReader<File>,
                Vec<(Vec<String>, HashMap<String, Vec<Box<dcl::FeederLoad>>>)>,
            >(r)
            {
                lpyd
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }
}
