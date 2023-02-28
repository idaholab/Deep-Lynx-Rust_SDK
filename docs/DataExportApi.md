# \DataExportApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_data_export**](DataExportApi.md#create_data_export) | **POST** /containers/{container_id}/data/export | Create Data Export
[**delete_data_export**](DataExportApi.md#delete_data_export) | **DELETE** /containers/{container_id}/data/export/{export_id} | Delete Data Export
[**list_data_exports**](DataExportApi.md#list_data_exports) | **GET** /containers/{container_id}/data/export | List Data Exports
[**retrieve_data_export**](DataExportApi.md#retrieve_data_export) | **GET** /containers/{container_id}/data/export/{export_id} | Retrieve Data Export
[**start_data_export**](DataExportApi.md#start_data_export) | **POST** /containers/{container_id}/data/export/{export_id} | Start Data Export
[**stop_data_export**](DataExportApi.md#stop_data_export) | **PUT** /containers/{container_id}/data/export/{export_id} | Stop Data Export



## create_data_export

> crate::models::Generic200Response create_data_export(container_id, body)
Create Data Export

Create a new data export with the included configuration. Configuration values may be encrypted depending on the adapter you've choosen. See the readme for the exporters for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**body** | [**CreateDataExportRequest**](CreateDataExportRequest.md) |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_data_export

> crate::models::Generic200Response delete_data_export(container_id, export_id)
Delete Data Export

Deletes a data export record. This does not guarantee the export will stop immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**export_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_data_exports

> crate::models::ListDataExportsResponse list_data_exports(container_id, count, limit, offset, sort_by, sort_desc)
List Data Exports

List data exports for the container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**count** | Option<**bool**> | boolean indicating if the return value should be a count only |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**sort_by** | Option<**String**> | column to sort results by |  |
**sort_desc** | Option<**bool**> | boolean indicating if results should be in descending order |  |

### Return type

[**crate::models::ListDataExportsResponse**](ListDataExportsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_data_export

> crate::models::GetDataExportResponse retrieve_data_export(container_id, export_id)
Retrieve Data Export

Fetch a data export record by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**export_id** | **String** |  | [required] |

### Return type

[**crate::models::GetDataExportResponse**](GetDataExportResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_data_export

> crate::models::Generic200Response start_data_export(container_id, export_id)
Start Data Export

Start or restart a data export by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**export_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_data_export

> crate::models::Generic200Response stop_data_export(container_id, export_id)
Stop Data Export

Stops a data export. Please note that this just sends a **stop** signal. The application's export adapter determines how to handle the said signal. In some cases the export stopping might not be immediate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**export_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

