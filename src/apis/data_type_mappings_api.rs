/*
 * Deep Lynx
 *
 * The construction of megaprojects has consistently demonstrated challenges for project managers in regard to meeting cost, schedule, and performance requirements. Megaproject construction challenges are common place within megaprojects with many active projects in the United States failing to meet cost and schedule efforts by significant margins. Currently, engineering teams operate in siloed tools and disparate teams where connections across design, procurement, and construction systems are translated manually or over brittle point-to-point integrations. The manual nature of data exchange increases the risk of silent errors in the reactor design, with each silent error cascading across the design. These cascading errors lead to uncontrollable risk during construction, resulting in significant delays and cost overruns. Deep Lynx allows for an integrated platform during design and operations of mega projects. The Deep Lynx Core API delivers a few main features. 1. Provides a set of methods and endpoints for manipulating data in an object oriented database. This allows us to store complex datatypes as records and then to compile them into actual, modifiable objects at run-time. Users can store taxonomies or ontologies in a readable format. 2. Provides methods for storing and retrieving data in a graph database. This data is structured and validated against the aformentioned object oriented database before storage.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`copy_transformations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopyTransformationsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_transformation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTransformationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_data_type_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDataTypeMappingError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_transformation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTransformationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`export_type_mappings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExportTypeMappingsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`import_data_type_mappings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImportDataTypeMappingsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_data_type_mappings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDataTypeMappingsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_transformations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTransformationsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_data_type_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveDataTypeMappingError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_data_type_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDataTypeMappingError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_transformation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTransformationError {
    UnknownValue(serde_json::Value),
}


/// This endpoint copies transformations from the {originalMappingID} type mapping (final parameter) to the {mappingID} type mapping. This POST has NO body.
pub fn copy_transformations(configuration: &configuration::Configuration, container_id: &str, source_id: &str, mapping_id: &str, original_mapping_id: &str) -> Result<(), Error<CopyTransformationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{sourceID}/mappings/{mappingID}/copy/{originalMappingID}", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), sourceID=crate::apis::urlencode(source_id), mappingID=crate::apis::urlencode(mapping_id), originalMappingID=crate::apis::urlencode(original_mapping_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CopyTransformationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a transformation for the type mapping.
pub fn create_transformation(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str, mapping_id: &str, body: crate::models::CreateTypeMappingTransformationsRequest) -> Result<crate::models::CreateTransformationResponse, Error<CreateTransformationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/mappings/{mapping_id}/transformations", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id), mapping_id=crate::apis::urlencode(mapping_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateTransformationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Permanently remove data type mapping.
pub fn delete_data_type_mapping(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str, mapping_id: &str) -> Result<crate::models::Generic200Response, Error<DeleteDataTypeMappingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/mappings/{mapping_id}", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id), mapping_id=crate::apis::urlencode(mapping_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteDataTypeMappingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a transformation.
pub fn delete_transformation(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str, mapping_id: &str, transformation_id: &str) -> Result<crate::models::Generic200Response, Error<DeleteTransformationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/mappings/{mapping_id}/transformations/{transformation_id}", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id), mapping_id=crate::apis::urlencode(mapping_id), transformation_id=crate::apis::urlencode(transformation_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteTransformationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Export type mappings for a datasource. Providing a JSON body is optional. If provided, the mapping_ids may be specified to indicate certain type mapping IDs to return. Additionally, a target data source may be provided to which the mappings will be copied.
pub fn export_type_mappings(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str, type_mapping_export_payload: Option<crate::models::TypeMappingExportPayload>) -> Result<Vec<crate::models::TypeMapping>, Error<ExportTypeMappingsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/mappings/export", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&type_mapping_export_payload);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ExportTypeMappingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Import type mappings for a datasource. Accepts either a JSON body or actual JSON file. The payload should be an array of type mapping classes, previously generated using the export route.
pub fn import_data_type_mappings(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str, is_enabled: Option<bool>, import_data_type_mappings_request: Option<crate::models::ImportDataTypeMappingsRequest>) -> Result<Vec<crate::models::ImportDataTypeMappingResponseInner>, Error<ImportDataTypeMappingsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/mappings/import", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = is_enabled {
        local_var_req_builder = local_var_req_builder.query(&[("isEnabled", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&import_data_type_mappings_request);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ImportDataTypeMappingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists data type mappings for the data source
pub fn list_data_type_mappings(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str, limit: Option<i32>, offset: Option<i32>, needs_transformations: Option<bool>, count: Option<bool>, sort_by: Option<&str>, sort_desc: Option<bool>, resulting_metatype_name: Option<&str>, resulting_metatype_relationship_name: Option<&str>) -> Result<crate::models::ListDataTypeMappingResponse, Error<ListDataTypeMappingsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/mappings", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = needs_transformations {
        local_var_req_builder = local_var_req_builder.query(&[("needsTransformations", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_by {
        local_var_req_builder = local_var_req_builder.query(&[("sortBy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_desc {
        local_var_req_builder = local_var_req_builder.query(&[("sortDesc", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = resulting_metatype_name {
        local_var_req_builder = local_var_req_builder.query(&[("resultingMetatypeName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = resulting_metatype_relationship_name {
        local_var_req_builder = local_var_req_builder.query(&[("resultingMetatypeRelationshipName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListDataTypeMappingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List transformations for a type mapping from storage.
pub fn list_transformations(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str, mapping_id: &str) -> Result<crate::models::ListTransformationResponse, Error<ListTransformationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/mappings/{mapping_id}/transformations", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id), mapping_id=crate::apis::urlencode(mapping_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListTransformationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a data type mapping
pub fn retrieve_data_type_mapping(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str, mapping_id: &str) -> Result<crate::models::GetDataTypeMappingResponse, Error<RetrieveDataTypeMappingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/mappings/{mapping_id}", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id), mapping_id=crate::apis::urlencode(mapping_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RetrieveDataTypeMappingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a data type mapping.
pub fn update_data_type_mapping(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str, mapping_id: &str, type_mapping: Option<crate::models::TypeMapping>) -> Result<crate::models::UpdateDataTypeMappingResponse, Error<UpdateDataTypeMappingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/mappings/{mapping_id}", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id), mapping_id=crate::apis::urlencode(mapping_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&type_mapping);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateDataTypeMappingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a transformation.
pub fn update_transformation(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str, mapping_id: &str, transformation_id: &str, body: crate::models::CreateTypeMappingTransformationsRequest) -> Result<crate::models::UpdateTransformationResponse, Error<UpdateTransformationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/mappings/{mapping_id}/transformations/{transformation_id}", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id), mapping_id=crate::apis::urlencode(mapping_id), transformation_id=crate::apis::urlencode(transformation_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateTransformationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

