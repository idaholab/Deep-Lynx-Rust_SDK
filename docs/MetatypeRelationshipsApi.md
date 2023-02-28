# \MetatypeRelationshipsApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive_metatype_relationship**](MetatypeRelationshipsApi.md#archive_metatype_relationship) | **DELETE** /containers/{container_id}/metatype_relationships/{relationship_id} | Archive Metatype Relationship
[**create_metatype_relationship**](MetatypeRelationshipsApi.md#create_metatype_relationship) | **POST** /containers/{container_id}/metatype_relationships | Create Metatype Relationship
[**list_metatype_relationships**](MetatypeRelationshipsApi.md#list_metatype_relationships) | **GET** /containers/{container_id}/metatype_relationships | List Metatype Relationships
[**retrieve_metatype_relationship**](MetatypeRelationshipsApi.md#retrieve_metatype_relationship) | **GET** /containers/{container_id}/metatype_relationships/{relationship_id} | Retrieve Metatype Relationship
[**update_metatype_relationship**](MetatypeRelationshipsApi.md#update_metatype_relationship) | **PUT** /containers/{container_id}/metatype_relationships/{relationship_id} | Update Metatype Relationship



## archive_metatype_relationship

> crate::models::Generic200Response archive_metatype_relationship(container_id, relationship_id)
Archive Metatype Relationship

Archive the metatype relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**relationship_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_metatype_relationship

> crate::models::CreateMetatypeRelationshipsResponse create_metatype_relationship(container_id, body)
Create Metatype Relationship

Create a new metatype relationship. Describes the connection that could exist between two metatypes and acts as a vehicle for relationship specific keys. Pass in an array for bulk creation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**body** | [**CreateMetatypeRelationshipRequest**](CreateMetatypeRelationshipRequest.md) |  | [required] |

### Return type

[**crate::models::CreateMetatypeRelationshipsResponse**](CreateMetatypeRelationshipsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_metatype_relationships

> crate::models::ListMetatypeRelationshipsResponse list_metatype_relationships(container_id, limit, offset, name, description, count, load_keys, sort_by, sort_desc)
List Metatype Relationships

List metatype relationships. Describes the connection between two metatypes and acts as a vehicle for relationship specific keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**name** | Option<**String**> | Filter metatype relationships with names that match this pattern |  |
**description** | Option<**String**> | Filter metatype relationships with descriptions that match this pattern |  |
**count** | Option<**String**> | Set to true to return an integer count of the number of metatype relationships |  |
**load_keys** | Option<**String**> | Set to false to not return the keys for the selected metatype relationships (true by default) |  |
**sort_by** | Option<**String**> | Supply the name of a metatype relationship attribute (name, created_at, etc) by which to sort |  |
**sort_desc** | Option<**String**> | Set true to sort descending |  |

### Return type

[**crate::models::ListMetatypeRelationshipsResponse**](ListMetatypeRelationshipsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_metatype_relationship

> crate::models::GetMetatypeRelationshipResponse retrieve_metatype_relationship(container_id, relationship_id)
Retrieve Metatype Relationship

Retrieve a single Metatype Relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**relationship_id** | **String** |  | [required] |

### Return type

[**crate::models::GetMetatypeRelationshipResponse**](GetMetatypeRelationshipResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metatype_relationship

> crate::models::UpdateMetatypeRelationshipResponse update_metatype_relationship(container_id, relationship_id, body)
Update Metatype Relationship

Updates the specified metatype relationship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**relationship_id** | **String** |  | [required] |
**body** | [**UpdateMetatypeRelationshipRequest**](UpdateMetatypeRelationshipRequest.md) |  | [required] |

### Return type

[**crate::models::UpdateMetatypeRelationshipResponse**](UpdateMetatypeRelationshipResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

