{% if persistence != 'None' %}use tonic::{Request, Response, Status};
use tracing::info;

use {{ prefix_name }}_{{ suffix_name }}_persistence::Page;

use crate::{{ PrefixName }}{{ SuffixName }}Core;
use crate::conversion::{ConvertFrom, TryConvertTo};
use crate::proto::{{'{'}}{{ PrefixName }}, Get{{ PrefixName }}Request, Get{{ PrefixName }}sRequest, Get{{ PrefixName }}sResponse};
use crate::proto::{{ prefix_name }}_{{ suffix_name }}_server::{{ PrefixName }}{{ SuffixName }};

#[tonic::async_trait]
impl {{ PrefixName }}{{ SuffixName }} for {{ PrefixName }}{{ SuffixName }}Core {

    async fn create_{{ prefix_name }}(&self, request: Request<{{ PrefixName }}>) -> Result<Response<{{ PrefixName }}>, Status> {
        let {{ prefix_name }} = request.into_inner();
        info!("Creating: {:?}", {{ prefix_name }});

        self.persistence.insert_{{ prefix_name }}({{ prefix_name }}.try_convert_to()?)
            .await
            .map({{ PrefixName }}::convert_from)
            .map(Response::new)
            .map_err(|err| Status::internal(format!("{err}")))
    }

    async fn get_{{ prefix_name }}(&self, request: Request<Get{{ PrefixName }}Request>) -> Result<Response<{{ PrefixName }}>, Status> {
        let request = request.into_inner();
        info!("Getting {{ PrefixName }}: {:?}", request);
        let id = request.id.try_convert_to()?;

        self.persistence.find_{{ prefix_name }}(id)
            .await
            .map_err(|err| {
                match err {
                    err => Status::internal(format!("Error: '{err}'")),
                }
            })?
            .ok_or(Status::not_found(format!("{{ PrefixName }} not found by id '{id}'")))
            .map({{ PrefixName }}::convert_from)
            .map(Response::new)
    }

    async fn get_{{ prefix_name }}s(&self, request: Request<Get{{ PrefixName }}sRequest>) -> Result<Response<Get{{ PrefixName }}sResponse>, Status> {
        let request = request.into_inner();
        info!("Getting {{ PrefixName }}s: {:?}", request);

        let response = self
            .persistence
            .get_{{ prefix_name }}_list(request.page_index, request.page_size)
            .await;

        match response {
            Ok(Page { records, index, next, has_next, previous, has_previous, total, total_records }) => {
                let records = records.into_iter().map({{ PrefixName }}::convert_from).collect();
                Ok(Response::new(Get{{ PrefixName }}sResponse {
                    records,
                    index,
                    next,
                    has_next,
                    previous,
                    has_previous,
                    total,
                    total_records,
                }))
            }
            Err(err) => Err(Status::internal(format!("Database Error: '{err}'"))),
        }
    }

    async fn update_{{ prefix_name }}(&self, request: Request<{{ PrefixName }}>) -> Result<Response<{{ PrefixName }}>, Status> {
        let {{ prefix_name }} = request.into_inner();
        info!("Updating: {:?}", {{ prefix_name }});

        if {{ prefix_name }}.id.is_none() {
            return Err(Status::invalid_argument("{{ prefix_name }} id is required"));
        }

        self.persistence.update_{{ prefix_name }}({{ prefix_name }}.try_convert_to()?)
            .await
            .map({{ PrefixName }}::convert_from)
            .map(Response::new)
            .map_err(|err| Status::internal(format!("{err}")))
    }
}
{% else %}use tonic::{Request, Response, Status};
use tracing::info;

use crate::{{ PrefixName }}{{ SuffixName }}Core;
use crate::proto::{{'{'}}{{ PrefixName }}, Get{{ PrefixName }}Request, Get{{ PrefixName }}sRequest, Get{{ PrefixName }}sResponse};
use crate::proto::{{ prefix_name }}_{{ suffix_name }}_server::{{ PrefixName }}{{ SuffixName }};

#[tonic::async_trait]
impl {{ PrefixName }}{{ SuffixName }} for {{ PrefixName }}{{ SuffixName }}Core {

    async fn create_{{ prefix_name }}(&self, request: Request<{{ PrefixName }}>) -> Result<Response<{{ PrefixName }}>, Status> {
        let {{ prefix_name }} = request.into_inner();
        info!("Creating: {:?}", {{ prefix_name }});
        Err(Status::unimplemented("create_{{ prefix_name }} not implemented"))
    }

    async fn get_{{ prefix_name }}(&self, request: Request<Get{{ PrefixName }}Request>) -> Result<Response<{{ PrefixName }}>, Status> {
        let request = request.into_inner();
        info!("Getting {{ PrefixName }}: {:?}", request);
        Err(Status::unimplemented("get_{{ prefix_name }} not implemented"))
    }

    async fn get_{{ prefix_name }}s(&self, request: Request<Get{{ PrefixName }}sRequest>) -> Result<Response<Get{{ PrefixName }}sResponse>, Status> {
        let request = request.into_inner();
        info!("Getting {{ PrefixName }}s: {:?}", request);
        Err(Status::unimplemented("get_{{ prefix_name }}s not implemented"))
    }

    async fn update_{{ prefix_name }}(&self, request: Request<{{ PrefixName }}>) -> Result<Response<{{ PrefixName }}>, Status> {
        let {{ prefix_name }} = request.into_inner();
        info!("Updating: {:?}", {{ prefix_name }});
        Err(Status::unimplemented("update_{{ prefix_name }} not implemented"))
    }
}
{% endif %}