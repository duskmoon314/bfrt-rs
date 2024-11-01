use bfrt::{
    bfrt::{
        bf_runtime_client::BfRuntimeClient, stream_message_request,
        stream_message_response::Update, subscribe::Notifications, ForwardingPipelineConfig,
        GetForwardingPipelineConfigRequest, GetForwardingPipelineConfigResponse,
        SetForwardingPipelineConfigRequest, SetForwardingPipelineConfigResponse,
        StreamMessageRequest, StreamMessageResponse, Subscribe,
    },
    google,
};
use tonic::transport::Channel;

use crate::{bfrt_info::BFRTInfo, DepsError, GetBFRTInfoError};

/// BFRuntime client wrapper
#[derive(Debug, derive_builder::Builder)]
pub struct Client {
    /// bfrt tonic client
    #[builder(default = None)]
    pub bfrt_client: Option<BfRuntimeClient<Channel>>,

    /// device id
    #[builder(default = 0)]
    pub device_id: u32,

    /// client id
    #[builder(default = 1)]
    pub client_id: u32,

    /// stream channel buffer size
    #[builder(default = 10000)]
    pub channel_buffer_size: usize,

    /// request sender
    #[builder(setter(skip), default = None)]
    pub stream_message_sender: Option<tokio::sync::mpsc::Sender<StreamMessageRequest>>,

    /// response receiver
    #[builder(setter(skip), default = None)]
    pub stream_message_receiver: Option<tonic::Streaming<StreamMessageResponse>>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }
}

impl Client {
    pub async fn connect<D>(&mut self, dst: D) -> Result<(), tonic::transport::Error>
    where
        D: TryInto<tonic::transport::Endpoint>,
        D::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
        let bfrt_client = BfRuntimeClient::connect(dst).await?;
        self.bfrt_client = Some(bfrt_client);
        Ok(())
    }

    /// Run the client by preparing channels and sending subscribe requests
    pub async fn run(&mut self) -> anyhow::Result<()> {
        let (request_sender, request_receiver) =
            tokio::sync::mpsc::channel::<StreamMessageRequest>(self.channel_buffer_size);

        self.stream_message_sender = Some(request_sender);

        self.subscribe(request_receiver).await?;

        Ok(())
    }

    pub async fn subscribe(
        &mut self,
        request_receiver: tokio::sync::mpsc::Receiver<StreamMessageRequest>,
    ) -> anyhow::Result<()> {
        let req = StreamMessageRequest {
            client_id: self.client_id,
            update: Some(stream_message_request::Update::Subscribe(Subscribe {
                device_id: self.device_id,
                notifications: Some(Notifications {
                    enable_learn_notifications: true,
                    enable_idletimeout_notifications: true,
                    enable_port_status_change_notifications: true,
                    enable_entry_active_notifications: true,
                }),
                status: None,

                ..Default::default()
            })),
        };

        self.stream_message_sender
            .as_ref()
            .unwrap()
            .send(req)
            .await?;

        let channel = self
            .bfrt_client
            .as_mut()
            .unwrap()
            .stream_channel(tokio_stream::wrappers::ReceiverStream::new(
                request_receiver,
            ))
            .await?
            .into_inner();

        self.stream_message_receiver = Some(channel);

        // Check if subscribe is successful
        let res = self
            .stream_message_receiver
            .as_mut()
            .unwrap()
            .message()
            .await?;

        match res {
            Some(StreamMessageResponse {
                update:
                    Some(Update::Subscribe(Subscribe {
                        status: Some(google::rpc::Status { code: 0, .. }),
                        ..
                    })),
            }) => Ok(()),
            _ => anyhow::bail!("Failed to subscribe"),
        }
    }

    /// Get the BFRT Info
    pub async fn get_bfrt_info(
        &mut self,
        p4_name: Option<String>,
    ) -> Result<BFRTInfo, GetBFRTInfoError> {
        let pipeline_config = self.get_forwarding_pipeline_config().await?.into_inner();

        let non_p4_config = pipeline_config.non_p4_config.map(|c| c.bfruntime_info);

        let pipeline_config = match p4_name {
            Some(p4_name) => pipeline_config.config.iter().find(|c| c.p4_name == p4_name),
            None => pipeline_config.config.first(),
        }
        .ok_or(GetBFRTInfoError::ConfigNotFound)?;

        let bfrt_info = BFRTInfo::new(
            pipeline_config.p4_name.clone(),
            &pipeline_config.bfruntime_info,
            non_p4_config,
        )?;

        Ok(bfrt_info)
    }

    /// Bind to a program on the device
    pub async fn bind_pipeline_config(&mut self, p4_name: String) -> Result<(), DepsError> {
        let config = ForwardingPipelineConfig {
            p4_name,
            ..Default::default()
        };

        // TODO: In the proto, all result indicates success. Need to check if it's really successful
        let _ = self
            .set_forwarding_pipeline_config(
                bfrt::bfrt::set_forwarding_pipeline_config_request::Action::Bind,
                bfrt::bfrt::set_forwarding_pipeline_config_request::DevInitMode::FastReconfig,
                "".to_string(),
                vec![config],
            )
            .await?
            .into_inner();

        Ok(())
    }
}

/// Simple wrapper for original methods
impl Client {
    /// Get the forwarding pipeline config
    pub async fn get_forwarding_pipeline_config(
        &mut self,
    ) -> Result<tonic::Response<GetForwardingPipelineConfigResponse>, tonic::Status> {
        let req = GetForwardingPipelineConfigRequest {
            device_id: self.device_id,
            client_id: self.client_id,
        };

        self.bfrt_client
            .as_mut()
            .unwrap()
            .get_forwarding_pipeline_config(req)
            .await
    }

    /// Set the forwarding pipeline config
    pub async fn set_forwarding_pipeline_config(
        &mut self,
        action: bfrt::bfrt::set_forwarding_pipeline_config_request::Action,
        dev_init_mode: bfrt::bfrt::set_forwarding_pipeline_config_request::DevInitMode,
        base_path: String,
        config: Vec<bfrt::bfrt::ForwardingPipelineConfig>,
    ) -> Result<tonic::Response<SetForwardingPipelineConfigResponse>, tonic::Status> {
        self.bfrt_client
            .as_mut()
            .unwrap()
            .set_forwarding_pipeline_config(SetForwardingPipelineConfigRequest {
                device_id: self.device_id,
                client_id: self.client_id,
                action: action as i32,
                dev_init_mode: dev_init_mode as i32,
                base_path,
                config,
            })
            .await
    }
}
