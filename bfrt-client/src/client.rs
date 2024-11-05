use bfrt::bfrt::{
    bf_runtime_client::BfRuntimeClient, stream_message_request, stream_message_response::Update,
    subscribe::Notifications, ForwardingPipelineConfig, StreamMessageRequest, Subscribe,
};
use log::{debug, info, trace};
use tokio_util::sync::CancellationToken;
use tonic::transport::Channel;

use crate::{bfrt_info::BFRTInfo, table::Table, ClientBasicError, GetBFRTInfoError};

/// BFRuntime client wrapper
#[derive(Debug, derive_builder::Builder)]
pub struct Client {
    /// bfrt tonic client
    #[builder(default = None)]
    pub bfrt_client: Option<BfRuntimeClient<Channel>>,

    /// device id
    #[builder(default = 0)]
    pub device_id: u32,

    /// pipe id
    ///
    /// If the arch supports 4 pipes, this value can be 0, 1, 2, 3
    ///
    /// default is 0xFFFF, which means all pipes
    #[builder(default = 0xFFFF)]
    pub pipe_id: u32,

    /// direction
    ///
    /// Ingress/Egress
    #[builder(default = 0xFF)]
    pub direction: u32,

    /// parser id
    #[builder(default = 0xFF)]
    pub prsr_id: u32,

    /// client id
    #[builder(default = 1)]
    pub client_id: u32,

    /// p4_name
    #[builder(default = None)]
    pub p4_name: Option<String>,

    #[builder(default = None)]
    pub bfrt_info: Option<BFRTInfo>,

    /// stream channel buffer size
    #[builder(default = 10000)]
    pub channel_buffer_size: usize,

    /// cancel token
    #[builder(setter(skip), default)]
    cancel_token: CancellationToken,

    /// request sender
    #[builder(setter(skip), default = None)]
    pub stream_message_sender: Option<tokio::sync::mpsc::Sender<StreamMessageRequest>>,

    #[builder(setter(skip), default = None)]
    subscribe_rx: Option<tokio::sync::broadcast::Receiver<bfrt::bfrt::Subscribe>>,

    #[builder(setter(skip), default = None)]
    digest_rx: Option<tokio::sync::broadcast::Receiver<bfrt::bfrt::DigestList>>,

    #[builder(setter(skip), default = None)]
    idle_timeout_rx: Option<tokio::sync::broadcast::Receiver<bfrt::bfrt::IdleTimeoutNotification>>,

    #[builder(setter(skip), default = None)]
    port_status_change_rx:
        Option<tokio::sync::broadcast::Receiver<bfrt::bfrt::PortStatusChgNotification>>,

    #[builder(setter(skip), default = None)]
    set_forwarding_pipeline_config_rx:
        Option<tokio::sync::broadcast::Receiver<bfrt::bfrt::SetForwardingPipelineConfigResponse>>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn subscribe_rx(&self) -> tokio::sync::broadcast::Receiver<bfrt::bfrt::Subscribe> {
        self.subscribe_rx.as_ref().unwrap().resubscribe()
    }

    pub fn digest_rx(&self) -> tokio::sync::broadcast::Receiver<bfrt::bfrt::DigestList> {
        self.digest_rx.as_ref().unwrap().resubscribe()
    }

    pub fn idle_timeout_rx(
        &self,
    ) -> tokio::sync::broadcast::Receiver<bfrt::bfrt::IdleTimeoutNotification> {
        self.idle_timeout_rx.as_ref().unwrap().resubscribe()
    }

    pub fn port_status_change_rx(
        &self,
    ) -> tokio::sync::broadcast::Receiver<bfrt::bfrt::PortStatusChgNotification> {
        self.port_status_change_rx.as_ref().unwrap().resubscribe()
    }

    pub fn set_forwarding_pipeline_config_rx(
        &self,
    ) -> tokio::sync::broadcast::Receiver<bfrt::bfrt::SetForwardingPipelineConfigResponse> {
        self.set_forwarding_pipeline_config_rx
            .as_ref()
            .unwrap()
            .resubscribe()
    }

    pub fn target(&self) -> bfrt::bfrt::TargetDevice {
        bfrt::bfrt::TargetDevice {
            device_id: self.device_id,
            pipe_id: self.pipe_id,
            direction: self.direction,
            prsr_id: self.prsr_id,
        }
    }

