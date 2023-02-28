# \DataQueryApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**data_query**](DataQueryApi.md#data_query) | **POST** /containers/{container_id}/data | Query Data
[**query_graph**](DataQueryApi.md#query_graph) | **POST** /containers/{container_id}/query | Query Graph (Deprecated)



## data_query

> crate::models::QueryGraph200Response data_query(container_id, body, metadata_enabled, point_in_time)
Query Data

Query data from your container using GraphQL. You can learn more here - https://gitlab.software.inl.gov/b650/Deep-Lynx/-/wikis/Querying-Data-With-GraphQL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |
**metadata_enabled** | Option<**bool**> |  |  |[default to false]
**point_in_time** | Option<**String**> |  |  |

### Return type

[**crate::models::QueryGraph200Response**](QueryGraph_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, text/plain, application/xml
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_graph

> crate::models::QueryGraph200Response query_graph(container_id, body)
Query Graph (Deprecated)

Query the graph of the specified container using GraphQL. GraphQL queries may be formatted as json or plain text. This has been deprecated in favor of the `/containers/{container_id}/data` endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::QueryGraph200Response**](QueryGraph_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

