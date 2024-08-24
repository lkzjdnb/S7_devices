use std::collections::HashMap;

use crate::errors::S7Error;
use crate::s7_connexion::S7Connexion;
use crate::types::RegisterValue;
use crate::S7Device;

use industrial_device::errors::IndustrialDeviceError;
use industrial_device::types::Value;
use industrial_device::IndustrialDevice;

impl IndustrialDevice for S7Device {
    async fn connect(&mut self) -> Result<(), IndustrialDeviceError> {
        S7Connexion::connect(self).await?;
        Ok(())
    }

    async fn dump_registers(&mut self) -> Result<HashMap<String, Value>, IndustrialDeviceError> {
        let vals = S7Connexion::dump_registers(self).await?;
        Ok(vals
            .iter()
            .map(|(name, val)| (name.clone(), Into::<Value>::into(*val)))
            .collect())
    }
}

impl From<S7Error> for IndustrialDeviceError {
    fn from(value: S7Error) -> Self {
        match value {
            S7Error::S7ClientError { err } => {
                IndustrialDeviceError::RequestError { err: Box::new(err) }
            }
            S7Error::DeviceNotConnectedError => IndustrialDeviceError::DeviceNotConnectedError {
                err: Box::new(value),
            },
            S7Error::MismatchedRegisterLengthError => IndustrialDeviceError::RequestError {
                err: Box::new(value),
            },
            S7Error::RegisterDoesNotExistsError => IndustrialDeviceError::RequestError {
                err: Box::new(value),
            },
            S7Error::InvalidRegisterValue => IndustrialDeviceError::RequestError {
                err: Box::new(value),
            },
        }
    }
}

impl From<RegisterValue> for Value {
    fn from(value: RegisterValue) -> Self {
        match value {
            RegisterValue::S16(val) => Value::S16(val),
            RegisterValue::S32(val) => Value::S32(val),
            RegisterValue::Float32(val) => Value::Float32(val),
            RegisterValue::Boolean(val) => Value::Boolean(val),
        }
    }
}
