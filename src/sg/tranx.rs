use crate::sg::ldp::{FeederTranxInfo, TranxInfo};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub async fn run() {
    load_txmtmp().await;
}

pub async fn load_txmtmp() {
    if let Ok(file) = File::open("data/txmtmp.bin") {
        let rd = BufReader::new(file);
        if let Ok(txmtmp) =
            bincode::deserialize_from::<BufReader<File>, HashMap<String, TranxInfo>>(rd)
        {
            //print!("txmtmp: {}\n", txmtmp.len());
            for (k, tx) in txmtmp {
                print!("k: {} \n", tx.trans_feed);
            }
        } // read txmtmp.bin
    } // end open file
}
