use crate::sg::ldp::base;
use crate::sg::wk2;
use askama::Template;
use askama_axum;

#[derive(Template, Debug, Default, Clone)]
#[template(path = "pg1/feed03.html")]
pub struct LoadProfList {
    pub ssv: Vec<wk2::Substation>,
}

pub async fn handler() -> LoadProfList {
    let base = base();
    let lpl = base.wk2_load_prof_list.read().await;
    LoadProfList {
        ssv: lpl.ssv.clone(),
    }
}
