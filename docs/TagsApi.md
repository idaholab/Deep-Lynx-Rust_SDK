# \TagsApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_webgl_file**](TagsApi.md#delete_webgl_file) | **DELETE** /containers/{container_id}/graphs/webgl/files/{file_id} | Delete WebGL File
[**detach_tag_from_edge**](TagsApi.md#detach_tag_from_edge) | **DELETE** /containers/{container_id}/graphs/tags/{tag_id}/edges/{edge_id} | Detach Tag From Edge
[**detach_tag_from_file**](TagsApi.md#detach_tag_from_file) | **DELETE** /containers/{container_id}/graphs/tags/{tag_id}/files/{file_id} | Detach Tag From File
[**detach_tag_from_node**](TagsApi.md#detach_tag_from_node) | **DELETE** /containers/{container_id}/graphs/tags/{tag_id}/nodes/{node_id} | Detach Tag From Node
[**get_containers_container_id_graphs_tags_edges_edge_id**](TagsApi.md#get_containers_container_id_graphs_tags_edges_edge_id) | **GET** /containers/{container_id}/graphs/tags/edges/{edge_id} | List Tags for Edge
[**get_containers_container_id_graphs_tags_files_file_id**](TagsApi.md#get_containers_container_id_graphs_tags_files_file_id) | **GET** /containers/{container_id}/graphs/tags/files/{file_id} | List Tags for File
[**get_containers_container_id_graphs_tags_nodes_node_id**](TagsApi.md#get_containers_container_id_graphs_tags_nodes_node_id) | **GET** /containers/{container_id}/graphs/tags/nodes/{node_id} | List Tags for Node
[**get_containers_container_id_graphs_tags_nodes_tag_id**](TagsApi.md#get_containers_container_id_graphs_tags_nodes_tag_id) | **GET** /containers/{container_id}/graphs/tags/{tag_id}/nodes | List Nodes with Tag
[**get_containers_container_id_graphs_tags_tag_id_edges**](TagsApi.md#get_containers_container_id_graphs_tags_tag_id_edges) | **GET** /containers/{container_id}/graphs/tags/{tag_id}/edges | List Edges with Tag
[**get_containers_container_id_graphs_tags_tag_id_files**](TagsApi.md#get_containers_container_id_graphs_tags_tag_id_files) | **GET** /containers/{container_id}/graphs/tags/{tag_id}/files | List Files with Tag
[**list_tags**](TagsApi.md#list_tags) | **GET** /containers/{container_id}/graphs/tags | List Tags
[**list_webgl**](TagsApi.md#list_webgl) | **GET** /containers/{container_id}/graphs/webgl | List WebGL
[**post_tags**](TagsApi.md#post_tags) | **POST** /containers/{container_id}/graphs/tags | Create Tag
[**put_containers_container_id_graphs_tags_nodes_node_id**](TagsApi.md#put_containers_container_id_graphs_tags_nodes_node_id) | **PUT** /containers/{container_id}/graphs/tags/{tag_id}/nodes/{node_id} | Tag Node
[**put_containers_container_id_graphs_tags_tag_id**](TagsApi.md#put_containers_container_id_graphs_tags_tag_id) | **PUT** /containers/{container_id}/graphs/tags/{tag_id} | Update Tag
[**put_containers_container_id_graphs_tags_tag_id_edges_edge_id**](TagsApi.md#put_containers_container_id_graphs_tags_tag_id_edges_edge_id) | **PUT** /containers/{container_id}/graphs/tags/{tag_id}/edges/{edge_id} | Tag Edge
[**put_containers_container_id_graphs_tags_tag_id_files_file_id**](TagsApi.md#put_containers_container_id_graphs_tags_tag_id_files_file_id) | **PUT** /containers/{container_id}/graphs/tags/{tag_id}/files/{file_id} | Tag File
[**update_webgl_files**](TagsApi.md#update_webgl_files) | **GET** /containers/{container_id}/graphs/webgl/files/{file_id} | Update WebGL Files
[**upload_webgl**](TagsApi.md#upload_webgl) | **POST** /containers/{container_id}/graphs/webgl | Upload WebGL



## delete_webgl_file

> delete_webgl_file(container_id, file_id)
Delete WebGL File

Deletes a WebGL file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**file_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detach_tag_from_edge

> detach_tag_from_edge(container_id, edge_id, tag_id)
Detach Tag From Edge

Removes the connection between a tag and an edge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**edge_id** | **String** |  | [required] |
**tag_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detach_tag_from_file

> detach_tag_from_file(container_id, file_id, tag_id)
Detach Tag From File

Removes the connection between a tag and a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**file_id** | **String** |  | [required] |
**tag_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detach_tag_from_node

> detach_tag_from_node(container_id, node_id, tag_id)
Detach Tag From Node

Removes the connection between a tag and a node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |
**tag_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_containers_container_id_graphs_tags_edges_edge_id

> get_containers_container_id_graphs_tags_edges_edge_id(container_id, edge_id)
List Tags for Edge

List all Tags for an Edge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**edge_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_containers_container_id_graphs_tags_files_file_id

> get_containers_container_id_graphs_tags_files_file_id(container_id, file_id)
List Tags for File

List all Tags for a File

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**file_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_containers_container_id_graphs_tags_nodes_node_id

> get_containers_container_id_graphs_tags_nodes_node_id(container_id, node_id)
List Tags for Node

List all Tags on a Node

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


## get_containers_container_id_graphs_tags_nodes_tag_id

> get_containers_container_id_graphs_tags_nodes_tag_id(container_id, tag_id)
List Nodes with Tag

List all Nodes with a Tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**tag_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_containers_container_id_graphs_tags_tag_id_edges

> get_containers_container_id_graphs_tags_tag_id_edges(container_id, tag_id)
List Edges with Tag

List all Edges with Tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**tag_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_containers_container_id_graphs_tags_tag_id_files

> get_containers_container_id_graphs_tags_tag_id_files(container_id, tag_id)
List Files with Tag

List all Files with Tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**tag_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tags

> list_tags(container_id)
List Tags

List all tags for a container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_webgl

> list_webgl(container_id)
List WebGL

Lists all WebGL files and tags for a container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tags

> post_tags(container_id, post_tags_request)
Create Tag

Create a Tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**post_tags_request** | Option<[**PostTagsRequest**](PostTagsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/javascript
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_containers_container_id_graphs_tags_nodes_node_id

> put_containers_container_id_graphs_tags_nodes_node_id(container_id, node_id, tag_id)
Tag Node

Attach Tag to Node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |
**tag_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_containers_container_id_graphs_tags_tag_id

> put_containers_container_id_graphs_tags_tag_id(container_id, tag_id, post_tags_request)
Update Tag

Update a tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**tag_id** | **String** |  | [required] |
**post_tags_request** | Option<[**PostTagsRequest**](PostTagsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_containers_container_id_graphs_tags_tag_id_edges_edge_id

> put_containers_container_id_graphs_tags_tag_id_edges_edge_id(container_id, edge_id, tag_id)
Tag Edge

Tag an Edge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**edge_id** | **String** |  | [required] |
**tag_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_containers_container_id_graphs_tags_tag_id_files_file_id

> put_containers_container_id_graphs_tags_tag_id_files_file_id(container_id, file_id, tag_id)
Tag File

Tag a File

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**file_id** | **String** |  | [required] |
**tag_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webgl_files

> update_webgl_files(container_id, file_id)
Update WebGL Files

Updates WebGL files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**file_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_webgl

> upload_webgl(container_id, tag, file)
Upload WebGL

Upload a WebGL build. The tag will be created if it doesn't exist, and the file will be uploaded. The tag is then associated with the file. The tag will automatically have {'webgl': true} added to its metadata field

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**tag** | Option<**String**> | Tag name |  |
**file** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

