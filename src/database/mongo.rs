use mongodb::{Client, options::{ClientOptions, FindOptions}, Collection, Database, Cursor};
use mongodb::bson::doc;
use tokio::stream::StreamExt;
use mongodb::{
    bson::{Document,Bson},
    bson,
};

use serde::{Serialize, Deserialize};
use mongodb::error::Error;
use crate::parse::msgtype::{OcpcMsg};

// 链接Mongolia 的操作
pub fn erfen(a:&mut Vec<i32>,stop:usize){
    let num = a.len()-1;
    for i in 0..num {
        for k in 0..num-i{
            if a[k] > a[k+1] {
                let temp = a[k];
                a[k] = a[k+1];
                a[k+1] = temp;
            }
        }
        if i > stop {
            println!("runjhewr===>{}",i);
            break;
        }
    }
}

pub struct Mymongo {
    pub client:Client,
    pub db:Database,
}

#[derive(Serialize, Deserialize)]
struct O {
    pid:i64
}

// 内核级别的速度
impl Mymongo {
    // 创建链接库
    pub async fn new() ->Self {
        let client_options = ClientOptions::parse("mongodb://root:xgbV5kY==wapMo852@192.168.1.241:44126/admin").await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("dsp");
        Self{client:client,db:db}
    }

    // 查询的
   pub async fn search(&self,search:impl Into<Option<Document>>,table:&str )->Option<OcpcMsg>{
        println!("The mongodb: {}",table);
        let  res = self.db.collection(table).find_one(search,None).await;
        if res.is_err() {
            println!("读取错误=》 {:?}",res.err());
            return None
        }
        let op = res.unwrap();
        if op.is_none() {
            return None
        }
        let res  = op.unwrap() as Document;
        let rz = res.clone();
        let mut d:OcpcMsg = bson::from_document(res).unwrap();
        println!("new_reerr=>{:?}",d);
        Some(d)
    }
}

// 链接数据库
pub  async fn mgo(){
    let mgo = Mymongo::new().await;
    let adtconf = mgo.db.collection("dspdata");
    let limit  = FindOptions::builder().limit(1).build();
    let res  = adtconf.find(doc!{"muid":"d04824d869deb938d5f49a5abbd99d4b"},limit).await;
    if !res.is_err() {
        return
    }

    let mut cur:Cursor = match res {
        Ok(t) => t,
        Err(e)=> {
            println!("{:?}",e);
            return
        }
    };

    while let Some(result) =  cur.next().await {
        match result {
            Ok(docs)=> {
                println!("dock");
                let d = docs as Document;
                let s = d.get("pid").unwrap().as_i32().unwrap();
                println!("mgo msg=> {:?}",s)},
            Err(e)=> println!("e：a {:?}",e.labels())
        }
    }
}

// mongo search test
pub async fn search_test(){
    let mg = Mymongo::new().await;
    let data = mg.search(doc!{"muid":"d04824d869deb938d5f49a5abbd99d4b"},"dspdata").await.unwrap();
    println!("search data => {:?}",data);
}

// 数据写入
pub async fn write_db(){
}