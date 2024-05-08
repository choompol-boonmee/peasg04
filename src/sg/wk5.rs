pub async fn task() {
    task1().await;
}
pub async fn save_wk5prc() {}
pub async fn load_wk5prc() {}

use crate::sg::ldp::base;
use crate::sg::load::load_pvcamp;
use crate::sg::{dcl, ldp, wk4};
use crate::web;
use crate::web::{wk5, wk5a};
use askama::Template;
use askama_axum;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, OnceLock};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Wk5Proc {
    pub repo1: wk5::Repo1,
    pub wk5a: web::wk5a::Repo1,
    pub wk5b: web::wk5b::Repo1,
    pub wk5c: web::wk5c::Report,
    pub wk5d: web::wk5d::Report,
    pub wk5e: web::wk5e::Report,
    pub wk5f: web::wk5f::Report,
    pub wk5g: web::wk5g::Report,
    pub wk5h: web::wk5h::Report,
    pub wk5i: web::wk5i::Report,
    pub wk5j: web::wk5j::Report,
    pub wk5k: web::wk5k::Report,
    pub wk5l: web::wk5l::Report,
    pub wk5m: web::wk5m::Report,
    pub wk5n: web::wk5n::Report,
    pub wk5o: web::wk5o::Report,
    pub wk5p: web::wk5p::Report,
    pub wk5q: web::wk5q::Report,
    pub wk5r: web::wk5r::Report,
    pub wk5s: web::wk5s::Report,
    pub wk5t: web::wk5t::Report,
    pub wk5t1: web::wk5t1::Report,
    pub wk5t2: web::wk5t2::Report,
    pub wk5t3: web::wk5t3::Report,
    pub wk5t4: web::wk5t4::Report,
    pub wk5t5: web::wk5t5::Report,
    pub wk5t6: web::wk5t6::Report,
    pub wk5t7: web::wk5t7::Report,
    pub wk5t8: web::wk5t8::Report,
    pub wk5t9: web::wk5t9::Report,
    pub wk5t10: web::wk5t10::Report,
    pub wk5u1: web::wk5u1::Report,
    pub ssv: Vec<Substation>,
    pub tx: Tranx,
    pub year_load: wk4::YearLoad,
    pub last_year_load: wk4::YearLoad,
    pub prov_v: Vec<String>,
    pub ev_prov_dist: HashMap<String, EvDistCalc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Substation {
    pub ssid: String,
    pub prov: String,
    pub name: String,
    pub last_year: bool,
    pub feeders: Vec<Box<FeederLoad>>,
    pub year_load: wk4::YearLoad,
    pub last_year_load: wk4::YearLoad,
    pub tx: Tranx,
    pub ev: EvDistCalc,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FeederLoad {
    pub ssid: String,
    pub fdid: String,
    pub fdid5: String,
    pub prov: String,
    pub year_re: wk4::YearLoad,
    pub year_ev: wk4::YearLoad,
    pub outage_hour: f64,
    pub year_load: wk4::YearLoad,
    pub last_year_load: wk4::YearLoad,
    pub trans: Vec<ldp::FeederTranx>,
    pub tx: Tranx,
    pub para1: EvalPara1,
    pub ev: EvDistCalc,
    pub target_year_solar_energy: f32,
    pub target_solar_power: f32,
    pub target_solar_energy_storage: f32,
    pub solar_energy_series: Vec<f32>,
    pub solar_power_series: Vec<f32>,
    pub solar_day_energy_series: Vec<f32>,
    pub solar_storage_series: Vec<f32>,
    pub solar_storage_cost_series: Vec<f32>,
    pub solar_revenue_series: Vec<f32>,
    pub ev_car_series: Vec<f32>,
    pub ev_power_series: Vec<f32>,
    pub ev_energy_series: Vec<f32>,
    pub ev_revenue_series: Vec<f32>,
    pub ev_batt_required_series: Vec<f32>,
    pub ev_batt_cost_series: Vec<f32>,
    pub tx_to_all_ratio: f32,

    pub infra_invest_year: f32,
    pub smart_trx_cost: f32,
    pub smart_m1p_cost: f32,
    pub smart_m3p_cost: f32,
    pub comm_cost_year: f32,
    pub platform_cost: f32,
    pub implement_cost: f32,
    pub operation_cost: f32, // yearly
    pub meter_reading_cost: f32,
    pub outage_operation_cost: f32,
    pub loss_in_power_line_cost: f32,
    pub loss_in_phase_balance_cost: f32,

    pub infra_invest_year_series: Vec<f32>,
    pub smart_trx_cost_series: Vec<f32>,
    pub smart_m1p_cost_series: Vec<f32>,
    pub smart_m3p_cost_series: Vec<f32>,
    pub comm_cost_year_series: Vec<f32>,
    pub platform_cost_series: Vec<f32>,
    pub implement_cost_series: Vec<f32>,
    pub operation_cost_series: Vec<f32>, // yearly
    pub meter_reading_cost_series: Vec<f32>,
    pub outage_operation_cost_series: Vec<f32>,
    pub loss_in_power_line_cost_series: Vec<f32>,
    pub loss_in_phase_balance_cost_series: Vec<f32>,

    pub financial_benefit: f32,
    pub economic_benefit: f32,
    pub total_cost: f32,

    pub financial_benefit_series: Vec<f32>,
    pub economic_benefit_series: Vec<f32>,
    pub total_cost_series: Vec<f32>,

    pub financial_benefit_npv: f32,
    pub economic_benefit_npv: f32,
    pub total_cost_npv: f32,

    pub financial_benefit_npv_series: Vec<f32>,
    pub economic_benefit_npv_series: Vec<f32>,
    pub total_cost_npv_series: Vec<f32>,

    pub firr: f32,
    pub eirr: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tranx {
    pub tx_no: usize,
    pub tx_pea: usize,
    pub tx_cus: usize,
    pub mt1_no: usize,
    pub mt3_no: usize,
    pub tx_pt: f32,
    pub mt1_pt: f32,
    pub mt3_pt: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EvDistCalc {
    pub id: String,
    pub ev_no: f32,
    pub ev_pc: f32,
    pub ev_ds: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EvalPara1 {
    pub energy: f32,
}

use regex::Regex;

async fn task1() {
    let mut wk5prc = Wk5Proc::default();
    let wk4prc = base().wk4_ssv.clone();
    {
        let mut prov_set = HashSet::new();
        let mut wk4prc = wk4prc.write().await;
        let mut txno = 0;
        let re = Regex::new(r"..._[0-9][0-9].+").unwrap();
        let (mut sum1, mut sum2) = (0.0, 0.0);
        let cfg = base().config.clone();
        let cfg = cfg.read().await;
        let stw = cfg.criteria.solar_time_window;
        for ss in &mut wk4prc.ssv {
            sum1 += ss.year_load.power_quality.pos_energy;
            sum2 += ss.last_year_load.power_quality.pos_energy;

            let mut ss2 = Substation::default();
            ss2.ssid = ss.sbst.to_string();
            ss2.prov = ss.prov.to_string();
            ss2.name = ss.name.to_string();
            ss2.last_year = ss.last_year;
            ss2.year_load = ss.year_load.clone();
            prov_set.insert(ss2.prov.to_string());

            let mut fdmp = HashMap::<String, FeederLoad>::new();
            for fd in &ss.feeders {
                if re.is_match(&fd.feed) == false {
                    continue;
                }
                //let fdid5 = fd.feed[0..6].to_string();
                let fdid5 = fd.feed[0..7].to_string();
                if let Some(fd2) = fdmp.get_mut(&fdid5) {
                    let (mut c1, mut c2) = (0, 0);
                    for di in 0..fd.year_load.loads.len() {
                        for hi in 0..fd.year_load.loads[di].load.len() {
                            let mut dd = dcl::LoadProfVal::Value(0.0);
                            if let dcl::LoadProfVal::Value(v) = &fd.year_load.loads[di].load[hi] {
                                dd = dcl::LoadProfVal::Value(*v);
                            } else {
                                c1 += 1;
                            }
                            if let (dcl::LoadProfVal::Value(d2), dcl::LoadProfVal::Value(d1)) = (
                                &fd2.year_load.loads[di].load[hi],
                                &fd2.year_load.loads[di].load[hi],
                            ) {
                                dd = dcl::LoadProfVal::Value(d1 + d2);
                            } else {
                                c2 += 1;
                            }
                            fd2.year_load.loads[di].load[hi] = dd;
                        }
                    }
                } else {
                    let mut fd2 = FeederLoad::default();
                    fd2.ssid = ss.sbst.to_string();
                    fd2.fdid = fd.feed.to_string();
                    //fd2.fdid5 = format!("{}{}", ss.sbst, &fd.feed[4..6]);
                    //fd2.fdid5 = format!("{}{}", ss.sbst, &fd.feed[4..7]);
                    fd2.fdid5 = fdid5.to_string();
                    fd2.prov = ss.prov.to_string();
                    fd2.trans = fd.trans.clone();
                    for tx in &fd.trans {
                        fd2.tx.tx_no += 1;
                        txno += 1;
                        if tx.tx_own == "P" {
                            fd2.tx.tx_pea += 1;
                        } else {
                            fd2.tx.tx_cus += 1;
                        }
                        fd2.tx.mt1_no += tx.mt_1_ph;
                        fd2.tx.mt3_no += tx.mt_3_ph;
                    }
                    fd2.year_load = fd.year_load.clone();
                    for di in 0..fd.year_load.loads.len() {
                        for hi in 0..fd.year_load.loads[di].load.len() {
                            match fd.year_load.loads[di].load[hi] {
                                dcl::LoadProfVal::Value(y) => {}
                                _ => {
                                    fd2.year_load.loads[di].load[hi] = dcl::LoadProfVal::Value(0.0)
                                }
                            }
                        }
                    }
                    fdmp.insert(fdid5, fd2);
                };
            }
            let mut fds: Vec<Box<FeederLoad>> =
                fdmp.into_iter().map(|(k, v)| Box::new(v)).collect();
            fds.sort_by(|a, b| a.fdid5.partial_cmp(&b.fdid5).unwrap());
            ss2.feeders = fds;
            wk5prc.ssv.push(ss2);
        }
        print!("sumx {} {}\n", sum1, sum2);
        wk5prc.prov_v = Vec::from_iter(prov_set);
        wk5prc.prov_v.sort();
    }
    print!("power calc\n");
    power(&mut wk5prc.ssv).await;
    gen_ev(&mut wk5prc).await;
    add_outage(&mut wk5prc).await;
    eval_para1(&mut wk5prc, base().config.clone()).await;
    print!("solr call\n");
    solar_calc(&mut wk5prc, base().config.clone()).await;
    print!("ev call\n");
    ev_calc(&mut wk5prc, base().config.clone()).await;
    infra_calc(&mut wk5prc, base().config.clone()).await;
    return_calc(&mut wk5prc, base().config.clone()).await;

    wk5::make_repo1(&mut wk5prc, base().config.clone()).await;
    web::wk5a::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5b::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5c::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5d::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5e::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5f::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5g::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5h::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5i::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5j::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5k::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5l::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5m::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5n::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5o::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5p::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5q::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5r::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5s::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5t::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5t1::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5t2::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5t3::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5t4::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5t5::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5t6::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5t7::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5t8::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5t9::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5t10::make_repo(&mut wk5prc, base().config.clone()).await;
    web::wk5u1::make_repo(&mut wk5prc, base().config.clone()).await;
    {
        let a_wk5prc = base().wk5prc.clone();
        let mut a_wk5prc = a_wk5prc.write().await;
        *a_wk5prc = wk5prc;
    }
}

pub async fn power(ssv: &mut Vec<Substation>) {
    let cfg = base().config.clone();
    let cfg = cfg.read().await;
    let stw = cfg.criteria.solar_time_window;
    for ss in ssv {
        for fd in &mut ss.feeders {
            fd.year_load.power(stw).await;
        }
        ss.year_load.power(stw).await;
    }
}

async fn solar_calc(wk5prc: &mut Wk5Proc, acfg: Arc<RwLock<dcl::Config>>) {
    let cfg = acfg.read().await;
    let sol = cfg.criteria.solar_energy_ratio;
    let max = cfg.criteria.bess_energy_max;
    let sot = cfg.criteria.solar_time_window;
    let syf = cfg.criteria.start_year_from_2022;
    let imy = cfg.criteria.implement_year;
    let opy = cfg.criteria.operate_year;
    let se0 = cfg.criteria.vspp_energy_ratio_start; // = 0.05
    let se1 = cfg.criteria.vspp_energy_ratio_end; // = 0.10
    let egr = cfg.criteria.energy_growth_rate; // = 0.05
    let sup = cfg.criteria.energy_sale_price; // 4000 B/mwh
    let bmx = cfg.criteria.bess_energy_max as f32; // 40 mwh
    let bup = cfg.criteria.bess_sell_per_mwh as f32; // 2500 B/mwh
    let yrl = syf + imy + opy;
    let yrl = yrl as usize;
    let yr0 = syf as usize + imy as usize;
    let yri = syf as usize;
    for si in 0..wk5prc.ssv.len() {
        for fi in 0..wk5prc.ssv[si].feeders.len() {
            let mut fd = &mut wk5prc.ssv[si].feeders[fi];
            fd.target_year_solar_energy = fd.year_load.power_quality.pos_energy * se0 as f32;
            fd.target_solar_power = fd.target_year_solar_energy / (365.0 * sot);
            fd.target_solar_energy_storage = fd.target_solar_power * sot;
            /*
            print!(
                "fd:{} e:{} yr:{}",
                fd.fdid, fd.target_year_solar_energy, yrl
            );
            */
            let mut soe = fd.year_load.power_quality.pos_energy * se0;
            let yst = (se1 - se0) / (yrl as f32);
            for i in 0..yrl {
                let yrt = 1.0 + yst * (i as f32 + 1.0);
                let soe = soe * yrt;
                //print!(" {}-{}-{}", yst, yrt, soe);
                fd.solar_energy_series.push(soe);
                let sop = soe / (365.0 * sot);
                fd.solar_power_series.push(sop);
                let dye = sop * sot;
                fd.solar_day_energy_series.push(dye);
                let ese = if dye > bmx { bmx } else { dye };
                let ste = ese;
                let ese = if i >= yr0 { ese } else { 0.0 };
                fd.solar_storage_series.push(ese);
                fd.solar_revenue_series.push(ese * sup * 365.0);
                let mut bcs = 0.0;
                if i >= yri && i < yr0 {
                    bcs = ste * bup / imy;
                }
                fd.solar_storage_cost_series.push(bcs);
            }
            //print!("\n");
        }
    }
}

async fn infra_calc(wk5prc: &mut Wk5Proc, acfg: Arc<RwLock<dcl::Config>>) {
    let cfg = acfg.read().await;
    let ifv = cfg.criteria.infra_invest_per_year;
    let syf = cfg.criteria.start_year_from_2022;
    let imy = cfg.criteria.implement_year;
    let opy = cfg.criteria.operate_year;
    let ims = syf as usize;
    let ops = syf as usize + imy as usize;
    let ope = ops + opy as usize;
    let mut txno = 0.0;
    for si in 0..wk5prc.ssv.len() {
        for fi in 0..wk5prc.ssv[si].feeders.len() {
            let mut fd = &mut wk5prc.ssv[si].feeders[fi];
            txno += fd.tx.tx_no as f32;
        }
    }
    for si in 0..wk5prc.ssv.len() {
        for fi in 0..wk5prc.ssv[si].feeders.len() {
            let mut fd = &mut wk5prc.ssv[si].feeders[fi];
            fd.tx_to_all_ratio = fd.tx.tx_no as f32 / txno;

            fd.infra_invest_year = fd.tx_to_all_ratio * txno * 1000000.0;
            fd.smart_trx_cost = cfg.criteria.smart_trx_unit_cost * fd.tx.tx_no as f32;
            fd.smart_m1p_cost = cfg.criteria.smart_m1p_unit_cost * fd.tx.mt1_no as f32;
            fd.smart_m3p_cost = cfg.criteria.smart_m3p_unit_cost * fd.tx.mt3_no as f32;
            let dev = fd.tx.tx_no as f32 + fd.tx.mt3_no as f32 + fd.tx.mt1_no as f32;
            fd.comm_cost_year =
                dev * cfg.criteria.comm_per_devic_per_month * cfg.criteria.operate_year;
            fd.platform_cost = dev * cfg.criteria.platform_cost_per_device;
            fd.implement_cost = dev * cfg.criteria.implement_cost_per_device;
            fd.operation_cost = dev * cfg.criteria.operation_cost_per_year_device;
            fd.meter_reading_cost =
                fd.tx.mt1_no as f32 * cfg.criteria.meter_reading_cost_cut * 12.0;
            fd.outage_operation_cost =
                fd.outage_hour as f32 * cfg.criteria.outage_operation_cost_per_hour;
            fd.loss_in_power_line_cost = fd.year_load.power_quality.pos_energy
                * cfg.criteria.loss_in_power_line_rate
                * cfg.criteria.energy_sale_price;
            fd.loss_in_phase_balance_cost = fd.year_load.power_quality.pos_energy
                * cfg.criteria.loss_in_phase_balance_rate
                * cfg.criteria.energy_sale_price;

            for i in 0..ope {
                let mut icst = 0.0;

                // transformer
                let mut cst = fd.smart_trx_cost / 3.0;
                if i < ims || i >= ops {
                    cst = 0.0;
                }
                fd.smart_trx_cost_series.push(cst);
                icst += cst;

                // meter 1 phase
                let mut cst = fd.smart_m1p_cost / 3.0;
                if i < ims || i >= ops {
                    cst = 0.0;
                }
                fd.smart_m1p_cost_series.push(cst);
                icst += cst;

                // meter 3 phase
                let mut cst = fd.smart_m3p_cost / 3.0;
                if i < ims || i >= ops {
                    cst = 0.0;
                }
                fd.smart_m3p_cost_series.push(cst);
                icst += cst;

                // communication_cost
                let mut cst = fd.comm_cost_year;
                if i < ops {
                    cst = 0.0;
                }
                fd.comm_cost_year_series.push(cst);
                icst += cst;

                // platform cost
                let mut cst = fd.platform_cost / 3.0;
                if i < ims || i >= ops {
                    cst = 0.0;
                }
                fd.platform_cost_series.push(cst);
                icst += cst;

                // implement cost
                let mut cst = fd.implement_cost / 3.0;
                if i < ims || i >= ops {
                    cst = 0.0;
                }
                fd.implement_cost_series.push(cst);
                icst += cst;

                // operation_cost
                let mut cst = fd.operation_cost;
                if i < ops {
                    cst = 0.0;
                }
                fd.operation_cost_series.push(cst);
                icst += cst;

                // meter_reading_cost
                let mut cst = fd.meter_reading_cost;
                if i < ops {
                    cst = 0.0;
                }
                fd.meter_reading_cost_series.push(cst);

                // outage_operation_cost
                let mut cst = fd.outage_operation_cost;
                if i < ops {
                    cst = 0.0;
                }
                fd.outage_operation_cost_series.push(cst);

                // loss_in_power_line_cost
                let mut cst = fd.loss_in_power_line_cost;
                if i < ops {
                    cst = 0.0;
                }
                fd.loss_in_power_line_cost_series.push(cst);

                // loss_in_phase_balance_cost
                let mut cst = fd.loss_in_phase_balance_cost;
                if i < ops {
                    cst = 0.0;
                }
                fd.loss_in_phase_balance_cost_series.push(cst);

                // infra invest
                let mut infra = fd.infra_invest_year;
                let mut infra = 50000000.0;
                if i < ops {
                    infra = 0.0;
                }
                fd.infra_invest_year_series.push(infra);
            }
        }
    }
}

async fn return_calc(wk5prc: &mut Wk5Proc, acfg: Arc<RwLock<dcl::Config>>) {
    let cfg = acfg.read().await;
    let syf = cfg.criteria.start_year_from_2022;
    let imy = cfg.criteria.implement_year;
    let opy = cfg.criteria.operate_year;
    let ims = syf as usize;
    let ops = syf as usize + imy as usize;
    let ope = ops + opy as usize;
    for si in 0..wk5prc.ssv.len() {
        for fi in 0..wk5prc.ssv[si].feeders.len() {
            let mut fd = &mut wk5prc.ssv[si].feeders[fi];
            let (mut fibe_a, mut ecbe_a, mut toco_a) = (0.0, 0.0, 0.0);
            let (mut fibe0_a, mut ecbe0_a, mut toco0_a) = (0.0, 0.0, 0.0);
            for i in 0..ope {
                let mut fibe = 0.0;
                let mut ecbe = 0.0;
                let mut toco = 0.0;

                fibe += fd.solar_revenue_series[i];
                toco += fd.solar_storage_cost_series[i];
                fibe += fd.ev_revenue_series[i];
                toco += fd.ev_batt_cost_series[i];
                ecbe += fd.infra_invest_year_series[i];
                toco += fd.smart_trx_cost_series[i];
                toco += fd.smart_m1p_cost_series[i];
                toco += fd.smart_m3p_cost_series[i];
                toco += fd.comm_cost_year_series[i];
                toco += fd.platform_cost_series[i];
                toco += fd.implement_cost_series[i];
                toco += fd.operation_cost_series[i];
                fibe += fd.meter_reading_cost_series[i];
                ecbe += fd.outage_operation_cost_series[i];
                ecbe += fd.loss_in_power_line_cost_series[i];
                ecbe += fd.loss_in_phase_balance_cost_series[i];

                fd.financial_benefit_series.push(fibe);
                fd.economic_benefit_series.push(ecbe);
                fd.total_cost_series.push(toco);

                let mut fibe0 = 0.0;
                let mut ecbe0 = 0.0;
                let mut toco0 = 0.0;
                if i >= ims {
                    let t = i - ims + 1;
                    let t = t as f32;
                    let deno = 1.0 + cfg.criteria.economi_discount_rate;
                    let deno = deno.powf(t);
                    //let deno = 1.0;
                    fibe0 = fibe / deno;
                    ecbe0 = ecbe / deno;
                    toco0 = toco / deno;
                }
                fd.financial_benefit_npv_series.push(fibe0);
                fd.economic_benefit_npv_series.push(ecbe0);
                fd.total_cost_npv_series.push(toco0);

                fibe_a += fibe;
                ecbe_a += ecbe;
                toco_a += toco;
                fibe0_a += fibe0;
                ecbe0_a += ecbe0;
                toco0_a += toco0;
            }
            fd.financial_benefit = fibe_a;
            fd.economic_benefit = ecbe_a;
            fd.total_cost = toco_a;

            fd.financial_benefit_npv = fibe0_a;
            fd.economic_benefit_npv = ecbe0_a;
            fd.total_cost_npv = toco0_a;

            fd.firr = (fibe0_a - toco0_a) / toco0_a;
            fd.eirr = (ecbe0_a - toco0_a) / toco0_a;
        }
    }
}

async fn ev_calc(wk5prc: &mut Wk5Proc, acfg: Arc<RwLock<dcl::Config>>) {
    let cfg = acfg.read().await;
    let syf = cfg.criteria.start_year_from_2022;
    let imy = cfg.criteria.implement_year;
    let opy = cfg.criteria.operate_year;
    let egr = cfg.criteria.energy_growth_rate; // = 0.05
    let ev0 = cfg.criteria.ev_growth_rate_start; // = 0.05
    let ev1 = cfg.criteria.ev_growth_rate_end; // = 0.08
    let ev4 = cfg.criteria.ev_car_all_reg / cfg.criteria.ev_car_reg_cnt;
    let epw = cfg.criteria.evcharger_type1_pw as f32;
    let eup = cfg.criteria.ev_energy_unit_price as f32; // 2500 Bht/MWh
    let buc = cfg.criteria.bess_sell_per_mw as f32; // 25941600.0 B/MW
    let bmx = cfg.criteria.bess_power_max;

    let yrl = syf + imy + opy;
    let yrl = yrl as usize;
    let evi = (ev1 - ev0) / (yrl as f32);
    let yr0 = syf as usize + imy as usize;
    let yri = syf as usize;
    for si in 0..wk5prc.ssv.len() {
        for fi in 0..wk5prc.ssv[si].feeders.len() {
            let mut fd = &mut wk5prc.ssv[si].feeders[fi];
            let mut evn = fd.ev.ev_ds * ev4;
            let evn0 = evn;
            let mut evds = fd.ev.ev_ds; // saled ev each year
                                        //print!("F:{} {}:", fd.fdid, evds);
            let mut evr = ev0;
            for i in 0..yrl {
                evr = evr + evi;
                evds = evds * (1.0 + evr);
                evn += evds;
                //print!(" {}-{}", evds, evn);
                fd.ev_car_series.push(evn);
                let evp = evn * epw;
                let eve = evp * 365.0 * cfg.criteria.ev_real_charge;
                let evr = eve * eup;
                fd.ev_power_series.push(evp);
                fd.ev_energy_series.push(eve);
                fd.ev_revenue_series.push(evr);
                let mut ebp = evn0 * epw;
                if ebp > bmx {
                    ebp = bmx;
                }
                let mut bcs = ebp * buc / imy;
                if i < yri || i >= yr0 {
                    bcs = 0.0;
                }
                if i < yr0 {
                    ebp = 0.0;
                }
                fd.ev_batt_required_series.push(ebp);
                fd.ev_batt_cost_series.push(bcs);
                //pub ev_batt_required_series: Vec<f32>,
                //pub ev_batt_cost_series: Vec<f32>,
                //ev_power_series
                //ev_energy_series
                //ev_revenue_series
            }
            //print!("\n");
        }
    }
    print!("ev calc\n");
}

async fn eval_para1(wk5prc: &mut Wk5Proc, acfg: Arc<RwLock<dcl::Config>>) {
    let cfg = acfg.read().await;
    let sat = cfg.residence.sat_energy;
    for si in 0..wk5prc.ssv.len() {
        for fi in 0..wk5prc.ssv[si].feeders.len() {
            let mut en = wk5prc.ssv[si].feeders[fi]
                .year_load
                .power_quality
                .pos_energy;
            if en > sat {
                en = sat;
            }
            wk5prc.ssv[si].feeders[fi].para1.energy = en;
        }
    }
}

async fn add_outage(wk5prc: &mut Wk5Proc) {
    let ss_fd_ot = base().ss_fd_ot.clone();
    let ss_fd_ot = ss_fd_ot.read().await;
    let tp0 = "ไฟฟ้าขัดข้อง";
    let fm0 = "%d-%m-%Y %H:%M:%S";
    for ss in &mut wk5prc.ssv {
        for fd in &mut ss.feeders {
            if let Some(ssot) = ss_fd_ot.get(&ss.ssid) {
                if let Some(fdot) = ssot.get(&fd.fdid5) {
                    let mut ot00 = 0;
                    for (st, ed, tp) in fdot {
                        if tp == tp0 {
                            let dttm1 = NaiveDateTime::parse_from_str(st.as_str(), fm0).unwrap();
                            let dttm2 = NaiveDateTime::parse_from_str(ed.as_str(), fm0).unwrap();
                            ot00 += dttm2.timestamp_millis() - dttm1.timestamp_millis();
                        }
                    }
                    let otd = ot00 as f64;
                    fd.outage_hour = otd / (1000f64 * 3600f64);
                }
            }
        }
    }
}

async fn car_reg_2023(wk5prc: &mut Wk5Proc) {
    let cfg = base().config.clone();

    let cfg = cfg.read().await;
    let bkks = cfg.criteria.car_reg_bkk_province.split(",");
    let bkks = bkks.collect::<Vec<&str>>();
    let bkkn = cfg.criteria.car_reg_bkk_percent.split(",");
    let bkkn = bkkn.collect::<Vec<&str>>();
    let bkkn = bkkn
        .iter()
        .map(|a| a.parse().unwrap())
        .collect::<Vec<f64>>();
    let asss = cfg.criteria.car_reg_to_province.split(",");
    let asss = asss.collect::<Vec<&str>>();
    let assn = cfg.criteria.car_reg_to_percent.split(",");
    let assn = assn.collect::<Vec<&str>>();
    let assn = assn
        .iter()
        .map(|a| a.parse().unwrap())
        .collect::<Vec<f64>>();
    if bkkn.len() != bkks.len() {
        panic!("car_reg_bkk_province is NG\n");
    }
    if assn.len() != asss.len() {
        panic!("car_reg_to_province is NG\n");
    }
    let mut pv_ca_mp = load_pvcamp();
    let mut pv_ca_mp2 = HashMap::new();
    let mut pv_ca_cn2 = HashMap::new();
    let mut cnt0 = 0.0;
    pv_ca_mp.insert("กรุงเทพมหานคร".to_string(), 967297.0);
    for (k, v) in &pv_ca_mp {
        cnt0 += *v;
        let mut kk = k.to_string();
        let mut vv = *v;
        if k == "ยะลา" {
            if let Some(v2) = pv_ca_mp.get("สาขา อ.เบตง") {
                //let v1 = *v2;
                vv += *v2;
            }
        }
        if kk == " พระนครศรีอยุธยา" {
            kk = "พระนครศรีอยุธยา".to_string();
        }
        if kk == "แม่ฮองสอน" {
            kk = "แม่ฮ่องสอน".to_string();
        }
        if kk == "สาขา อ.เบตง" {
            //print!("NO BETONG\n");
        } else {
            //print!("'{}' - {}\n", kk, vv);
            pv_ca_mp2.insert(kk.clone(), vv);
            pv_ca_cn2.insert(kk, 0);
        }
    }
    //let pv_mp = HashMap::new();
    for ss in &wk5prc.ssv {
        let pv = &ss.prov;
        if let Some(_) = pv_ca_mp2.get(pv) {
            if let Some(cn) = pv_ca_cn2.get_mut(pv) {
                *cn += 1;
            }
        } else {
            //print!("OUT {}\n", pv);
        }
    }
    //print!("all car {}\n", cnt0);
    let mut tk0 = 0.0;
    for (i, b) in bkks.iter().enumerate() {
        if let Some(nn) = pv_ca_mp2.get_mut(&b.to_string()) {
            let tk = *nn * bkkn[i] / 100.0;
            *nn -= tk;
            tk0 += tk;
            //print!("take from {} {}\n", b, tk);
        }
    }
    for (i, t) in asss.iter().enumerate() {
        let ts = t.to_string();
        if let Some(cn) = pv_ca_mp2.get_mut(&t.to_string()) {
            let ad = tk0 * assn[i] / 100.0;
            *cn += ad;
            //print!("add to {} {}\n", t, ad);
        }
    }
    let mut pv_car_reg_mp = HashMap::new();
    let mut total = 0.0f32;
    for (k, v) in &pv_ca_mp2 {
        if bkks.contains(&k.as_str()) {
            continue;
        }
        let mut pv_ca_reg = EvDistCalc::default();
        pv_ca_reg.id = k.to_string();
        pv_ca_reg.ev_no = *v as f32;
        total += pv_ca_reg.ev_no;
        pv_car_reg_mp.insert(k.to_string(), pv_ca_reg);
    }
    let ev_reg_no = cfg.criteria.ev_car_reg_cnt;
    for (k, v) in &mut pv_car_reg_mp {
        if total > 0.0 {
            v.ev_pc = v.ev_no / total as f32;
            v.ev_ds = v.ev_pc * ev_reg_no;
        } else {
            v.ev_ds = 0.0;
        }
    }
    wk5prc.ev_prov_dist = pv_car_reg_mp.clone();
}

async fn calc_tx_dist(wk5prc: &mut Wk5Proc) {
    for ss in &mut wk5prc.ssv {
        for fd in &ss.feeders {
            ss.tx.tx_no += fd.tx.tx_no;
            ss.tx.tx_pea += fd.tx.tx_pea;
            ss.tx.tx_cus += fd.tx.tx_cus;
            ss.tx.mt1_no += fd.tx.mt1_no;
            ss.tx.mt3_no += fd.tx.mt3_no;
        }
        for fd in &mut ss.feeders {
            fd.tx.tx_pt = fd.tx.tx_no as f32 / ss.tx.tx_no as f32;
            fd.tx.mt1_pt = fd.tx.mt1_no as f32 / ss.tx.mt1_no as f32;
            fd.tx.mt3_pt = fd.tx.mt3_no as f32 / ss.tx.mt3_no as f32;
        }
        wk5prc.tx.tx_no += ss.tx.tx_no;
        wk5prc.tx.tx_no += ss.tx.tx_no;
        wk5prc.tx.tx_no += ss.tx.tx_no;
        wk5prc.tx.mt1_no += ss.tx.mt1_no;
        wk5prc.tx.mt3_no += ss.tx.mt3_no;
    }
    for ss in &mut wk5prc.ssv {
        wk5prc.tx.tx_pt = ss.tx.tx_no as f32 / wk5prc.tx.tx_no as f32;
        wk5prc.tx.mt1_pt = ss.tx.mt1_no as f32 / wk5prc.tx.mt1_no as f32;
        wk5prc.tx.mt3_pt = ss.tx.mt3_no as f32 / wk5prc.tx.mt3_no as f32;
    }
}

fn ev_use_factor(txno: usize, m3no: usize, m1no: usize) -> f32 {
    let a = txno as f32 * 1.0 + m3no as f32 * 2.0 + m1no as f32;
    if a.is_nan() {
        return 0.0;
    }
    a
}

async fn calc_ev_dist(wk5prc: &mut Wk5Proc) {
    let cfg = base().config.clone();
    let cfg = cfg.read().await;
    let ev_reg_no = cfg.criteria.ev_car_reg_cnt;
    let mut pv_si_mp = HashMap::<String, Vec<usize>>::new();
    for (si, ss) in wk5prc.ssv.iter().enumerate() {
        if let Some(pvl) = pv_si_mp.get_mut(&ss.prov) {
            pvl.push(si);
        } else {
            pv_si_mp.insert(ss.prov.to_string(), vec![si]);
        }
    }
    let ev_prov_dist = &wk5prc.ev_prov_dist;
    let mut allev = 0.0;
    let mut txno0 = 0;
    for (pv, ev) in ev_prov_dist {
        if let Some(pvl) = pv_si_mp.get(pv) {
            let (mut txno, mut m1no, mut m3no) = (0, 0, 0);
            let mut ss_cus_ev = 0.0;
            for si in pvl {
                let ss = &wk5prc.ssv[*si];
                txno += ss.tx.tx_no;
                m1no += ss.tx.mt1_no;
                m3no += ss.tx.mt3_no;
                ss_cus_ev += ev_use_factor(ss.tx.tx_no, ss.tx.mt3_no, ss.tx.mt1_no);
            }
            if ss_cus_ev.is_nan() {
                print!("NAN {}\n", ss_cus_ev);
            }
            txno0 += txno;
            //let ss_cus_ev = txno * 4 + m3no * 2 + m1no;
            // all ev in substation
            //let ss_cus_ev = ev_use_factor(txno, m3no, m1no);
            let mut tt = 0.0;
            let mut ssds = 0.0;
            for si in pvl {
                let ss = &mut wk5prc.ssv[*si];
                ss.ev.id = (*ss.ssid).to_string();
                ss.ev.ev_pc = ev_use_factor(ss.tx.tx_no, ss.tx.mt3_no, ss.tx.mt1_no) / ss_cus_ev;
                ss.ev.ev_ds = ev.ev_ds * ss.ev.ev_pc;
                /*
                if ss.ev.ev_ds.is_nan() {
                    ss.ev.ev_ds = 0.0;
                }
                print!(
                    "  {} pc:{} ds:{} de:{}\n",
                    si, ss.ev.ev_pc, ss.ev.ev_ds, ss_cus_ev
                );
                */
                ssds += ss.ev.ev_ds;
                tt += ss.ev.ev_pc;
                ss.ev.ev_no = 0.0;
                for fi in 0..ss.feeders.len() {
                    let txno = ss.feeders[fi].tx.tx_no;
                    let m1no = ss.feeders[fi].tx.mt1_no;
                    let m3no = ss.feeders[fi].tx.mt3_no;
                    let ev_no = ev_use_factor(txno, m3no, m1no);
                    ss.ev.ev_no += ev_no;
                    ss.feeders[fi].ev.ev_no = ev_no;
                }
                for fi in 0..ss.feeders.len() {
                    ss.feeders[fi].ev.ev_pc = ss.feeders[fi].ev.ev_no / ss.ev.ev_no;
                    ss.feeders[fi].ev.ev_ds = ss.ev.ev_ds * ss.feeders[fi].ev.ev_pc;
                    if ss.feeders[fi].ev.ev_ds.is_nan() {
                        // 1
                        ss.feeders[fi].ev.ev_ds = 0.0;
                    }
                }
            }
            /*
            print!(
                "pr:{} tx:{} m3:{} m1:{} ev:{} ev:{}\n",
                pv, txno, m3no, m1no, ev.ev_ds, ssds
            );
            */
            allev += ev.ev_ds;
            /*
            print!(
                "pv: {} - ss len:{} ev:{:.2} regpc:{:.3}\n",
                pv,
                pvl.len(),
                ev.ev_ds,
                ev.ev_pc
            );
            */
        } else {
            print!("NG prov {}\n", pv);
        } // end of ss calculation
    } // end of province
}

async fn gen_ev(wk5prc: &mut Wk5Proc) {
    calc_tx_dist(wk5prc).await;
    car_reg_2023(wk5prc).await;
    calc_ev_dist(wk5prc).await;
}

