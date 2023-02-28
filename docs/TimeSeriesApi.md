# \TimeSeriesApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**timeseries_data_source_query**](TimeSeriesApi.md#timeseries_data_source_query) | **POST** /containers/{container_id}/import/datasources/{data_source_id}/data | Timeseries Data Source Query
[**timeseries_node_query**](TimeSeriesApi.md#timeseries_node_query) | **POST** /containers/{container_id}/graphs/nodes/{node_id}/timeseries | Timeseries Node Query



## timeseries_data_source_query

> timeseries_data_source_query(container_id, data_source_id)
Timeseries Data Source Query

This is an endpoint that accepts a GraphQL query and returns the results of that query. Primarily used for working with time series data without requiring attachment to a node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timeseries_node_query

> timeseries_node_query(container_id, node_id)
Timeseries Node Query

This is an endpoint that accepts a GraphQL query and returns the results of that query. Primarily used for working with time series data on nodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

