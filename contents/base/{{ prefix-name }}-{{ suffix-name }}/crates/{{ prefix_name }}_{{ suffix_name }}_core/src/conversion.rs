{% if persistence != 'None' %}use anyhow::Result;
use {{ prefix_name }}_{{ suffix_name }}_persistence::{entities::*, sea_orm::prelude::Uuid, sea_orm::ActiveValue};
use std::str::FromStr;
use tonic::Status;

use crate::proto::*;

pub trait ConvertTo<T>: Sized {
    fn convert_to(self) -> T;
}

pub trait TryConvertTo<T, E>: Sized {
    fn try_convert_to(self) -> Result<T, E>;
}

pub trait ConvertFrom<T>: Sized {
    fn convert_from(value: T) -> Self;
}

pub trait TryConvertFrom<T: Sized, E> {
    fn try_convert_from(value: T) -> Result<T, E>;
}

impl ConvertFrom<{{ prefix_name }}::Model> for {{ PrefixName }} {
    fn convert_from(value: {{ prefix_name }}::Model) -> Self {
        {{ PrefixName }} {
            id: Some(value.id.to_string()),
            contents : value.contents,
        }
    }
}

impl TryConvertTo<{{ prefix_name }}::ActiveModel, Status> for {{ PrefixName }} {
    fn try_convert_to(self) -> std::result::Result<{{ prefix_name }}::ActiveModel, Status> {
        let id = self.id.try_convert_to()?;
        Ok({{ prefix_name }}::ActiveModel {
            id: id.map(|id| ActiveValue::Set(id)).unwrap_or( ActiveValue::NotSet),
            contents: ActiveValue::Set(self.contents),
        })
    }
}

impl TryConvertTo<Option<Uuid>, Status> for Option<String> {
    fn try_convert_to(self) -> Result<Option<Uuid>, Status> {
        match self {
            None => Ok(None),
            Some(id) => Ok(Some(id.try_convert_to()?)),
        }
    }
}

impl TryConvertTo<Uuid, Status> for String {
    fn try_convert_to(self) -> Result<Uuid, Status> {
        Uuid::from_str(self.as_str())
            .map_err(|_| Status::invalid_argument("Id was not set to a valid UUID".to_string()))
    }
}
{% endif %}