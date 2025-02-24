use futures::stream::StreamExt;
use lapin::{
    options::{BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties,
};

use crate::config;

pub struct RabbitMQ;

impl RabbitMQ {
    pub async fn connect() -> Result<Channel, lapin::Error> {
        let settings = config::rabbit_mq_settings();
        let connection =
            Connection::connect(&settings.url, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;

        channel
            .queue_declare(
                &settings.queue,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(channel)
    }

    pub async fn consume(channel: Channel) -> Result<(), lapin::Error> {
        let settings = config::rabbit_mq_settings();

        let mut consumer = channel
            .basic_consume(
                &settings.queue,
                &settings.consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        println!("Started consuming messages from RabbitMQ...");

        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    println!("Received: {:?}", String::from_utf8_lossy(&delivery.data));

                    channel
                        .basic_ack(delivery.delivery_tag, Default::default())
                        .await?;
                }
                Err(e) => eprintln!("Error while consuming message: {:?}", e),
            }
        }

        Ok(())
    }
}
