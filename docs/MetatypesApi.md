# \MetatypesApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive_metatype**](MetatypesApi.md#archive_metatype) | **DELETE** /containers/{container_id}/metatypes/{metatype_id} | Archive Metatype
[**create_metatype**](MetatypesApi.md#create_metatype) | **POST** /containers/{container_id}/metatypes | Create Metatype
[**list_metatypes**](MetatypesApi.md#list_metatypes) | **GET** /containers/{container_id}/metatypes | List Metatypes
[**retrieve_metaype**](MetatypesApi.md#retrieve_metaype) | **GET** /containers/{container_id}/metatypes/{metatype_id} | Retrieve Metatype
[**update_metatype**](MetatypesApi.md#update_metatype) | **PUT** /containers/{container_id}/metatypes/{metatype_id} | Update Metatype
[**validate_metatype_properties**](MetatypesApi.md#validate_metatype_properties) | **POST** /containers/{container_id}/metatypes/{metatype_id} | Validate Metatype Properties



## archive_metatype

> crate::models::Generic200Response archive_metatype(container_id, metatype_id)
Archive Metatype

Archives the metatype. This is preferred over deletion as deletion has a cascading effect on the deleted metatype's keys, relationships, and relationship keys. When in doubt, archive over delete. We'd rather have tombstones than cremating the metatype.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**metatype_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_metatype

> crate::models::CreateMetatypesResponse create_metatype(container_id, body)
Create Metatype

Create a new metatype. Pass in an array for bulk creation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**body** | [**CreateMetatypeRequest**](CreateMetatypeRequest.md) |  | [required] |

### Return type

[**crate::models::CreateMetatypesResponse**](CreateMetatypesResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_metatypes

> crate::models::ListMetatypesResponse list_metatypes(container_id, limit, offset, name, description, count, load_keys, sort_by, sort_desc)
List Metatypes

List all metatypes that the container has access to. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**name** | Option<**String**> | Filter metatypes with names that match this pattern |  |
**description** | Option<**String**> | Filter metatypes with descriptions that match this pattern |  |
**count** | Option<**String**> | Set to true to return an integer count of the number of metatypes |  |
**load_keys** | Option<**String**> | Set to false to not return the keys for the selected metatypes (true by default) |  |
**sort_by** | Option<**String**> | Supply the name of a metatype attribute (name, created_at, etc) by which to sort |  |
**sort_desc** | Option<**String**> | Set true to sort descending |  |

### Return type

[**crate::models::ListMetatypesResponse**](ListMetatypesResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_metaype

> crate::models::GetMetatypeResponse retrieve_metaype(container_id, metatype_id)
Retrieve Metatype

Retrieves a single metatype.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**metatype_id** | **String** |  | [required] |

### Return type

[**crate::models::GetMetatypeResponse**](GetMetatypeResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metatype

> crate::models::UpdateMetatypeResponse update_metatype(container_id, metatype_id, body)
Update Metatype

Update a single Metatype in storage. Will fail if the updated name has already been taken.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**metatype_id** | **String** |  | [required] |
**body** | [**UpdateMetatypeRequest**](UpdateMetatypeRequest.md) |  | [required] |

### Return type

[**crate::models::UpdateMetatypeResponse**](UpdateMetatypeResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_metatype_properties

> crate::models::ValidateMetatypePropertiesResponse validate_metatype_properties(container_id, metatype_id, validate_metatype_properties_request)
Validate Metatype Properties

Returns any errors associated with the intended properties or keys for a metatype or else the data itself if no errors are present.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**metatype_id** | **String** |  | [required] |
**validate_metatype_properties_request** | Option<[**ValidateMetatypePropertiesRequest**](ValidateMetatypePropertiesRequest.md)> |  |  |

### Return type

[**crate::models::ValidateMetatypePropertiesResponse**](ValidateMetatypePropertiesResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

