use crate::dto::bid_request::BidRequest;
use crate::sample_ortb_req::*;
use serde_json;

#[test]
fn req_instream_pre_roll() {
    let exp = "e5411942-39a1-4019-afdd-82955584124e";
    let jd = &mut serde_json::Deserializer::from_str(&INSTREAM_PRE_ROLL);
    let ortb_req_result: Result<BidRequest, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_req_result {
        Ok(v) => {
            assert_eq!(v.id, exp);
        }
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, exp)
        }
    }
}

#[test]
fn req_instream_mid_roll() {
    let exp = "2a1c9224-0803-41da-93d1-7d5b3d2010a7";
    let jd = &mut serde_json::Deserializer::from_str(&INSTREAM_MID_ROLL);
    let ortb_req_result: Result<BidRequest, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_req_result {
        Ok(v) => {
            assert_eq!(v.id, exp);
        }
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, exp)
        }
    }
}

#[test]
fn req_instream_post_roll() {
    let exp = "68d49693-6312-40c1-b9bd-68f92e65acac";
    let jd = &mut serde_json::Deserializer::from_str(&INSTREAM_POST_ROLL);
    let ortb_req_result: Result<BidRequest, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_req_result {
        Ok(v) => {
            assert_eq!(v.id, exp);
        }
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, exp)
        }
    }
}

#[test]
fn req_instream_interstitial() {
    let exp = "8d0160b7-8f48-404a-a060-84eda7b61012";
    let jd = &mut serde_json::Deserializer::from_str(&INSTREAM_INTERSTITIAL);
    let ortb_req_result: Result<BidRequest, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_req_result {
        Ok(v) => {
            assert_eq!(v.id, exp);
        }
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, exp)
        }
    }
}

#[test]
fn req_outstream() {
    let exp = "ece5db3d-fc5a-4a98-91e8-c81df883bc62";
    let jd = &mut serde_json::Deserializer::from_str(&OUTSTREAM);
    let ortb_req_result: Result<BidRequest, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_req_result {
        Ok(v) => {
            assert_eq!(v.id, exp);
        }
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, exp)
        }
    }
}

#[test]
fn req_rewarded() {
    let exp = "ff093fd0-c6ba-4de6-968f-80668c075847";
    let jd = &mut serde_json::Deserializer::from_str(&REWARDED);
    let ortb_req_result: Result<BidRequest, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_req_result {
        Ok(v) => {
            assert_eq!(v.id, exp);
        }
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, exp)
        }
    }
}

#[test]
fn req_display() {
    let exp = "820e63f1-d68a-4267-ad8d-c63b9329d3db";
    let jd = &mut serde_json::Deserializer::from_str(&DISPLAY);
    let ortb_req_result: Result<BidRequest, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_req_result {
        Ok(v) => {
            assert_eq!(v.id, exp);
        }
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, exp)
        }
    }
}

#[test]
fn req_native_1_1() {
    let exp_id = "268f7e51-11a9-45fd-ab6f-72b0cc3bf5e0";
    let exp_ver = "1.1";
    let jd = &mut serde_json::Deserializer::from_str(&NATIVE_1_1);
    let ortb_req_result: Result<BidRequest, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_req_result {
        Ok(v) => {
            assert_eq!(v.id, exp_id);
            match &v.imp[0].native {
                Some(n) => match &n.ver {
                    Some(ver) => {
                        assert_eq!(ver, exp_ver);
                    }
                    None => {
                        assert_eq!("".to_owned(), exp_ver);
                    }
                },
                None => {
                    assert_eq!("".to_owned(), exp_ver);
                }
            }
        }
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, exp_id)
        }
    }
}

#[test]
fn req_native_1_2() {
    let exp_id = "5a514203-b7a9-48d0-ad2e-4da05bf0c735";
    let exp_ver = "1.2";
    let jd = &mut serde_json::Deserializer::from_str(&NATIVE_1_2);
    let ortb_req_result: Result<BidRequest, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_req_result {
        Ok(v) => {
            assert_eq!(v.id, exp_id);
            match &v.imp[0].native {
                Some(n) => match &n.ver {
                    Some(ver) => {
                        assert_eq!(ver, exp_ver);
                    }
                    None => {
                        assert_eq!("".to_owned(), exp_ver);
                    }
                },
                None => {
                    assert_eq!("".to_owned(), exp_ver);
                }
            }
        }
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, exp_id)
        }
    }
}

#[test]
fn req_preferred_deal() {
    let exp_id = "136c8024-1688-4110-95af-daf3a8468e3d";
    let pa = 1;
    let jd = &mut serde_json::Deserializer::from_str(&PREFERRED_DEAL);
    let ortb_req_result: Result<BidRequest, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_req_result {
        Ok(v) => {
            assert_eq!(v.id, exp_id);
            match &v.imp[0].pmp {
                Some(p) => {
                    assert_eq!(p.private_auction, pa);
                }
                None => {
                    assert_eq!(0, pa);
                }
            }
        }
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, exp_id)
        }
    }
}

#[test]
fn req_private_exchange() {
    let exp_id = "93cd8f83-c5c7-44ed-8294-01c9d7de81fc";
    let at = 2;
    let jd = &mut serde_json::Deserializer::from_str(&PRIVATE_EXCH);
    let ortb_req_result: Result<BidRequest, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_req_result {
        Ok(v) => {
            assert_eq!(v.id, exp_id);
            match &v.imp[0].pmp {
                Some(p) => match &p.deals {
                    Some(d) => match d[0].at {
                        Some(d_at) => {
                            assert_eq!(d_at, at);
                        }
                        None => {
                            assert_eq!(0, at);
                        }
                    },
                    None => {
                        assert_eq!(0, at);
                    }
                },
                None => {
                    assert_eq!(0, at);
                }
            }
        }
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, exp_id)
        }
    }
}

#[test]
fn req_rich_media_android() {
    let exp = "f9078d56-66cb-4e2a-9b22-9418b102b00a";
    let jd = &mut serde_json::Deserializer::from_str(&RICH_MEDIA_ANDROID);
    let ortb_req_result: Result<BidRequest, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_req_result {
        Ok(v) => {
            assert_eq!(v.id, exp);
        }
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, exp)
        }
    }
}

#[test]
fn req_rich_media_ios() {
    let exp = "42bf6a9b-4532-4c27-b94f-caa8a9a30b87";
    let jd = &mut serde_json::Deserializer::from_str(&RICH_MEDIA_IOS);
    let ortb_req_result: Result<BidRequest, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_req_result {
        Ok(v) => {
            assert_eq!(v.id, exp);
        }
        Err(e) => {
            let err = format!("{:?}", e);
            assert_eq!(err, exp)
        }
    }
}

#[test]
fn req_missing_imp() {
    let jd = &mut serde_json::Deserializer::from_str(&MISSING_IMP);
    let ortb_req_result: Result<BidRequest, serde_path_to_error::Error<serde_json::Error>> =
        serde_path_to_error::deserialize(jd);
    match ortb_req_result {
        Ok(v) => {
            //should never get here
            assert_eq!(v.id, "");
        }
        Err(_) => {
            //should always get here
            assert_eq!(1, 1)
        }
    }
}
