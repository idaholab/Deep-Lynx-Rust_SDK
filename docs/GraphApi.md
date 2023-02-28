# \GraphApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive_edge**](GraphApi.md#archive_edge) | **DELETE** /containers/{container_id}/graphs/edges/{edge_id} | Archive Edge
[**archive_node**](GraphApi.md#archive_node) | **DELETE** /containers/{container_id}/graphs/nodes/{node_id} | Archive Node
[**attach_edge_file**](GraphApi.md#attach_edge_file) | **PUT** /containers/{container_id}/graphs/edges/{edge_id}/files/{file_id} | Attach Edge File
[**attach_node_file**](GraphApi.md#attach_node_file) | **PUT** /containers/{container_id}/graphs/nodes/{node_id}/files/{file_id} | Attach Node File
[**create_or_update_edges**](GraphApi.md#create_or_update_edges) | **POST** /containers/{container_id}/graphs/edges | Create or Update Edges
[**create_or_update_nodes**](GraphApi.md#create_or_update_nodes) | **POST** /containers/{container_id}/graphs/nodes | Create Or Update Nodes
[**delete_node_file**](GraphApi.md#delete_node_file) | **DELETE** /containers/{container_id}/graphs/nodes/{node_id}/files/{file_id} | Detach Node File
[**detach_node_file**](GraphApi.md#detach_node_file) | **DELETE** /containers/{container_id}/graphs/edges/{edge_id}/files/{file_id} | Detach Node File
[**list_edge_files**](GraphApi.md#list_edge_files) | **GET** /containers/{container_id}/graphs/edges/{edge_id}/files | List Edge Files
[**list_edges**](GraphApi.md#list_edges) | **GET** /containers/{container_id}/graphs/edges | List Edges
[**list_node_files**](GraphApi.md#list_node_files) | **GET** /containers/{container_id}/graphs/nodes/{node_id}/files | List Node Files
[**list_nodes**](GraphApi.md#list_nodes) | **GET** /containers/{container_id}/graphs/nodes | List Nodes
[**list_nodes_by_metatype_id**](GraphApi.md#list_nodes_by_metatype_id) | **GET** /containers/{container_id}/graphs/nodes/metatype/{metatype_id} | List Nodes By Metatype ID
[**retrieve_edge**](GraphApi.md#retrieve_edge) | **GET** /containers/{container_id}/graphs/edges/{edge_id} | Retrieve Edge
[**retrieve_node**](GraphApi.md#retrieve_node) | **GET** /containers/{container_id}/graphs/nodes/{node_id} | Retrieve Node
[**retrieve_nth_nodes**](GraphApi.md#retrieve_nth_nodes) | **GET** /containers/{container_id}/graphs/nodes/{node_id}/graph | Nth Node Query
[**timeseries_data_source_query**](GraphApi.md#timeseries_data_source_query) | **POST** /containers/{container_id}/import/datasources/{data_source_id}/data | Timeseries Data Source Query
[**timeseries_node_query**](GraphApi.md#timeseries_node_query) | **POST** /containers/{container_id}/graphs/nodes/{node_id}/timeseries | Timeseries Node Query



## archive_edge

> crate::models::Generic200Response archive_edge(container_id, edge_id)
Archive Edge

Archives an edge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**edge_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## archive_node

> crate::models::Generic200Response archive_node(container_id, node_id)
Archive Node

Archives a node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attach_edge_file

> crate::models::Generic200Response attach_edge_file(container_id, file_id, edge_id)
Attach Edge File

Attach a file to an edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**file_id** | **String** |  | [required] |
**edge_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attach_node_file

> crate::models::Generic200Response attach_node_file(container_id, node_id, file_id)
Attach Node File

Attach a file to a node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |
**file_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_or_update_edges

> Vec<crate::models::Edge> create_or_update_edges(container_id, create_or_update_edges_request)
Create or Update Edges

This endpoint will attempt to create a connection between two nodes. You can either pass in the node's Deep Lynx IDs, or the node's original id, metatype id, and data source id to create these edges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**create_or_update_edges_request** | Option<[**CreateOrUpdateEdgesRequest**](CreateOrUpdateEdgesRequest.md)> |  |  |

### Return type

[**Vec<crate::models::Edge>**](Edge.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_or_update_nodes

> Vec<crate::models::Node> create_or_update_nodes(container_id, body)
Create Or Update Nodes

This endpoint will either create new nodes or update nodes if one with the same original_id is passed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**body** | [**CreateOrUpdateNodesRequest**](CreateOrUpdateNodesRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::Node>**](Node.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_node_file

> crate::models::Generic200Response delete_node_file(container_id, node_id, file_id)
Detach Node File

Detach file from node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |
**file_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detach_node_file

> crate::models::Generic200Response detach_node_file(container_id, file_id, edge_id)
Detach Node File

Detach file from an edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**file_id** | **String** |  | [required] |
**edge_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_edge_files

> crate::models::ListEdgeFiles list_edge_files(container_id, edge_id)
List Edge Files

Lists all attached files for edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**edge_id** | **String** |  | [required] |

### Return type

[**crate::models::ListEdgeFiles**](ListEdgeFiles.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_edges

> crate::models::ListEdgesResponse list_edges(container_id, limit, offset, origin_id, destination_id, relationship_pair_id, relationship_pair_name)
List Edges

List Edges from storage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**origin_id** | Option<**String**> |  |  |
**destination_id** | Option<**String**> |  |  |
**relationship_pair_id** | Option<**String**> |  |  |
**relationship_pair_name** | Option<**String**> |  |  |

### Return type

[**crate::models::ListEdgesResponse**](ListEdgesResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_node_files

> crate::models::ListNodeFiles list_node_files(container_id, node_id)
List Node Files

Lists all attached files for node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

[**crate::models::ListNodeFiles**](ListNodeFiles.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_nodes

> crate::models::ListNodesResponse list_nodes(container_id, limit, offset, transformation_id, metatype_id, data_source_id)
List Nodes

List nodes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**transformation_id** | Option<**String**> | Return only nodes for the selected type transformation |  |
**metatype_id** | Option<**String**> | Return only nodes for the selected metatype |  |
**data_source_id** | Option<**String**> | Return only nodes for the selected datasource |  |

### Return type

[**crate::models::ListNodesResponse**](ListNodesResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_nodes_by_metatype_id

> crate::models::ListNodesByMetatypeResponse list_nodes_by_metatype_id(container_id, metatype_id, limit, offset)
List Nodes By Metatype ID

List Nodes, filter by MetatypeID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**metatype_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**crate::models::ListNodesByMetatypeResponse**](ListNodesByMetatypeResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_edge

> crate::models::GetEdgeResponse retrieve_edge(container_id, edge_id)
Retrieve Edge

Retrieve a single edge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**edge_id** | **String** |  | [required] |

### Return type

[**crate::models::GetEdgeResponse**](GetEdgeResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_node

> crate::models::GetNodeResponse retrieve_node(container_id, node_id)
Retrieve Node

Retrieve a single node from storage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

[**crate::models::GetNodeResponse**](GetNodeResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_nth_nodes

> crate::models::RetrieveNthNodes200Response retrieve_nth_nodes(container_id, node_id, depth)
Nth Node Query

Retrieve n layers of node-edge relationships given a depth n and an origin node id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |
**depth** | Option<**String**> | Number of layers deep to query. Defaults to 10. |  |[default to 10]

### Return type

[**crate::models::RetrieveNthNodes200Response**](RetrieveNthNodes_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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

