use crate::parse::msgtype::OcpcMsg;
use std::collections::HashMap;
use std::ops::Deref;
use tokio::macros::support::Pin;

// parse_msg  解析数据
pub fn parse_msg<'a>(msg:&'a str) ->OcpcMsg{
    let ms:HashMap<&str,String> = serde_json::from_str(msg).unwrap();
    let mut  ocpc = OcpcMsg::new();
     for  (key,v) in ms{
        match key {
                    "gameid" |"game_id" => {
                        let pid:i64 = v.parse().unwrap();
                            ocpc.pid = pid;
                        },
                        "muid"|"meiMD5"|"uid"|"imei_md5"|"idfaMD5"|"imeiMD5"|"oaid_md5" =>{
                            if v != "__OAID_MD5__" {
                                ocpc.muid = Some(v);
                            }
                        },
                        "callback"|"baclurl"|"callback_url" =>{
                            ocpc.baclurl = Some(v);
                        },
                        "rid" =>{
                            let rid :i64 = v.parse().unwrap_or(0);
                            ocpc.rid = rid;
                        },
                        "ostype"|"os"|"OS"|"device_os_type" =>{
                            ocpc.os = Some(v);
                        },
                        "idfa"|"ifa"|"IDFA" =>{
                            ocpc.idfa = Some(v);
                        },
                        "imei" =>{
                            ocpc.imei = Some(v);
                        },
                        "apptype"|"app_type" =>{
                            ocpc.apptype = Some(v);
                        },
                        "ip"|"Ip"=>{
                            ocpc.ip = Some(v);
                        },
                        "advertiser_id"|"company_id"|"company"|"companyid"|"userid" =>{
                            ocpc.userid = Some(v);
                        },
                        "mac"|"MAC_MD5" =>{
                            ocpc.mac = Some(v);
                        },
                        "uuid"|"UniqueID" => {
                            ocpc.uuid = Some(v);
                        },
                        "akey"|"c"|"key"|"delivery" =>{
                            ocpc.akey = Some(v);
                        },
                        "appid" |"app" =>{
                            ocpc.appid = Some(v);
                        },
                        "click_id" =>{
                            ocpc.clickid = Some(v);
                        },
                        "productid" =>{
                            ocpc.productid = Some(v);
                        },
                        "deviceid"|"devicetype" =>{
                            ocpc.deviceid = Some(v);
                        },
                        "signature"|"p"|"source" =>{
                            ocpc.signature = Some(v);
                        },
                        "sign"|"token" =>{
                            ocpc.sign = Some(v);
                        },
                        "campaign_id"|"outer_campaign_id"|"wx_adgroup_id" =>{
                            ocpc.adgropup = Some(v);
                        },
                        "adid"|"adgroup_id"|"outer_adgroup_id"|"pid"|"wx_campaign_id" =>{
                            ocpc.adid = Some(v);
                        },
                        "cid"|"creative_id"|"outer_creative_id"|"aid"|"wx_creative_id" => {
                            ocpc.cid = Some(v);
                        },
                        "is_md" =>{
                            ocpc.ismd = Some(v);
                        },
                        "slot" =>{
                            ocpc.slot = Some(v);
                        },
                        "union_site,adgroup_name" =>{
                            ocpc.unionsite = Some(v);
                        },
                        "ua" =>{
                            ocpc.ua = Some(v);
                        },
                        "oaid" =>{
                            if v != "__OAID__" {
                                ocpc.oaid = Some(v);
                            }
                        },
                        "mt" =>{
                            let mt = format!("{:.1}",v);
                            let mt:i8 = mt.parse().unwrap_or(0);
                            ocpc.mt = mt;
                        },
                        "business_id" =>{
                            let bs:i8 = v.parse().unwrap();
                            ocpc.businessid = bs;
                        },
                        "page_id" =>{
                            let page_id:i64 = v.parse().unwrap();
                            ocpc.pageid = page_id;
                        },
                        "click_time"|"ts" =>{
                            let ts:i64 = v.parse().unwrap_or(0);
                            ocpc.clicktime = ts;
                            ocpc.nginxts = ts;
                        },
                        "channel"|"ch"|"af_channel" =>{
                            let ch:i32 = v.parse().unwrap_or(0);
                            if ch != 0 {
                                ocpc.channel = ch;
                            }
                        },
                        "apk_id" =>{
                            let gid:i64 = v.parse().unwrap_or(0);
                            ocpc.apkid = gid;
                        }
                        _=> ()
                    }
     }
    println!("parse_msg=>{:?}",ocpc);
    //Pin::new(Box::new(ocpc))
    ocpc
}
