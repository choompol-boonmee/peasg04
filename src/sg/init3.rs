use tokio::join;
use tokio::spawn;

pub async fn run() {
    let base = crate::sg::ldp::base();
    // read data
    join!(
        //spawn(async { crate::sg::ldp::load_lpyd().await }),
        //spawn(async { crate::sg::ldp::load_sspvmp().await }),
        //spawn(async { crate::sg::ldp::load_txmtmp().await }),
        spawn(async { crate::sg::wk4::load_wk4prc().await }),
        spawn(async { crate::sg::ldp::load_ssfdot().await }),
    );

    // process data
    join!(
        //        spawn(async { crate::sg::wk1::run().await }),
        //        spawn(async { crate::sg::wk2::run().await }),
        //        spawn(async { crate::sg::wk3::run().await }),
        //spawn(async { crate::sg::wk4::run().await }),
        //spawn(async { crate::sg::wk4::run3().await }),
        spawn(async { crate::sg::wk5::task().await }),
    );
    print!("===========================\n");
    //std::io::stdout().flush().expect("?");

    // serv web
    join!(spawn(async { crate::ws::srv::http_serve().await }));
}
