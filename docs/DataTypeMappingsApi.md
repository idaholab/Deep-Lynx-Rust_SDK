# \DataTypeMappingsApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**copy_transformations**](DataTypeMappingsApi.md#copy_transformations) | **POST** /containers/{container_id}/import/datasources/{sourceID}/mappings/{mappingID}/copy/{originalMappingID} | 
[**create_transformation**](DataTypeMappingsApi.md#create_transformation) | **POST** /containers/{container_id}/import/datasources/{data_source_id}/mappings/{mapping_id}/transformations | Create Data Type Mapping's Transformations
[**delete_data_type_mapping**](DataTypeMappingsApi.md#delete_data_type_mapping) | **DELETE** /containers/{container_id}/import/datasources/{data_source_id}/mappings/{mapping_id} | Delete Data Type Mapping
[**delete_transformation**](DataTypeMappingsApi.md#delete_transformation) | **DELETE** /containers/{container_id}/import/datasources/{data_source_id}/mappings/{mapping_id}/transformations/{transformation_id} | Delete Data Type Mapping's Transformations
[**export_type_mappings**](DataTypeMappingsApi.md#export_type_mappings) | **POST** /containers/{container_id}/import/datasources/{data_source_id}/mappings/export | Export Type Mappings
[**import_data_type_mappings**](DataTypeMappingsApi.md#import_data_type_mappings) | **POST** /containers/{container_id}/import/datasources/{data_source_id}/mappings/import | Import Data Type Mappings
[**list_data_type_mappings**](DataTypeMappingsApi.md#list_data_type_mappings) | **GET** /containers/{container_id}/import/datasources/{data_source_id}/mappings | List Data Type Mappings
[**list_transformations**](DataTypeMappingsApi.md#list_transformations) | **GET** /containers/{container_id}/import/datasources/{data_source_id}/mappings/{mapping_id}/transformations | List Data Type Mapping's Transformations
[**retrieve_data_type_mapping**](DataTypeMappingsApi.md#retrieve_data_type_mapping) | **GET** /containers/{container_id}/import/datasources/{data_source_id}/mappings/{mapping_id} | Retrieve Data Type Mapping
[**update_data_type_mapping**](DataTypeMappingsApi.md#update_data_type_mapping) | **PUT** /containers/{container_id}/import/datasources/{data_source_id}/mappings/{mapping_id} | Update Data Type Mapping
[**update_transformation**](DataTypeMappingsApi.md#update_transformation) | **PUT** /containers/{container_id}/import/datasources/{data_source_id}/mappings/{mapping_id}/transformations/{transformation_id} | Update Data Type Mapping's Transformations



## copy_transformations

> copy_transformations(container_id, source_id, mapping_id, original_mapping_id)


This endpoint copies transformations from the {originalMappingID} type mapping (final parameter) to the {mappingID} type mapping. This POST has NO body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**source_id** | **String** |  | [required] |
**mapping_id** | **String** |  | [required] |
**original_mapping_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_transformation

> crate::models::CreateTransformationResponse create_transformation(container_id, data_source_id, mapping_id, body)
Create Data Type Mapping's Transformations

Create a transformation for the type mapping.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**mapping_id** | **String** |  | [required] |
**body** | [**CreateTypeMappingTransformationsRequest**](CreateTypeMappingTransformationsRequest.md) |  | [required] |

### Return type

[**crate::models::CreateTransformationResponse**](CreateTransformationResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_data_type_mapping

> crate::models::Generic200Response delete_data_type_mapping(container_id, data_source_id, mapping_id)
Delete Data Type Mapping

Permanently remove data type mapping.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**mapping_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_transformation

> crate::models::Generic200Response delete_transformation(container_id, data_source_id, mapping_id, transformation_id)
Delete Data Type Mapping's Transformations

Delete a transformation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**mapping_id** | **String** |  | [required] |
**transformation_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_type_mappings

> Vec<crate::models::TypeMapping> export_type_mappings(container_id, data_source_id, type_mapping_export_payload)
Export Type Mappings

Export type mappings for a datasource. Providing a JSON body is optional. If provided, the mapping_ids may be specified to indicate certain type mapping IDs to return. Additionally, a target data source may be provided to which the mappings will be copied.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**type_mapping_export_payload** | Option<[**TypeMappingExportPayload**](TypeMappingExportPayload.md)> |  |  |

### Return type

[**Vec<crate::models::TypeMapping>**](TypeMapping.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_data_type_mappings

> Vec<crate::models::ImportDataTypeMappingResponseInner> import_data_type_mappings(container_id, data_source_id, is_enabled, import_data_type_mappings_request)
Import Data Type Mappings

Import type mappings for a datasource. Accepts either a JSON body or actual JSON file. The payload should be an array of type mapping classes, previously generated using the export route.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**is_enabled** | Option<**bool**> |  |  |[default to false]
**import_data_type_mappings_request** | Option<[**ImportDataTypeMappingsRequest**](ImportDataTypeMappingsRequest.md)> |  |  |

### Return type

[**Vec<crate::models::ImportDataTypeMappingResponseInner>**](ImportDataTypeMappingResponse_inner.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_data_type_mappings

> crate::models::ListDataTypeMappingResponse list_data_type_mappings(container_id, data_source_id, limit, offset, needs_transformations, count, sort_by, sort_desc, resulting_metatype_name, resulting_metatype_relationship_name)
List Data Type Mappings

Lists data type mappings for the data source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**needs_transformations** | Option<**bool**> | boolean indicating if the return should consist of only mappings that need transformations |  |
**count** | Option<**bool**> | boolean indicating if the return value should be a count only |  |
**sort_by** | Option<**String**> | column to sort results by |  |
**sort_desc** | Option<**bool**> | boolean indicating if results should be in descending order |  |
**resulting_metatype_name** | Option<**String**> | if supplied, filters returned transformations by those that produce the resulting metatype |  |
**resulting_metatype_relationship_name** | Option<**String**> | if supplied, filters returned transformations by those that produce the resulting metatype relationship |  |

### Return type

[**crate::models::ListDataTypeMappingResponse**](ListDataTypeMappingResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transformations

> crate::models::ListTransformationResponse list_transformations(container_id, data_source_id, mapping_id)
List Data Type Mapping's Transformations

List transformations for a type mapping from storage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**mapping_id** | **String** |  | [required] |

### Return type

[**crate::models::ListTransformationResponse**](ListTransformationResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_data_type_mapping

> crate::models::GetDataTypeMappingResponse retrieve_data_type_mapping(container_id, data_source_id, mapping_id)
Retrieve Data Type Mapping

Retrieve a data type mapping

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**mapping_id** | **String** |  | [required] |

### Return type

[**crate::models::GetDataTypeMappingResponse**](GetDataTypeMappingResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_data_type_mapping

> crate::models::UpdateDataTypeMappingResponse update_data_type_mapping(container_id, data_source_id, mapping_id, type_mapping)
Update Data Type Mapping

Updates a data type mapping.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**mapping_id** | **String** |  | [required] |
**type_mapping** | Option<[**TypeMapping**](TypeMapping.md)> |  |  |

### Return type

[**crate::models::UpdateDataTypeMappingResponse**](UpdateDataTypeMappingResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_transformation

> crate::models::UpdateTransformationResponse update_transformation(container_id, data_source_id, mapping_id, transformation_id, body)
Update Data Type Mapping's Transformations

Update a transformation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**data_source_id** | **String** |  | [required] |
**mapping_id** | **String** |  | [required] |
**transformation_id** | **String** |  | [required] |
**body** | [**CreateTypeMappingTransformationsRequest**](CreateTypeMappingTransformationsRequest.md) |  | [required] |

### Return type

[**crate::models::UpdateTransformationResponse**](UpdateTransformationResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