    pub fn table(&self) -> Table<&Self> {
        Table::new(self)
    }

    pub fn table_mut(&mut self) -> Table<&mut Self> {
        Table::new(self)
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
        info!("Running BFRuntime client");
        debug!("  device_id: {}", self.device_id);
        debug!("  pipe_id: {}", self.pipe_id);
        debug!("  direction: {}", self.direction);
        debug!("  prsr_id: {}", self.prsr_id);
        debug!("  client_id: {}", self.client_id);

        let (request_sender, request_receiver) =
            tokio::sync::mpsc::channel::<StreamMessageRequest>(self.channel_buffer_size);

        self.stream_message_sender = Some(request_sender);

        self.subscribe(request_receiver).await?;

        let bfrt_info = self.get_bfrt_info().await?;
        self.bfrt_info = Some(bfrt_info);

        debug!("BFRuntime client is running");

        Ok(())
    }

    /// Spawn a listener for stream messages, and send the messages to corresponding channels
    fn set_up_stream_message_listener(
        &mut self,
        mut channel: tonic::codec::Streaming<bfrt::bfrt::StreamMessageResponse>,
        subscribe_tx: tokio::sync::broadcast::Sender<bfrt::bfrt::Subscribe>,
        digest_tx: tokio::sync::broadcast::Sender<bfrt::bfrt::DigestList>,
        idle_timeout_tx: tokio::sync::broadcast::Sender<bfrt::bfrt::IdleTimeoutNotification>,
        port_status_change_tx: tokio::sync::broadcast::Sender<
            bfrt::bfrt::PortStatusChgNotification,
        >,
        set_forwarding_pipeline_config_tx: tokio::sync::broadcast::Sender<
            bfrt::bfrt::SetForwardingPipelineConfigResponse,
        >,
    ) {
        let cancel_token = self.cancel_token.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancel_token.cancelled() => {
                        break;
                    }

                    // TODO: handle error
                    Ok(Some(msg)) = channel.message() => {
                        match msg.update {
                            Some(Update::Subscribe(subscribe)) => {
                                subscribe_tx.send(subscribe).unwrap();
                            }

                            Some(Update::Digest(digest_list)) => {
                                digest_tx.send(digest_list).unwrap();
                            }

                            Some(Update::IdleTimeoutNotification(idle_timeout)) => {
                                idle_timeout_tx.send(idle_timeout).unwrap();
                            }

                            Some(Update::PortStatusChangeNotification(port_status_change)) => {
                                port_status_change_tx.send(port_status_change).unwrap();
                            }

                            Some(Update::SetForwardingPipelineConfigResponse(res)) => {
                                set_forwarding_pipeline_config_tx.send(res).unwrap();
                            }

                            _ => {}
                        }
                    }

                    else => {
                        break;
                    }
                }
            }
        });
    }

    pub async fn get_digest(
        &mut self,
        timeout: u64,
    ) -> Result<bfrt::bfrt::DigestList, ClientBasicError> {
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(timeout)) => {
                Err(ClientBasicError::Timeout)
            }

            msg = self.digest_rx.as_mut().unwrap().recv() => {
                msg.map_err(|e| ClientBasicError::WrappedError {
                    msg: format!("{e}"),
                })
            }
        }
    }

    pub async fn get_idle_timeout(
        &mut self,
        timeout: u64,
    ) -> Result<bfrt::bfrt::IdleTimeoutNotification, ClientBasicError> {
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(timeout)) => {
                Err(ClientBasicError::Timeout)
            }

            msg = self.idle_timeout_rx.as_mut().unwrap().recv() => {
                msg.map_err(|e| ClientBasicError::WrappedError {
                    msg: format!("{e}"),
                })
            }
        }
    }

    pub async fn get_port_status_change(
        &mut self,
        timeout: u64,
    ) -> Result<bfrt::bfrt::PortStatusChgNotification, ClientBasicError> {
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(timeout)) => {
                Err(ClientBasicError::Timeout)
            }

            msg = self.port_status_change_rx.as_mut().unwrap().recv() => {
                msg.map_err(|e| ClientBasicError::WrappedError {
                    msg: format!("{e}"),
                })
            }
        }
    }

    pub async fn get_set_pipeline_config_response(
        &mut self,
        timeout: u64,
    ) -> Result<bfrt::bfrt::SetForwardingPipelineConfigResponse, ClientBasicError> {
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(timeout)) => {
                Err(ClientBasicError::Timeout)
            }

            msg = self.set_forwarding_pipeline_config_rx.as_mut().unwrap().recv() => {
                msg.map_err(|e| ClientBasicError::WrappedError {
                    msg: format!("{e}"),
                })
            }
        }
    }

    pub async fn subscribe(
        &mut self,
        request_receiver: tokio::sync::mpsc::Receiver<StreamMessageRequest>,
    ) -> Result<(), ClientBasicError> {
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
            .ok_or(ClientBasicError::MissingStreamMessageSender)?
            .send(req)
            .await
            .map_err(|e| ClientBasicError::WrappedError {
                msg: format!("{e}"),
            })?;

        // We start a thread to receive the stream messages response
        // Then send the response to the corresponding channel
        let channel = self
            .bfrt_client
            .as_mut()
            .unwrap()
            .stream_channel(tokio_stream::wrappers::ReceiverStream::new(
                request_receiver,
            ))
            .await?
            .into_inner();

        let (subscribe_tx, subscribe_rx) =
            tokio::sync::broadcast::channel::<bfrt::bfrt::Subscribe>(self.channel_buffer_size);
        let (digest_tx, digest_rx) =
            tokio::sync::broadcast::channel::<bfrt::bfrt::DigestList>(self.channel_buffer_size);
        let (idle_timeout_tx, idle_timeout_rx) = tokio::sync::broadcast::channel::<
            bfrt::bfrt::IdleTimeoutNotification,
        >(self.channel_buffer_size);
        let (port_status_change_tx, port_status_change_rx) = tokio::sync::broadcast::channel::<
            bfrt::bfrt::PortStatusChgNotification,
        >(self.channel_buffer_size);
        let (set_forwarding_pipeline_config_tx, set_forwarding_pipeline_config_rx) =
            tokio::sync::broadcast::channel::<bfrt::bfrt::SetForwardingPipelineConfigResponse>(
                self.channel_buffer_size,
            );

        self.subscribe_rx = Some(subscribe_rx);
        self.digest_rx = Some(digest_rx);
        self.idle_timeout_rx = Some(idle_timeout_rx);
        self.port_status_change_rx = Some(port_status_change_rx);
        self.set_forwarding_pipeline_config_rx = Some(set_forwarding_pipeline_config_rx);

        self.set_up_stream_message_listener(
            channel,
            subscribe_tx,
            digest_tx,
            idle_timeout_tx,
            port_status_change_tx,
            set_forwarding_pipeline_config_tx,
        );

        // Check if subscribe is successful
        let msg = self
            .subscribe_rx
            .as_mut()
            .unwrap()
            .recv()
            .await
            .map_err(|e| ClientBasicError::WrappedError {
                msg: format!("{e}"),
            })?;

        if let Some(status) = msg.status {
            if status.code == 0 {
                info!("BFRuntime subscribe success!");
            }
            // TODO: handle error status
        }

        Ok(())
    }

    /// Get the BFRT Info
    pub async fn get_bfrt_info(&mut self) -> Result<BFRTInfo, GetBFRTInfoError> {
        let pipeline_config = self.get_forwarding_pipeline_config().await?;

        let non_p4_config = pipeline_config.non_p4_config.map(|c| c.bfruntime_info);

        let pipeline_config = match self.p4_name.as_ref() {
            Some(p4_name) => pipeline_config
                .config
                .iter()
                .find(|c| c.p4_name == *p4_name),
            None => pipeline_config.config.first(),
        }
        .ok_or(GetBFRTInfoError::ConfigNotFound)?;

        let bfrt_info = BFRTInfo::new(
            pipeline_config.p4_name.clone(),
            &pipeline_config.bfruntime_info,
            non_p4_config,
        )?;

        self.p4_name = Some(pipeline_config.p4_name.clone());

        Ok(bfrt_info)
    }

    /// Bind to a program on the device
    pub async fn bind_pipeline_config(&mut self, p4_name: String) -> Result<(), ClientBasicError> {
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
            .await?;

        Ok(())
    }

    /// Get and set the forwarding pipeline config
    pub async fn get_and_set_pipeline_config(&mut self) -> Result<(), ClientBasicError> {
        let config = self.get_forwarding_pipeline_config().await?;

        self.set_forwarding_pipeline_config(
            bfrt::bfrt::set_forwarding_pipeline_config_request::Action::VerifyAndWarmInitBeginAndEnd, bfrt::bfrt::set_forwarding_pipeline_config_request::DevInitMode::FastReconfig, "".to_string(), config.config).await?;

        if self
            .is_set_pipeline_config_done(
                bfrt::bfrt::SetForwardingPipelineConfigResponseType::WarmInitFinished,
            )
            .await
        {
            Ok(())
        } else {
            Err(ClientBasicError::Timeout)
        }
    }

    pub async fn is_set_pipeline_config_done(
        &mut self,
        check_type: bfrt::bfrt::SetForwardingPipelineConfigResponseType,
    ) -> bool {
        match self.get_set_pipeline_config_response(5).await {
            Ok(res) => res.set_forwarding_pipeline_config_response_type == check_type as i32,
            Err(_) => false,
        }
    }

    pub async fn clear_all_tables(&mut self) -> Result<(), ClientBasicError> {
        self.get_and_set_pipeline_config().await
    }
}

