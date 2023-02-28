# \MetatypeKeysApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive_metatype_key**](MetatypeKeysApi.md#archive_metatype_key) | **DELETE** /containers/{container_id}/metatypes/{metatype_id}/keys/{key_id} | Archive Metatype Key
[**create_metatype_key**](MetatypeKeysApi.md#create_metatype_key) | **POST** /containers/{container_id}/metatypes/{metatype_id}/keys | Create Metatype Key
[**list_metatypes_keys**](MetatypeKeysApi.md#list_metatypes_keys) | **GET** /containers/{container_id}/metatypes/{metatype_id}/keys | List Metatype's Keys
[**retrieve_metatype_key**](MetatypeKeysApi.md#retrieve_metatype_key) | **GET** /containers/{container_id}/metatypes/{metatype_id}/keys/{key_id} | Retrieve Metatype Key
[**update_metatype_key**](MetatypeKeysApi.md#update_metatype_key) | **PUT** /containers/{container_id}/metatypes/{metatype_id}/keys/{key_id} | Update Metatype Key



## archive_metatype_key

> crate::models::Generic200Response archive_metatype_key(container_id, metatype_id, key_id)
Archive Metatype Key

Archiving the metatype key prevents any new types from implementing the key. It *does not remove key/value pairs on existing types*. We highly recommend you archive type keys instead of deleting them so that previous data is not affected.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**metatype_id** | **String** |  | [required] |
**key_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_metatype_key

> crate::models::CreateMetatypeKeysResponse create_metatype_key(container_id, metatype_id, body)
Create Metatype Key

Creates a new key for a metatype. Keys consist of a unique key name (unique to the metatype only), key type, default values, and allowed values. Of those, only the first two are required. The `dataType` field accepts only one of the following values: number, string, date, boolean, enumeration, file.  The fields `defaultValue` and `options` will only accept an array of the following types: string, boolean, number, float. Pass in an array for bulk creation. Currently the validation and cardinality functionality of keys are NOT checked at data insertion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**metatype_id** | **String** |  | [required] |
**body** | [**CreateMetatypeKeyRequest**](CreateMetatypeKeyRequest.md) |  | [required] |

### Return type

[**crate::models::CreateMetatypeKeysResponse**](CreateMetatypeKeysResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_metatypes_keys

> crate::models::ListMetatypeKeysResponse list_metatypes_keys(container_id, metatype_id)
List Metatype's Keys

Lists all currently valid and available keys for the metatype.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**metatype_id** | **String** |  | [required] |

### Return type

[**crate::models::ListMetatypeKeysResponse**](ListMetatypeKeysResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_metatype_key

> crate::models::GetMetatypeKeyResponse retrieve_metatype_key(container_id, metatype_id, key_id)
Retrieve Metatype Key

Retrieve the specified key for the metatype.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**metatype_id** | **String** |  | [required] |
**key_id** | **String** |  | [required] |

### Return type

[**crate::models::GetMetatypeKeyResponse**](GetMetatypeKeyResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metatype_key

> crate::models::UpdateMetatypeKeyResponse update_metatype_key(container_id, metatype_id, key_id, body)
Update Metatype Key

Updates a single key for a metatype.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**metatype_id** | **String** |  | [required] |
**key_id** | **String** |  | [required] |
**body** | [**MetatypeKey**](MetatypeKey.md) |  | [required] |

### Return type

[**crate::models::UpdateMetatypeKeyResponse**](UpdateMetatypeKeyResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

