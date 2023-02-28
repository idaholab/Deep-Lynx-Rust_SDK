# \MetatypeRelationshipPairsApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive_metatype_relationship_pair**](MetatypeRelationshipPairsApi.md#archive_metatype_relationship_pair) | **DELETE** /containers/{container_id}/metatype_relationship_pairs/{pair_id} | Archive Metatype Relationship Pair
[**create_metatype_relationship_pair**](MetatypeRelationshipPairsApi.md#create_metatype_relationship_pair) | **POST** /containers/{container_id}/metatype_relationship_pairs | Create Metatype Relationship Pair
[**list_metatype_relationship_pairs**](MetatypeRelationshipPairsApi.md#list_metatype_relationship_pairs) | **GET** /containers/{container_id}/metatype_relationship_pairs | List Metatype Relationship Pairs
[**retrieve_metatype_relationship_pair**](MetatypeRelationshipPairsApi.md#retrieve_metatype_relationship_pair) | **GET** /containers/{container_id}/metatype_relationship_pairs/{pair_id} | Retrieve Metatype Relationship Pair
[**update_metatype_relationship_pair**](MetatypeRelationshipPairsApi.md#update_metatype_relationship_pair) | **PUT** /containers/{container_id}/metatype_relationship_pairs/{pair_id} | Update Metaype Relationship Pair



## archive_metatype_relationship_pair

> crate::models::Generic200Response archive_metatype_relationship_pair(container_id, pair_id)
Archive Metatype Relationship Pair

Archives a Metatype Relationship Pair.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**pair_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_metatype_relationship_pair

> crate::models::CreateMetatypeRelationshipPairsResponse create_metatype_relationship_pair(container_id, body)
Create Metatype Relationship Pair

Create a new Metaype Relationship Pair. Describes the connection between two metatypes by connecting them using a Metatype Relationship. Pass in an array for bulk creation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**body** | [**CreateMetatypeRelationshipPairRequest**](CreateMetatypeRelationshipPairRequest.md) |  | [required] |

### Return type

[**crate::models::CreateMetatypeRelationshipPairsResponse**](CreateMetatypeRelationshipPairsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_metatype_relationship_pairs

> crate::models::ListMetatypeRelationshipPairsResponse list_metatype_relationship_pairs(container_id, limit, offset, name, archived, count, load_relationships, destination_id, origin_id, metatype_id)
List Metatype Relationship Pairs

List all Metatype Relationship Pairs for current container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**name** | Option<**String**> | Filter metatype relationship pairs with names that match this pattern |  |
**archived** | Option<**String**> | Set to true to include archived metatype relationship pairs |  |
**count** | Option<**String**> | Set to true to return an integer count of the number of metatype relationship pairs |  |
**load_relationships** | Option<**String**> | Set to false to not return the relationships for the selected metatype relationship pairs (true by default) |  |
**destination_id** | Option<**String**> | Filter destination by metatype ID |  |
**origin_id** | Option<**String**> | Filter origin by metatype ID |  |
**metatype_id** | Option<**String**> | Filter by metatype ID |  |

### Return type

[**crate::models::ListMetatypeRelationshipPairsResponse**](ListMetatypeRelationshipPairsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_metatype_relationship_pair

> crate::models::GetMetatypeRelationshipPairResponse retrieve_metatype_relationship_pair(container_id, pair_id)
Retrieve Metatype Relationship Pair

Retrieves a single Metatype Relationship Pair.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**pair_id** | **String** |  | [required] |

### Return type

[**crate::models::GetMetatypeRelationshipPairResponse**](GetMetatypeRelationshipPairResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metatype_relationship_pair

> crate::models::UpdateMetatypeRelationshipPairResponse update_metatype_relationship_pair(container_id, pair_id, relationship_pair)
Update Metaype Relationship Pair

Updates the specified metatype relationship pair.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**pair_id** | **String** |  | [required] |
**relationship_pair** | Option<[**RelationshipPair**](RelationshipPair.md)> |  |  |

### Return type

[**crate::models::UpdateMetatypeRelationshipPairResponse**](UpdateMetatypeRelationshipPairResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

