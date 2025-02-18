use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder as ReflectionBuilder;
use tokio_stream::wrappers::{IntervalStream,ReceiverStream};
use tokio_stream::StreamExt; // Importa questo trait!
// use tokio_stream::wrappers::ReceiverStream;
use tokio::time::Duration;
use futures::Stream;
use std::pin::Pin;
use tokio::sync::mpsc;

mod descriptor;


pub mod envoy {
    tonic::include_proto!("envoy.service.discovery.v3");
}

use envoy::aggregated_discovery_service_server::{
    AggregatedDiscoveryService, AggregatedDiscoveryServiceServer,
};

use envoy::{DiscoveryRequest, DiscoveryResponse};

type ResponseStream = Pin<Box<dyn Stream<Item = Result<DiscoveryResponse, Status>> + Send + 'static>>;

#[derive(Debug, Default)]
pub struct ControlPlane {}

// #[tonic::async_trait]
// impl AggregatedDiscoveryService for ControlPlane {
//     type StreamAggregatedResourcesStream = ResponseStream;

//     async fn stream_aggregated_resources(
//         &self,
//         request: Request<tonic::Streaming<DiscoveryRequest>>,
//     ) -> Result<Response<Self::StreamAggregatedResourcesStream>, Status> {
//         println!("Ricevuta connessione xDS: {:?}", request);


//         let response = DiscoveryResponse {
//             version_info: "v1".into(),
//             nonce: "nonce-1".into(),
//             type_url: "type.googleapis.com/envoy.config.cluster.v3.Cluster".into(),
//         };


//         let output = tokio_stream::iter(vec![Ok(response)]);
//         Ok(Response::new(Box::pin(output) as Self::StreamAggregatedResourcesStream))
//     }
// }
// #[tonic::async_trait]
// impl AggregatedDiscoveryService for ControlPlane {
//     type StreamAggregatedResourcesStream = ResponseStream;

//     async fn stream_aggregated_resources(
//         &self,
//         request: Request<tonic::Streaming<DiscoveryRequest>>,
//     ) -> Result<Response<Self::StreamAggregatedResourcesStream>, Status> {
//         println!("Ricevuta connessione xDS: {:?}", request);

//         // Crea un intervallo che "ticca" ogni 10 secondi
//         let interval = tokio::time::interval(Duration::from_secs(10));
//         let stream = IntervalStream::new(interval).map(|_| {
//             Ok(DiscoveryResponse {
//                 version_info: "v1".into(),
//                 nonce: "nonce-1".into(),
//                 type_url: "type.googleapis.com/envoy.config.cluster.v3.Cluster".into(),
//             })
//         });

//         Ok(Response::new(Box::pin(stream) as Self::StreamAggregatedResourcesStream))
//     }
// }

#[tonic::async_trait]
impl AggregatedDiscoveryService for ControlPlane {
    type StreamAggregatedResourcesStream = ResponseStream;

    async fn stream_aggregated_resources(
        &self,
        request: Request<tonic::Streaming<DiscoveryRequest>>,
    ) -> Result<Response<Self::StreamAggregatedResourcesStream>, Status> {
        println!("Ricevuta connessione xDS: {:?}", request);

        // Creiamo un canale per inviare aggiornamenti in modo asincrono.
        let (tx, rx) = mpsc::channel(16);

        // Avviamo un task che invia periodicamente aggiornamenti (o keep-alive).
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                // Costruiamo una risposta di esempio.
                let response = DiscoveryResponse {
                    version_info: "v1".into(),
                    nonce: "nonce-1".into(),
                    type_url: "type.googleapis.com/envoy.config.cluster.v3.Cluster".into(),
                };
                // Inviamo la risposta sul canale; se il canale è chiuso, usciamo dal loop.
                if tx.send(Ok(response)).await.is_err() {
                    break;
                }
            }
        });

        // Converto il ricevitore in uno stream.
        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream) as Self::StreamAggregatedResourcesStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let control_plane = ControlPlane::default();
    // Costruiamo il servizio di reflection, includendo il nostro servizio
    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(descriptor::file_descriptor_set())
        .build_v1()?;

    println!("Control Plane in ascolto su {}", addr);

    Server::builder()
        .add_service(AggregatedDiscoveryServiceServer::new(control_plane))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
