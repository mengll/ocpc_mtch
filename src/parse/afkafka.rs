use rdkafka::{ClientConfig, ClientContext, TopicPartitionList, Message};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{ConsumerContext, Rebalance, CommitMode, StreamConsumer, Consumer};
use rdkafka::error::KafkaResult;
use tokio::stream::StreamExt;
use rdkafka::message::Headers;
use log::{info, warn};
use crate::parse::logparse::parse_msg;
use crate::parse::msgtype::Ocpcmatch;

struct CustomContext;
impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}
type LoggingConsumer = StreamConsumer<CustomContext>;
use crate::database::mongo;
use crate::database::mongo::Mymongo;

// consume_and_print 传递的参数分别是集群的地址 分组id 消费的主题名称
pub async fn consume_and_print(brokers: &str, group_id: &str, topics: &[&str]) {
    println!("Consumer broker:{} group:{} topic:{:#?}",brokers,group_id,topics);
    let context:CustomContext = CustomContext;
    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");
    // consumer.start() returns a stream. The stream can be used ot chain together expensive steps,
    // such as complex computations on a thread pool or asynchronous IO.
    let mut message_stream = consumer.start();
    println!("start consumer");

    let mg = Mymongo::new().await;

    while let Some(message) = message_stream.next().await {
        println!("get msg");
        match message {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "none",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };

                println!("--->{:?}",m);
                let msg = parse_msg(payload);

                // 匹配 imei,imei2,idfa
                msg.match_by_muid(&mg);

                println!("one world one dream => {:?}",msg); // 同一个世界，同一个梦想

                println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                         m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());

                info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for i in 0..headers.count() {
                        let header = headers.get(i).unwrap();
                        info!("  Header {:#?}: {:?}", header.0, header.1);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}