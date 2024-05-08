use crate::sg::ldp::base;
use crate::sg::wk3;
use askama::Template;
use askama_axum;

#[derive(Template, Debug, Default, Clone)]
#[template(path = "pg2/ssfd1-1.html")]
pub struct LoadProfList {
    pub ssv: Vec<Substation>,
}

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
    pub good: i32,
    pub null: i32,
    pub none: i32,
    pub adj_lead: i32,
    pub adj_one: i32,
}

pub async fn handler() -> LoadProfList {
    let base = base();
    let mut web = LoadProfList::default();
    let lpl = base.wk3_subst.read().await;
    for (i, ss) in lpl.iter().enumerate() {
        let mut nss = Substation::default();
        nss.sbst = ss.sbst.clone();
        nss.prov = ss.prov.clone();
        nss.name = ss.name.clone();
        for fl in &ss.feed {
            let mut nfl = FeederLoad::default();
            nfl.feed = fl.feed.to_string();
            nfl.good = fl.good;
            nfl.null = fl.null;
            nfl.none = fl.none;
            nfl.adj_lead = fl.adj_lead;
            nfl.adj_one = fl.adj_one;
            nss.feed.push(Box::new(nfl));
        }
        web.ssv.push(nss);
    }
    web
}