/// Simple wrapper for original methods
impl Client {
    /// Update one or more P4 entities on the target.
    pub async fn write(
        &mut self,
        updates: Vec<bfrt::bfrt::Update>,
    ) -> Result<bfrt::bfrt::WriteResponse, ClientBasicError> {
        let req = bfrt::bfrt::WriteRequest {
            target: Some(bfrt::bfrt::TargetDevice {
                device_id: self.device_id,
                pipe_id: self.pipe_id,
                direction: self.direction,
                prsr_id: self.prsr_id,
            }),
            client_id: self.client_id,
            updates,
            atomicity: bfrt::bfrt::write_request::Atomicity::ContinueOnError as i32,
            p4_name: self.p4_name.clone().unwrap_or_default(),
        };

        let res = self
            .bfrt_client
            .as_mut()
            .ok_or(ClientBasicError::MissingBfrtClient)?
            .write(req)
            .await?
            .into_inner();

        Ok(res)
    }

    pub async fn read(
        &mut self,
        entities: Vec<bfrt::bfrt::Entity>,
    ) -> Result<tonic::codec::Streaming<bfrt::bfrt::ReadResponse>, ClientBasicError> {
        let req = bfrt::bfrt::ReadRequest {
            target: Some(self.target()),
            client_id: self.client_id,
            entities,
            p4_name: self.p4_name.clone().unwrap_or_default(),
        };

        let res = self
            .bfrt_client
            .as_mut()
            .ok_or(ClientBasicError::MissingBfrtClient)?
            .read(req)
            .await?
            .into_inner();

        Ok(res)
    }

