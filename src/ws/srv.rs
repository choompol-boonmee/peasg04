#![allow(unused)]
//use crate::sg::{pg1, pg2, pg3, pg4, pg5, pg6, pg7, root};
use crate::sg::{pg1, pg2, pg3, pg4, root};
use crate::web;
use crate::web::{city_calc01, city_calc02, feeder_power02, feeder_power03};
use crate::web::{feeder_day01, feeder_tranx01, feeder_year01};
use crate::web::{feeder_list01, feeder_list02, feeder_power01, feeder_yrpw01};
use crate::web::{ss_day01, ss_list01, ss_list02, ss_power01, ss_year01};
use axum::routing::get;
use axum::Router;
use tokio::sync::oneshot;

pub async fn http_serve() {
    let base = crate::sg::ldp::base();
    let app = Router::new()
        //.route("/", get(root::handler))
        .route("/", get(web::wk5e::handler))
        .route("/wk5a", get(web::wk5a::handler))
        .route("/wk5b", get(web::wk5b::handler))
        .route("/wk5c", get(web::wk5c::handler))
        .route("/wk5d", get(web::wk5d::handler))
        .route("/wk5e", get(web::wk5e::handler))
        .route("/wk5f", get(web::wk5f::handler))
        .route("/wk5g", get(web::wk5g::handler))
        .route("/wk5h", get(web::wk5h::handler))
        .route("/wk5i", get(web::wk5i::handler))
        .route("/wk5j", get(web::wk5j::handler))
        .route("/wk5k", get(web::wk5k::handler))
        .route("/wk5l", get(web::wk5l::handler))
        .route("/wk5m", get(web::wk5m::handler))
        .route("/wk5n", get(web::wk5n::handler))
        .route("/wk5o", get(web::wk5o::handler))
        .route("/wk5p", get(web::wk5p::handler))
        .route("/wk5q", get(web::wk5q::handler))
        .route("/wk5r", get(web::wk5r::handler))
        .route("/wk5s", get(web::wk5s::handler))
        .route("/wk5t", get(web::wk5t::handler))
        .route("/wk5t1", get(web::wk5t1::handler))
        .route("/wk5t2", get(web::wk5t2::handler))
        .route("/wk5t3", get(web::wk5t3::handler))
        .route("/wk5t4", get(web::wk5t4::handler))
        .route("/wk5t5", get(web::wk5t5::handler))
        .route("/wk5t6", get(web::wk5t6::handler))
        .route("/wk5t7", get(web::wk5t7::handler))
        .route("/wk5t8", get(web::wk5t8::handler))
        .route("/wk5t9", get(web::wk5t9::handler))
        .route("/wk5t10", get(web::wk5t10::handler))
        .route("/wk5u1/:prov", get(web::wk5u1::handler))
        .route("/city_calc01", get(city_calc01::handler))
        .route("/city_calc02", get(city_calc02::handler))
        .route("/feeder_yrpw01/:ssid/:fdid", get(feeder_yrpw01::handler))
        .route("/feeder_tranx01/:ssid/:fdid", get(feeder_tranx01::handler))
        .route("/feeder_power01/:ssid", get(feeder_power01::handler))
        .route("/feeder_power02/:ssid", get(feeder_power02::handler))
        .route("/feeder_power03", get(feeder_power03::handler))
        .route("/ss_power01", get(ss_power01::handler))
        .route("/ss_day01/:ssid/:day", get(ss_day01::handler))
        .route("/ss_year01/:ssid", get(ss_year01::handler))
        .route("/feeder_day01/:ssid/:fdid/:day", get(feeder_day01::handler))
        .route("/feeder_year01/:ssid/:fdid", get(feeder_year01::handler))
        .route("/feeder_list02/:ssid", get(feeder_list02::handler))
        .route("/feeder_list01/:ssid", get(feeder_list01::handler))
        .route("/ss_list01", get(ss_list01::handler))
        .route("/ss_list02", get(ss_list02::handler))
        //.route("/ssfd2/:fid", get(ssfd2::handler))
        //.route("/ssfd1", get(ssfd1::handler))
        //.route("/pg7/:png", get(pg7::handler))
        //.route("/pg6/:sgv", get(pg6::handler))
        //.route("/pg5/:xlsx", get(pg5::handler))
        //.route("/pg4", get(pg4::handler))
        //.route("/pg3", get(pg3::handler))
        //.route("/pg2", get(pg2::handler))
        //.route("/pg1", get(pg1::handler));
        ;
    let lisn = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(lisn, app).await.unwrap();
}
