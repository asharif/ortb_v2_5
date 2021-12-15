use crate::dto::bid_response::BidResponse;
use crate::sample_ortb_res::*;
use serde_json;

#[test]
fn res_instream_pre_roll() {
    let bid_price = 9.051027;
    let jd = &mut serde_json::Deserializer::from_str(&INSTREAM_PRE_ROLL);
    let ortb_res_result: Result<BidResponse, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_res_result {
        Ok(res) => match &res.seatbid {
            Some(sb) => {
                assert_eq!(sb[0].bid[0].price, bid_price);
            }
            None => {
                assert_eq!(0.0, bid_price);
            }
        },
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, "")
        }
    }
}

#[test]
fn res_display() {
    let bid_price = 5.5;
    let jd = &mut serde_json::Deserializer::from_str(&DISPLAY);
    let ortb_res_result: Result<BidResponse, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_res_result {
        Ok(res) => match &res.seatbid {
            Some(sb) => {
                assert_eq!(sb[0].bid[0].price, bid_price);
            }
            None => {
                assert_eq!(0.0, bid_price);
            }
        },
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, "")
        }
    }
}

#[test]
fn res_native_1_1() {
    let bid_price = 1.0;
    let jd = &mut serde_json::Deserializer::from_str(&NATIVE_1_1);
    let ortb_res_result: Result<BidResponse, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_res_result {
        Ok(res) => match &res.seatbid {
            Some(sb) => {
                assert_eq!(sb[0].bid[0].price, bid_price);
            }
            None => {
                assert_eq!(0.0, bid_price);
            }
        },
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, "")
        }
    }
}

#[test]
fn res_native_1_2() {
    let bid_price = 2.0;
    let jd = &mut serde_json::Deserializer::from_str(&NATIVE_1_2);
    let ortb_res_result: Result<BidResponse, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_res_result {
        Ok(res) => match &res.seatbid {
            Some(sb) => {
                assert_eq!(sb[0].bid[0].price, bid_price);
            }
            None => {
                assert_eq!(0.0, bid_price);
            }
        },
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, "")
        }
    }
}

#[test]
fn res_preferred_deal() {
    let bid_price = 10.0;
    let jd = &mut serde_json::Deserializer::from_str(&PREFERRED_DEAL);
    let ortb_res_result: Result<BidResponse, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_res_result {
        Ok(res) => match &res.seatbid {
            Some(sb) => {
                assert_eq!(sb[1].bid[0].price, bid_price);
            }
            None => {
                assert_eq!(0.0, bid_price);
            }
        },
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, "")
        }
    }
}

#[test]
fn res_private_exchange() {
    let bid_price = 99.0;
    let jd = &mut serde_json::Deserializer::from_str(&PRIVATE_EXCHANGE);
    let ortb_res_result: Result<BidResponse, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_res_result {
        Ok(res) => match &res.seatbid {
            Some(sb) => {
                assert_eq!(sb[0].bid[0].price, bid_price);
            }
            None => {
                assert_eq!(0.0, bid_price);
            }
        },
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, "")
        }
    }
}

#[test]
fn res_rich_media() {
    let bid_price = 1.1;
    let imp_id = "1";
    let jd = &mut serde_json::Deserializer::from_str(&RICH_MEDIA);
    let ortb_res_result: Result<BidResponse, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_res_result {
        Ok(res) => match &res.seatbid {
            Some(sb) => {
                assert_eq!(sb[0].bid[0].price, bid_price);
                assert_eq!(sb[0].bid[0].impid, imp_id);
            }
            None => {
                assert_eq!(0.0, bid_price);
            }
        },
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, "")
        }
    }
}

#[test]
fn res_missing_bid() {
    let jd = &mut serde_json::Deserializer::from_str(&MISSING_BID);
    let ortb_res_result: Result<BidResponse, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_res_result {
        Ok(_) => {
            //should never get here
            assert_eq!(1, 2)
        }
        Err(_) => {
            //should always get here
            assert_eq!(1, 1)
        }
    }
}
