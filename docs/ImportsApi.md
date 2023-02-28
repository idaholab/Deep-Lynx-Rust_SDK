# \ImportsApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_data_to_import**](ImportsApi.md#add_data_to_import) | **POST** /containers/{container_id}/datasources/{data_source_id}/imports/{import_id}/data | Add Data to Import
[**create_import**](ImportsApi.md#create_import) | **POST** /containers/{container_id}/datasources/{data_source_id}/imports | Create Import
[**delete_file**](ImportsApi.md#delete_file) | **DELETE** /containers/:container_id/import/datasources/:data_source_id/files/:file_id | Delete File
[**delete_import**](ImportsApi.md#delete_import) | **DELETE** /containers/{container_id}/import/imports/{import_id} | Delete Import
[**delete_import_data**](ImportsApi.md#delete_import_data) | **DELETE** /containers/{container_id}/import/imports/{import_id}/data/{data_id} | Delete Import Data
[**list_imports_data**](ImportsApi.md#list_imports_data) | **GET** /containers/{container_id}/import/imports/{import_id}/data | List Import's Data
[**put_containers_container_id_import_datasources_datasource_id_files_file_id**](ImportsApi.md#put_containers_container_id_import_datasources_datasource_id_files_file_id) | **PUT** /containers/:container_id/import/datasources/:data_source_id/files/:file_id | Update files
[**retrieve_import_data**](ImportsApi.md#retrieve_import_data) | **GET** /containers/{container_id}/import/imports/{import_id}/data/{data_id} | Retrieve Import Data
[**update_import_data**](ImportsApi.md#update_import_data) | **PUT** /containers/{container_id}/import/imports/{import_id}/data/{data_id} | Update Import Data



## add_data_to_import

> crate::models::AddDataToImportResponse add_data_to_import(container_id, import_id, data_source_id, request_body)
Add Data to Import

Adds data to an existing import. Accepts an array of JSON objects or a file in JSON or CSV format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**import_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**request_body** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  |  |

### Return type

[**crate::models::AddDataToImportResponse**](AddDataToImportResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_import

> crate::models::CreateImportResponse create_import(container_id, data_source_id, create_import_request)
Create Import

Creates a new import.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**create_import_request** | Option<[**CreateImportRequest**](CreateImportRequest.md)> |  |  |

### Return type

[**crate::models::CreateImportResponse**](CreateImportResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_file

> delete_file()
Delete File

Delete a file

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_import

> delete_import(container_id, import_id)
Delete Import

Delete import will delete an import ONLY IF the import has not been processed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**import_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_import_data

> crate::models::Generic200Response delete_import_data(container_id, import_id, data_id)
Delete Import Data

Delete a single piece of data from an import.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**import_id** | **String** |  | [required] |
**data_id** | **i32** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_imports_data

> crate::models::ListImportDataResponse list_imports_data(container_id, import_id, count, limit, offset, sort_by, sort_desc)
List Import's Data

List the data for an import.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**import_id** | **String** |  | [required] |
**count** | Option<**String**> | boolean indicating if the return value should be a count only |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**sort_by** | Option<**String**> | column to sort results by |  |
**sort_desc** | Option<**bool**> | boolean indicating if results should be in descending order |  |

### Return type

[**crate::models::ListImportDataResponse**](ListImportDataResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_containers_container_id_import_datasources_datasource_id_files_file_id

> put_containers_container_id_import_datasources_datasource_id_files_file_id(authorization, put_containers_container_id_import_datasources_datasource_id_files_file_id_request)
Update files

Update a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | Option<**String**> | Bearer token |  |
**put_containers_container_id_import_datasources_datasource_id_files_file_id_request** | Option<[**PutContainersContainerIdImportDatasourcesDatasourceIdFilesFileIdRequest**](PutContainersContainerIdImportDatasourcesDatasourceIdFilesFileIdRequest.md)> | Key/value pair where the key is named 'file' and the file is any file.  Optionally, any metadata can be added |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data, application/xml
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_import_data

> crate::models::GetImportDataResponse retrieve_import_data(container_id, import_id, data_id)
Retrieve Import Data

Retrieve a single piece of data from an import.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**import_id** | **String** |  | [required] |
**data_id** | **i32** |  | [required] |

### Return type

[**crate::models::GetImportDataResponse**](GetImportDataResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_import_data

> crate::models::UpdateImportDataResponse update_import_data(container_id, import_id, data_id, data_staging)
Update Import Data

Update the data of an existing import.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**import_id** | **String** |  | [required] |
**data_id** | **i32** |  | [required] |
**data_staging** | Option<[**DataStaging**](DataStaging.md)> |  |  |

### Return type

[**crate::models::UpdateImportDataResponse**](UpdateImportDataResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

