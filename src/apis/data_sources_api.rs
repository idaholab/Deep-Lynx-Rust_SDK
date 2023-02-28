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


/// struct for typed errors of method [`archive_data_source`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArchiveDataSourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_data_source`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDataSourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_manual_import`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateManualImportError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`download_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadFileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_data_sources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDataSourcesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_imports_for_data_source`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListImportsForDataSourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_data_source`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveDataSourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveFileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_data_source_active`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetDataSourceActiveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_data_source_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetDataSourceConfigurationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_data_source_inactive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetDataSourceInactiveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadFileError {
    UnknownValue(serde_json::Value),
}


/// Archive a data source, with options to permanently remove it (and associated data).
pub fn archive_data_source(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str, archive: Option<&str>, force_delete: Option<&str>, remove_data: Option<&str>) -> Result<crate::models::Generic200Response, Error<ArchiveDataSourceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = archive {
        local_var_req_builder = local_var_req_builder.query(&[("archive", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = force_delete {
        local_var_req_builder = local_var_req_builder.query(&[("forceDelete", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = remove_data {
        local_var_req_builder = local_var_req_builder.query(&[("removeData", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ArchiveDataSourceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create new datasource. Supported data source types are `http`, `standard` (or `manual`), `jazz`, `p6`, `aveva`, and `timeseries`.
pub fn create_data_source(configuration: &configuration::Configuration, container_id: &str, body: crate::models::CreateDataSourceRequest) -> Result<crate::models::CreateDataSourcesResponse, Error<CreateDataSourceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id));
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
        let local_var_entity: Option<CreateDataSourceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a manual import.
pub fn create_manual_import(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str, body: serde_json::Value) -> Result<crate::models::CreateManualImportResponse, Error<CreateManualImportError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/imports", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id));
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
        let local_var_entity: Option<CreateManualImportError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Downloads a previously uploaded file.
pub fn download_file(configuration: &configuration::Configuration, container_id: &str, file_id: &str) -> Result<(), Error<DownloadFileError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/files/{file_id}/download", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), file_id=crate::apis::urlencode(file_id));
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
        Ok(())
    } else {
        let local_var_entity: Option<DownloadFileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List the datasources for the container.
pub fn list_data_sources(configuration: &configuration::Configuration, container_id: &str, decrypted: Option<bool>) -> Result<crate::models::ListDataSourcesResponse, Error<ListDataSourcesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = decrypted {
        local_var_req_builder = local_var_req_builder.query(&[("decrypted", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListDataSourcesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List the imports for the datasource.
pub fn list_imports_for_data_source(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str) -> Result<crate::models::ListDataSourceImportsResponse, Error<ListImportsForDataSourceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/imports", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id));
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
        let local_var_entity: Option<ListImportsForDataSourceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a single data source by ID.
pub fn retrieve_data_source(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str) -> Result<crate::models::GetDataSourceResponse, Error<RetrieveDataSourceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id));
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
        let local_var_entity: Option<RetrieveDataSourceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get information about a file by ID.
pub fn retrieve_file(configuration: &configuration::Configuration, container_id: &str, file_id: &str) -> Result<crate::models::GetFileInfoResponse, Error<RetrieveFileError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/files/{file_id}", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), file_id=crate::apis::urlencode(file_id));
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
        let local_var_entity: Option<RetrieveFileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sets a data source active.
pub fn set_data_source_active(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str) -> Result<crate::models::Generic200Response, Error<SetDataSourceActiveError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/active", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id));
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetDataSourceActiveError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a data source's configuration in storage. Note that this request body's structure must match that of the data source's adapter type.
pub fn set_data_source_configuration(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str, body: crate::models::CreateDataSourceConfig) -> Result<crate::models::UpdateDataSourceResponse, Error<SetDataSourceConfigurationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id));
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
        let local_var_entity: Option<SetDataSourceConfigurationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sets a data source inactive.
pub fn set_data_source_inactive(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str) -> Result<crate::models::Generic200Response, Error<SetDataSourceInactiveError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/active", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id));
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
        let local_var_entity: Option<SetDataSourceInactiveError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Uploads a file and it's metadata to Deep Lynx. All additional fields on the multipart form will be processed and added as metadata to the file upload itself.  This should be a collection of files and normal fields. If you include a file field and call that \"metadata\" - you can include a normal metadata upload as either a json, csv, or xml file. This data will be processed like a normal import and the files attached to the processed data. Once Deep Lynx generates nodes and edges from that data, any files attached will automatically be attached to the resulting nodes/edges as well. NOTE: The metadata file you upload, if json, must be wrapped in an array. If you do not pass in an array of objects, even if it's a single object, then Deep Lynx will attempt to split up your metadata into its parts instead of treating it like a whole object.
pub fn upload_file(configuration: &configuration::Configuration, container_id: &str, data_source_id: &str, import_id: Option<&str>, file: Option<std::path::PathBuf>, metadata: Option<std::path::PathBuf>) -> Result<crate::models::UploadFileResponse, Error<UploadFileError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/containers/{container_id}/import/datasources/{data_source_id}/files", local_var_configuration.base_path, container_id=crate::apis::urlencode(container_id), data_source_id=crate::apis::urlencode(data_source_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = import_id {
        local_var_req_builder = local_var_req_builder.query(&[("importID", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    if let Some(local_var_param_value) = file {
        local_var_form = local_var_form.file("file", local_var_param_value)?;
    }
    if let Some(local_var_param_value) = metadata {
        local_var_form = local_var_form.file("metadata", local_var_param_value)?;
    }
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UploadFileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

