use serde::{Deserialize, Serialize};
use mongodb::{
    Collection,
};
use mongodb::bson::doc;
use async_std::task;
use crate::database::mongo::Mymongo;
// 日志格式定义

#[derive(Debug,Deserialize,Serialize)]
pub struct OcpcMsg {
    pub mt:i8,          // 1 展示数据  2 广告点击数据 3 sdk 检测数据
    pub businessid:i8,  // 公司主体  1 安锋 7 团团
    pub actype:i8,     // 数据检测类型
    pub status:i8,      // 数据状态  -1 广告数据归因   1 当前激活  2 当前注册 3 当前付费  5 设备不做上报处理
    pub channel:i32,    // 广告媒体
    pub pageid:i64,    // 落地页idString
    pub eventts:i64,   // 广告创建时间
    pub clicktime:i64, // 广告点击时间
    pub pid:i64,        // 游戏包id
    pub rid:i64,        // 计划id
    pub apkid:i64,     // 马甲包id
    pub nginxts:i64,   // 到达你nginx 的时间
    pub nginxday:Option<String>, // 访问ngin的时间
    pub os:Option<String>,        // 当前的操作系统
    pub delivery:Option<String>,  // 新数回传参数
    pub baclurl:Option<String>,
    pub idfa:Option<String>,
    pub idfv:Option<String>,    // 苹果的设备信息
    pub qaid:Option<String>,    // 苹果的qaid 设备信息
    pub imei:Option<String>,    // 监测的设备信息
    pub muid:Option<String>,    // 设备信息的md5值
    pub apptype:Option<String>, // app的类型
    pub ip:Option<String>,      //
    pub userid:Option<String>,  // 投放账户的id
    pub mac:Option<String>,     // 投放mac
    pub uuid:Option<String>,    // 用户id
    pub akey:Option<String>,    // 百度上报参数
    pub appid:Option<String>,   // 广告监测的应用id
    pub clickid:Option<String>, // 广告点击地址
    pub productid:Option<String>, //
    pub deviceid:Option<String>,   // 设备信息
    pub signature:Option<String>,  // 签名信息
    pub sign :Option<String>,      // 签名字段
    pub adgropup:Option<String>,   // 广告组id
    pub adid :Option<String>,      // 广告id
    pub cid :Option<String>,       // 广告cid
    pub ismd:Option<String>,       // 是否是加密
    pub slot:Option<String>,       // 优化参数
    pub unionsite:Option<String>,  // 关联站点
    pub ua :Option<String>,        // ua
    pub oaid:Option<String>,       // oaid
    pub androidid:Option<String>,  // Android
    pub ocpcuuid:Option<String>,   // 数据账户的id
    pub extendmatch:Option<Vec<String>>  // 扩展匹配的字段
}

pub trait Ocpcevent {
     fn activate(&self);
     fn register(&self);
     fn pay(&self);
     fn imevent(&self);
}

// 归因匹配的相关的操作
pub trait Ocpcmatch {
    fn match_by_muid(&self,mg:&Mymongo); // 通过设备信息匹配
    fn match_by_oaid(&self); // 通过caid的操作
    fn match_by_caid(&self); // ios14 新增的匹配
    fn match_by_event(&self);// 新增时间类型的处理

    fn insert_dat(&self); // 数据写入
    fn update_dat(&self); // 数据更新
    fn send_tosdk(&self); // iOS的归因数据发送到sdk
}

impl Default for OcpcMsg {
    fn default() ->Self{
        Self{
            mt:0,          // 1 展示数据  2 广告点击数据 3 sdk 检测数据
            businessid:0,  // 公司主体  1 安锋 7 团团
            actype:0,     // 数据检测类型
            status:0,      // 数据状态  -1 广告数据归因   1 当前激活  2 当前注册 3 当前付费  5 设备不做上报处理
            channel:0,    // 广告媒体
            pageid:0,    // 落地页id
            eventts:0,   // 广告创建时间
            clicktime:0, // 广告点击时间
            pid:0,        // 游戏包id
            rid:0,        // 计划id
            apkid:0,     // 马甲包id
            nginxts:0,   // 到达你nginx 的时间
            nginxday:Some("".to_string()), // 访问ngin的时间
            os:Some("".to_string()),        // 当前的操作系统
            delivery:Some("".to_string()),  // 新数回传参数
            baclurl:Some("".to_string()),
            idfa:Some("".to_string()),
            idfv:Some("".to_string()),    // 苹果的设备信息
            qaid:Some("".to_string()),    // 苹果的qaid 设备信息
            imei:Some("".to_string()),    // 监测的设备信息
            muid:Some("".to_string()),    // 设备信息的md5值
            apptype:Some("".to_string()), // app的类型
            ip:Some("".to_string()),      //
            userid:Some("".to_string()),  // 投放账户的id
            mac:Some("".to_string()),     // 投放mac
            uuid:Some("".to_string()),    // 用户id
            akey:Some("".to_string()),    // 百度上报参数
            appid:Some("".to_string()),   // 广告监测的应用id
            clickid:Some("".to_string()), // 广告点击地址
            productid:Some("".to_string()), //
            deviceid:Some("".to_string()),   // 设备信息
            signature:Some("".to_string()),  // 签名信息
            sign :Some("".to_string()),      // 签名字段
            adgropup:Some("".to_string()),   // 广告组id
            adid :Some("".to_string()),      // 广告id
            cid :Some("".to_string()),       // 广告cid
            ismd:Some("".to_string()),       // 是否是加密
            slot:Some("".to_string()),       // 优化参数
            unionsite:Some("".to_string()),  // 关联站点
            ua :Some("".to_string()),        // ua
            oaid:Some("".to_string()),       // oaid
            androidid:Some("".to_string()),  // Android
            ocpcuuid:Some("".to_string()),   // 数据账户的id
            extendmatch:None
        }
    }
}

impl OcpcMsg {
    pub fn new()->Self{
        OcpcMsg{ ..Default::default()}
    }
}

impl  Ocpcmatch for OcpcMsg {
    fn match_by_muid(&self,col:&Mymongo)->(){
        if self.muid == Some("".to_string()) || self.muid ==Some("12".to_string()) {
            println!("数据类型为空");
            return
        }

        // 执行可变数据的查询
        task::block_on( async{
            let data = col.search(doc!{"muid":"d04824d869deb938d5f49a5abbd99d4b","nginxts":{$gte:1607042516}},"dspdata").await;
            match data {
                Some(dat) => println!("inner=》{:?}",dat),
                None=> println!("The value is None")
            }
         });

        // 查询当前的
        println!("有数据了匹配当前的数据库的操作");
    }

    fn match_by_oaid(&self){
        if self.oaid == Some("".to_string()) {
            return
        }
        println!("现在是匹配处理的oaid")
    }

    fn match_by_caid(&self){
        if self.cid == Some("".to_string()) {
            println!("caid is empty")
        }
        println!("现在处理的是caid的操作")
    }

    fn match_by_event(&self){
        println!("The resource")
    }


    // 未能正确的匹配到相关额数据将数据
    fn insert_dat(&self){
    }

    //
    fn update_dat(&self){

    }

    // 发送归因的数据到sdk
    fn send_tosdk(&self){

    }

}
