# \DataSourcesApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive_data_source**](DataSourcesApi.md#archive_data_source) | **DELETE** /containers/{container_id}/import/datasources/{data_source_id} | Archive Data Source
[**create_data_source**](DataSourcesApi.md#create_data_source) | **POST** /containers/{container_id}/import/datasources | Create Data Source
[**create_manual_import**](DataSourcesApi.md#create_manual_import) | **POST** /containers/{container_id}/import/datasources/{data_source_id}/imports | Create Manual Import
[**download_file**](DataSourcesApi.md#download_file) | **GET** /containers/{container_id}/files/{file_id}/download | Download File
[**list_data_sources**](DataSourcesApi.md#list_data_sources) | **GET** /containers/{container_id}/import/datasources | List Data Sources
[**list_imports_for_data_source**](DataSourcesApi.md#list_imports_for_data_source) | **GET** /containers/{container_id}/import/datasources/{data_source_id}/imports | List Imports for Data Source
[**retrieve_data_source**](DataSourcesApi.md#retrieve_data_source) | **GET** /containers/{container_id}/import/datasources/{data_source_id} | Retrieve Data Source
[**retrieve_file**](DataSourcesApi.md#retrieve_file) | **GET** /containers/{container_id}/files/{file_id} | Retrieve File
[**set_data_source_active**](DataSourcesApi.md#set_data_source_active) | **POST** /containers/{container_id}/import/datasources/{data_source_id}/active | Set Data Source Active
[**set_data_source_configuration**](DataSourcesApi.md#set_data_source_configuration) | **PUT** /containers/{container_id}/import/datasources/{data_source_id} | Set Data Source Configuration
[**set_data_source_inactive**](DataSourcesApi.md#set_data_source_inactive) | **DELETE** /containers/{container_id}/import/datasources/{data_source_id}/active | Set Data Source Inactive
[**upload_file**](DataSourcesApi.md#upload_file) | **POST** /containers/{container_id}/import/datasources/{data_source_id}/files | Upload File



## archive_data_source

> crate::models::Generic200Response archive_data_source(container_id, data_source_id, archive, force_delete, remove_data)
Archive Data Source

Archive a data source, with options to permanently remove it (and associated data).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**archive** | Option<**String**> | Set to true to archive the data source. |  |
**force_delete** | Option<**String**> | Set to true to force deletion of the data source. |  |
**remove_data** | Option<**String**> | Set to true to remove data associated with the data source. |  |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_data_source

> crate::models::CreateDataSourcesResponse create_data_source(container_id, body)
Create Data Source

Create new datasource. Supported data source types are `http`, `standard` (or `manual`), `jazz`, `p6`, `aveva`, and `timeseries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**body** | [**CreateDataSourceRequest**](CreateDataSourceRequest.md) |  | [required] |

### Return type

[**crate::models::CreateDataSourcesResponse**](CreateDataSourcesResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_manual_import

> crate::models::CreateManualImportResponse create_manual_import(container_id, data_source_id, body)
Create Manual Import

Create a manual import.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::CreateManualImportResponse**](CreateManualImportResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_file

> download_file(container_id, file_id)
Download File

Downloads a previously uploaded file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**file_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_data_sources

> crate::models::ListDataSourcesResponse list_data_sources(container_id, decrypted)
List Data Sources

List the datasources for the container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**decrypted** | Option<**bool**> | Return decrypted data sources. Requires read-write permissions on data. |  |

### Return type

[**crate::models::ListDataSourcesResponse**](ListDataSourcesResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_imports_for_data_source

> crate::models::ListDataSourceImportsResponse list_imports_for_data_source(container_id, data_source_id)
List Imports for Data Source

List the imports for the datasource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |

### Return type

[**crate::models::ListDataSourceImportsResponse**](ListDataSourceImportsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_data_source

> crate::models::GetDataSourceResponse retrieve_data_source(container_id, data_source_id)
Retrieve Data Source

Retrieve a single data source by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |

### Return type

[**crate::models::GetDataSourceResponse**](GetDataSourceResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_file

> crate::models::GetFileInfoResponse retrieve_file(container_id, file_id)
Retrieve File

Get information about a file by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**file_id** | **String** |  | [required] |

### Return type

[**crate::models::GetFileInfoResponse**](GetFileInfoResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_data_source_active

> crate::models::Generic200Response set_data_source_active(container_id, data_source_id)
Set Data Source Active

Sets a data source active.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_data_source_configuration

> crate::models::UpdateDataSourceResponse set_data_source_configuration(container_id, data_source_id, body)
Set Data Source Configuration

Updates a data source's configuration in storage. Note that this request body's structure must match that of the data source's adapter type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**body** | [**CreateDataSourceConfig**](CreateDataSourceConfig.md) |  | [required] |

### Return type

[**crate::models::UpdateDataSourceResponse**](UpdateDataSourceResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_data_source_inactive

> crate::models::Generic200Response set_data_source_inactive(container_id, data_source_id)
Set Data Source Inactive

Sets a data source inactive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_file

> crate::models::UploadFileResponse upload_file(container_id, data_source_id, import_id, file, metadata)
Upload File

Uploads a file and it's metadata to Deep Lynx. All additional fields on the multipart form will be processed and added as metadata to the file upload itself.  This should be a collection of files and normal fields. If you include a file field and call that \"metadata\" - you can include a normal metadata upload as either a json, csv, or xml file. This data will be processed like a normal import and the files attached to the processed data. Once Deep Lynx generates nodes and edges from that data, any files attached will automatically be attached to the resulting nodes/edges as well. NOTE: The metadata file you upload, if json, must be wrapped in an array. If you do not pass in an array of objects, even if it's a single object, then Deep Lynx will attempt to split up your metadata into its parts instead of treating it like a whole object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**import_id** | Option<**String**> | You can attach the metadata to an existing import if desired. |  |
**file** | Option<**std::path::PathBuf**> |  |  |
**metadata** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**crate::models::UploadFileResponse**](UploadFileResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

