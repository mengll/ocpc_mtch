mod parse;
mod database;
use parse::{
    msgtype,
    msgtype::{
        Ocpcmatch
    }
};

use database::{
    mongo::{erfen,mgo,search_test}
};
use crate::database::mongo::Mymongo;

#[tokio::main]
async fn main(){

    let mg = Mymongo::new().await;
    let mut s = msgtype::OcpcMsg::new();
    s.mt = 1;
    s.businessid =1 ;
    s.muid = Some("theworld is so big!".to_string());
    s.oaid = Some("TYhus".to_string());
    s.match_by_muid(&mg);

    // 创建一个数据 传递的数组的内容
    let c:[i32;5] = [1,2,3,4,5];

    let mut v:Vec<i32> = vec![11,9,4,5,2,6,1,7,3,8];
    erfen(&mut v,4);
    println!("{:?}",v);

    v.truncate(4);
    println!("{:?}",v);
    kk().await;
    // 读取Mongo的数据
    // mgo().await;
    // search_test().await;

    // 246 kafka
    let brokes = "192.168.1.246:9092";
    let group_id  = "mll";
    let topic :Vec<&str> = vec!["anasdk"];
    parse::afkafka::consume_and_print(brokes,group_id,&topic).await;
}

// yibutest
async fn kk(){
      let handle = tokio::spawn(async move{
          "return value"
      });

    // any的类型检查的实现
    let out = handle.await.unwrap();
    println!("Got {}", out);
    if let Some(a) = Some(12) {
         println!("The value: {}",a)
    }
}

