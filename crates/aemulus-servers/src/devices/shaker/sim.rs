use aemulus_proto::{
    sila2::{
        dev::arturgalstyan::shakers::shakercontroller::v1::{
            GetMaxSpeedParameters, GetMaxSpeedResponses, GetMinSpeedParameters,
            GetMinSpeedResponses, LockPlateParameters, LockPlateResponses, StartShakingParameters,
            StartShakingResponses, StopShakingParameters, StopShakingResponses,
            SubscribePlateLockedParameters, SubscribePlateLockedResponses,
            SubscribeSpeedParameters, SubscribeSpeedResponses, UnlockPlateParameters,
            UnlockPlateResponses, shaker_controller_server::ShakerController,
        },
        org::silastandard::{Boolean, Integer, Real},
    },
    sim::v1::{GetJointRequest, SetActuatorRequest, sim_service_client::SimServiceClient},
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status, transport::Channel};

use crate::{devices::shaker::ShakerModel, models::ShakerSpec};

pub(crate) struct ShakerSim {
    pub sim: SimServiceClient<Channel>,
    pub device_name: String,
    pub spec: ShakerSpec,
}

#[tonic::async_trait]
impl ShakerController for ShakerSim {
    type Subscribe_SpeedStream = tonic::codegen::BoxStream<SubscribeSpeedResponses>;
    type Subscribe_PlateLockedStream = tonic::codegen::BoxStream<SubscribePlateLockedResponses>;

    async fn get_min_speed(
        &self,
        _: tonic::Request<GetMinSpeedParameters>,
    ) -> Result<Response<GetMinSpeedResponses>, Status> {
        Ok(Response::new(GetMinSpeedResponses {
            min_speed: Some(Integer {
                value: self.spec.min_rpm,
            }),
        }))
    }

    async fn get_max_speed(
        &self,
        _: tonic::Request<GetMaxSpeedParameters>,
    ) -> Result<Response<GetMaxSpeedResponses>, Status> {
        Ok(Response::new(GetMaxSpeedResponses {
            max_speed: Some(Integer {
                value: self.spec.max_rpm,
            }),
        }))
    }

    async fn start_shaking(
        &self,
        request: tonic::Request<StartShakingParameters>,
    ) -> Result<Response<StartShakingResponses>, Status> {
        let Some(target_rpm) = request.into_inner().speed else {
            return Err(Status::invalid_argument("speed is required"));
        };

        if !(self.spec.min_rpm..=self.spec.max_rpm).contains(&target_rpm.value) {
            return Err(Status::invalid_argument(
                "speed must be between 200 and 3000 rpm",
            ));
        }

        let mut sim = self.sim.clone();

        let set_actuator_request = SetActuatorRequest {
            actuator: format!("{}/shake", self.device_name),
            value: target_rpm.value as f64 * 2.0 * std::f64::consts::PI / 60.0,
        };
        sim.set_actuator(set_actuator_request).await?;

        let get_joint_request = GetJointRequest {
            joint: format!("{}/shake", self.device_name),
        };
        loop {
            let target = target_rpm.value as f64 * 2.0 * std::f64::consts::PI / 60.0;

            let joint_response = sim.get_joint(get_joint_request.clone()).await?;
            let current_vel = joint_response.into_inner().velocity;

            if (current_vel - target).abs() < 0.1 {
                break;
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        Ok(Response::new(StartShakingResponses {}))
    }

    async fn stop_shaking(
        &self,
        _request: tonic::Request<StopShakingParameters>,
    ) -> Result<Response<StopShakingResponses>, Status> {
        let mut sim = self.sim.clone();

        let set_actuator_request = SetActuatorRequest {
            actuator: format!("{}/shake", self.device_name),
            value: 0.0,
        };
        sim.set_actuator(set_actuator_request).await?;

        Ok(Response::new(StopShakingResponses {}))
    }

    async fn lock_plate(
        &self,
        _request: tonic::Request<LockPlateParameters>,
    ) -> Result<Response<LockPlateResponses>, Status> {
        let mut sim = self.sim.clone();

        let set_actuator_request = SetActuatorRequest {
            actuator: format!("{}/elm", self.device_name),
            value: 0.002,
        };
        sim.set_actuator(set_actuator_request).await?;

        Ok(Response::new(LockPlateResponses {}))
    }

    async fn unlock_plate(
        &self,
        _request: tonic::Request<UnlockPlateParameters>,
    ) -> Result<Response<UnlockPlateResponses>, Status> {
        let mut sim = self.sim.clone();

        let set_actuator_request = SetActuatorRequest {
            actuator: format!("{}/elm", self.device_name),
            value: 0.0,
        };
        sim.set_actuator(set_actuator_request).await?;

        Ok(Response::new(UnlockPlateResponses {}))
    }

    async fn subscribe_speed(
        &self,
        _request: tonic::Request<SubscribeSpeedParameters>,
    ) -> Result<tonic::Response<Self::Subscribe_SpeedStream>, tonic::Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(16);
        let mut sim = self.sim.clone();
        let device_name = self.device_name.clone();

        tokio::spawn(async move {
            loop {
                let get_joint_request = GetJointRequest {
                    joint: format!("{}/shake", device_name),
                };

                let response = match sim.get_joint(get_joint_request).await {
                    Ok(response) => response,
                    Err(status) => {
                        let _ = tx.send(Err(status)).await;
                        break;
                    }
                };
                let joint_response = response.into_inner();

                let message = SubscribeSpeedResponses {
                    speed: Some(Real {
                        value: joint_response.velocity * 60.0 / (2.0 * std::f64::consts::PI),
                    }),
                };

                if tx.send(Ok(message)).await.is_err() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn subscribe_plate_locked(
        &self,
        _request: tonic::Request<SubscribePlateLockedParameters>,
    ) -> Result<tonic::Response<Self::Subscribe_PlateLockedStream>, tonic::Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(16);
        let mut sim = self.sim.clone();
        let device_name = self.device_name.clone();

        tokio::spawn(async move {
            loop {
                let get_joint_request = GetJointRequest {
                    joint: format!("{}/elm", device_name),
                };

                let response = match sim.get_joint(get_joint_request).await {
                    Ok(response) => response,
                    Err(status) => {
                        let _ = tx.send(Err(status)).await;
                        break;
                    }
                };
                let joint_response = response.into_inner();

                let locked = joint_response.position > 0.0014;

                let message = SubscribePlateLockedResponses {
                    plate_locked: Some(Boolean { value: locked }),
                };

                if tx.send(Ok(message)).await.is_err() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }
}
