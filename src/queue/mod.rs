use std::sync::Arc;

use envelope::Envelope;
use futures::stream::StreamExt;
use lapin::{
    options::{BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties,
};

use crate::{config, context::Context};

mod envelope;

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

    pub async fn consume(context: &Arc<Context>, channel: Channel) -> Result<(), lapin::Error> {
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
                    let message: Result<Envelope<_, _, _>, serde_json::Error> =
                        serde_json::from_slice(&delivery.data);

                    match message {
                        Ok(data) => {
                            let _ = Self::handle_message::<String, String, String>(&context, data);
                        }
                        Err(err) => eprintln!("Error parsing message: {:?}", err),
                    }

                    channel
                        .basic_ack(delivery.delivery_tag, Default::default())
                        .await?;
                }
                Err(e) => eprintln!("Error while consuming message: {:?}", e),
            }
        }

        Ok(())
    }

    fn handle_message<T, P, M>(
        context: &Arc<Context>,
        message: Envelope<T, P, M>,
    ) -> Result<bool, String> {
        let mut connection = context
            .redis_pool
            .get()
            .map_err(|error| format!("Failed to get Redis connection: {}", error))?;

        // Add message handling here
        Ok(true)
    }
}
