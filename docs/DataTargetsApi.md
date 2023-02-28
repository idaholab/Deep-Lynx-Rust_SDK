# \DataTargetsApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive_data_target**](DataTargetsApi.md#archive_data_target) | **DELETE** /containers/{container_id}/export/datatargets/{data_target_id} | Archive Data Target
[**create_data_target**](DataTargetsApi.md#create_data_target) | **POST** /containers/{container_id}/export/datatargets | Create Data Target
[**list_dat_targets**](DataTargetsApi.md#list_dat_targets) | **GET** /containers/{container_id}/export/datatargets | List Data Targets
[**retrieve_data_target**](DataTargetsApi.md#retrieve_data_target) | **GET** /containers/{container_id}/export/datatargets/{data_target_id} | Retrieve Data Target
[**set_data_target_active**](DataTargetsApi.md#set_data_target_active) | **POST** /containers/{container_id}/export/datatargets/{data_target_id}/active | Set Data Target Active
[**set_data_target_configuration**](DataTargetsApi.md#set_data_target_configuration) | **PUT** /containers/{container_id}/export/datatargets/{data_target_id} | Set Data Target Configuration
[**set_data_target_inactive**](DataTargetsApi.md#set_data_target_inactive) | **DELETE** /containers/{container_id}/export/datatargets/{data_target_id}/active | Set Data Target Inactive



## archive_data_target

> crate::models::Generic200Response archive_data_target(container_id, data_target_id, archive, force_delete)
Archive Data Target

Archive a data target, with options to permanently remove it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_target_id** | **String** |  | [required] |
**archive** | Option<**String**> | Set to true to archive the data target. |  |
**force_delete** | Option<**String**> | Set to true to force deletion of the data target. |  |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_data_target

> crate::models::CreateDataTargetsResponse create_data_target(container_id, body)
Create Data Target

Create new datatarget. Supported data target types are `http` and `standard`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**body** | Option<[**CreateDataTargetRequest**](CreateDataTargetRequest.md)> |  |  |

### Return type

[**crate::models::CreateDataTargetsResponse**](CreateDataTargetsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dat_targets

> crate::models::ListDataTargetsResponse list_dat_targets(container_id)
List Data Targets

List the datatargets for the container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |

### Return type

[**crate::models::ListDataTargetsResponse**](ListDataTargetsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_data_target

> crate::models::GetDataTargetResponse retrieve_data_target(container_id, data_target_id)
Retrieve Data Target

Retrieve a single data target by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_target_id** | **String** |  | [required] |

### Return type

[**crate::models::GetDataTargetResponse**](GetDataTargetResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_data_target_active

> crate::models::Generic200Response set_data_target_active(container_id, data_target_id)
Set Data Target Active

Sets a data target active.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_target_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_data_target_configuration

> crate::models::UpdateDataTargetResponse set_data_target_configuration(container_id, data_target_id, body)
Set Data Target Configuration

Updates a data target's configuration in storage. Note that this request body's structure must match that of the data target's adapter type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_target_id** | **String** |  | [required] |
**body** | [**CreateDataTargetConfig**](CreateDataTargetConfig.md) |  | [required] |

### Return type

[**crate::models::UpdateDataTargetResponse**](UpdateDataTargetResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_data_target_inactive

> crate::models::Generic200Response set_data_target_inactive(container_id, data_target_id)
Set Data Target Inactive

Sets a data target inactive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_target_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

