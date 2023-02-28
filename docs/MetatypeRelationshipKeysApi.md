# \MetatypeRelationshipKeysApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive_metatype_relationship_key**](MetatypeRelationshipKeysApi.md#archive_metatype_relationship_key) | **DELETE** /containers/{container_id}/metatype_relationships/{relationship_id}/keys/{key_id} | Archive Metatype Relationship Key
[**create_metatype_relationship_key**](MetatypeRelationshipKeysApi.md#create_metatype_relationship_key) | **POST** /containers/{container_id}/metatype_relationships/{relationship_id}/keys | Create Metatype Relationship Key
[**list_metatype_relationship_keys**](MetatypeRelationshipKeysApi.md#list_metatype_relationship_keys) | **GET** /containers/{container_id}/metatype_relationships/{relationship_id}/keys | List Metatype Relationship Keys
[**retrieve_metatype_relationship_key**](MetatypeRelationshipKeysApi.md#retrieve_metatype_relationship_key) | **GET** /containers/{container_id}/metatype_relationships/{relationship_id}/keys/{key_id} | Retrieve Metatype Relationship Key
[**update_metatype_relationship_key**](MetatypeRelationshipKeysApi.md#update_metatype_relationship_key) | **PUT** /containers/{container_id}/metatype_relationships/{relationship_id}/keys/{key_id} | Update Metaype Relationship Key



## archive_metatype_relationship_key

> crate::models::Generic200Response archive_metatype_relationship_key(container_id, relationship_id, key_id)
Archive Metatype Relationship Key

Archives a Metatype Relationship Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**relationship_id** | **String** |  | [required] |
**key_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_metatype_relationship_key

> crate::models::CreateMetatypeRelationshipKeysResponse create_metatype_relationship_key(container_id, relationship_id, body)
Create Metatype Relationship Key

Creates a new key for a metatype relationship. Keys consist of a unique key name (unique to the metatype relationship), key type, default values, and allowed values. Of those, only the first two are required. The `dataType` field accepts only one of the following values: number, string, date, boolean, enumeration, file.  The fields `defaultValue` and `options` will only accept an array of the following types: string, boolean, number, float.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**relationship_id** | **String** |  | [required] |
**body** | [**CreateMetatypeRelationshipKeyRequest**](CreateMetatypeRelationshipKeyRequest.md) |  | [required] |

### Return type

[**crate::models::CreateMetatypeRelationshipKeysResponse**](CreateMetatypeRelationshipKeysResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_metatype_relationship_keys

> crate::models::ListMetatypeRelationshipKeysResponse list_metatype_relationship_keys(container_id, relationship_id)
List Metatype Relationship Keys

Retrieves all keys for a Metatype Relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**relationship_id** | **String** |  | [required] |

### Return type

[**crate::models::ListMetatypeRelationshipKeysResponse**](ListMetatypeRelationshipKeysResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_metatype_relationship_key

> crate::models::GetMetatypeRelationshipKeyResponse retrieve_metatype_relationship_key(container_id, relationship_id, key_id)
Retrieve Metatype Relationship Key

Retrieve a single key for a Metatype Relationship by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**relationship_id** | **String** |  | [required] |
**key_id** | **String** |  | [required] |

### Return type

[**crate::models::GetMetatypeRelationshipKeyResponse**](GetMetatypeRelationshipKeyResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metatype_relationship_key

> crate::models::UpdateMetatypeRelationshipKeyResponse update_metatype_relationship_key(container_id, relationship_id, key_id, body)
Update Metaype Relationship Key

Updates a Metatype Relationship key. The update must follow the same format as creation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**relationship_id** | **String** |  | [required] |
**key_id** | **String** |  | [required] |
**body** | [**RelationshipKey**](RelationshipKey.md) |  | [required] |

### Return type

[**crate::models::UpdateMetatypeRelationshipKeyResponse**](UpdateMetatypeRelationshipKeyResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

