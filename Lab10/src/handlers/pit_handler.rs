use actix_web::{get, post, web, Responder, HttpResponse};
use log::{debug, info};
use serde_json::json;

use crate::models::pit::{ Pit, InputPit, ResPit};

#[get("/tax/pit")]
async fn get_pit() -> impl Responder {
    info!("get pit");
    debug!("get: ✅");

    let url = "https://private-f5d89-ittipolcha.apiary-mock.com/tax/pit";

    let response = reqwest::get(url)
        .await
        .unwrap()
        .json::<Pit>()
        .await
        .unwrap();

    HttpResponse::Ok().json(response)
}

#[post("/tax/pit")]
async fn post_pit(data: web::Json<InputPit>) -> impl Responder {
    info!("post pit");
    debug!("post: ✅");

    let _data = data.into_inner();
    
    let _ai = _data.assessable_income.forty_1+ _data.assessable_income.forty_2+ _data.assessable_income.forty_3+ _data.assessable_income.forty_4+ _data.assessable_income.forty_5+ _data.assessable_income.forty_6+ _data.assessable_income.forty_7+ _data.assessable_income.forty_8;
    let _ai1 = _data.assessable_income.forty_1;

    let mut _le = 0;
    let mut _le_1 = 0;
    let mut _le_2 = 0;
    let mut _le_3 = 0;
    let mut _le_4 = 0;
    let mut _le_5 = 0;
    let mut _le_6 = 0;
    let mut _le_7 = 0;
    let mut _le_8 = 0;

    let _it = ( _ai- _data.assessable_income.forty_1 )* 5/ 1000;
    
    if _data.assessable_income.forty_1+ _data.assessable_income.forty_2 >= 200000 {
        _le_1 = 100000;
    } 
    else {
        _le_1 = ( _data.assessable_income.forty_1+ _data.assessable_income.forty_2 )/ 2;
    }

    if _data.assessable_income.forty_3 >= 200000 {
        _le_3 = 100000;
    }
    else {
        _le_3 = ( _data.assessable_income.forty_3 )/ 2;
    }

    _le_5 = _data.assessable_income.forty_3* 30/ 100;
    _le_6 = _data.assessable_income.forty_3* 30/ 100;
    _le_7 = _data.assessable_income.forty_3* 60/ 100;
    _le_8 = _data.assessable_income.forty_3* 40/ 100;

    _le = _le_1 + _le_2 + _le_3 + _le_4 + _le_5 + _le_6 + _le_7 + _le_8;

    let _ni = _ai- _le;

    let mut _tr = 0;
    let mut _tr_1 = _data.deductible.personal;
    let mut _tr_2 = _data.deductible.child_per_person;
    let mut _tr_3 = _data.deductible.parent_per_person;
    let mut _tr_4 = _data.deductible.disabled;
    let mut _tr_5 = _data.deductible.pregnancy_childbirth;

    if _data.deductible.child_per_person >= 2 {
        _tr_2 = _data.deductible.child_per_person* 60000;
    }
    else {
        _tr_2 = 30000;
    }

    _tr = ( _tr_1* 60000 )+ _tr_2+ ( _tr_3* 30000 )+ ( _tr_4* 60000 )+ ( _tr_5* 60000 );

    let _dai = _ni- _tr;

    let mut _r = "";
    let mut _lpi = 0;
    if _dai >= 0 && _dai <= 150000 {
        _r = "0%";
        _lpi = 0;
    }
    else if _dai > 150000 && _dai <= 300000 {
        _r = "5%";
        _lpi = ( _dai- 150000 )* 5/ 100;
    }
    else if _dai > 300001 && _dai <= 500000 {
        _r = "10%";
        _lpi = ( ( _dai- 300000 )* 10/ 100 )+ 7500;
    }
    else if _dai > 500001 && _dai <= 750000 {
        _r = "15%";
        _lpi = ( ( _dai- 500000 )* 15/ 100 )+ 27500;
    }
    else if _dai > 750001 && _dai <= 1000000 {
        _r = "20%";
        _lpi = ( ( _dai- 750000 )* 20/ 100 )+ 65000;
    }
    else if _dai > 1000001 && _dai <= 2000000 {
        _r = "25%";
        _lpi = ( ( _dai- 1000000 )* 25/ 100 )+ 115000;
    }
    else if _dai > 2000001 && _dai <= 5000000 {
        _r = "30%";
        _lpi = ( ( _dai- 2000000 )* 30/ 100 )+ 365000;
    }
    else if _dai > 5000001 {
        _r = "35%";
        _lpi = ( ( _dai- 5000000 )* 35/ 100 )+ 1265000;
    }

    let mut _rc = "";
    if _it > _lpi {
        _rc = "individual_income_tax";
    }
    else {
        _rc = "ladder_personal_income_tax";
    }

    let _ra = "2015-08-05T08:40:51.620Z";

    let res_pit = ResPit {
        assessable_income: _ai,
        forty_1: _ai1,
        expenses: _le,
        net_income: _ni,
        tax_reduction: _tr,
        deductible_assessable_income: _dai,
        tax_rate: _r.to_string(),
        individual_income_tax: _it,
        ladder_personal_income_tax: _lpi,
        recommend_choosing: _rc.to_string(),
        response_at: _ra.to_string(),
     };
     
     let response_body = json!(res_pit);

     HttpResponse::Created().json(response_body)
}