    /// Get the forwarding pipeline config
    pub async fn get_forwarding_pipeline_config(
        &mut self,
    ) -> Result<bfrt::bfrt::GetForwardingPipelineConfigResponse, ClientBasicError> {
        trace!("Getting forwarding pipeline config");

        let req = bfrt::bfrt::GetForwardingPipelineConfigRequest {
            device_id: self.device_id,
            client_id: self.client_id,
        };

        let res = self
            .bfrt_client
            .as_mut()
            .ok_or(ClientBasicError::MissingBfrtClient)?
            .get_forwarding_pipeline_config(req)
            .await?
            .into_inner();

        Ok(res)
    }

    /// Set the forwarding pipeline config
    pub async fn set_forwarding_pipeline_config(
        &mut self,
        action: bfrt::bfrt::set_forwarding_pipeline_config_request::Action,
        dev_init_mode: bfrt::bfrt::set_forwarding_pipeline_config_request::DevInitMode,
        base_path: String,
        config: Vec<bfrt::bfrt::ForwardingPipelineConfig>,
    ) -> Result<bfrt::bfrt::SetForwardingPipelineConfigResponse, ClientBasicError> {
        let res = self
            .bfrt_client
            .as_mut()
            .ok_or(ClientBasicError::MissingBfrtClient)?
            .set_forwarding_pipeline_config(bfrt::bfrt::SetForwardingPipelineConfigRequest {
                device_id: self.device_id,
                client_id: self.client_id,
                action: action as i32,
                dev_init_mode: dev_init_mode as i32,
                base_path,
                config,
            })
            .await?
            .into_inner();

        Ok(res)
    }
}
