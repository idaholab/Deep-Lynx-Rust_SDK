# CreateOrUpdateNodesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**container_id** | **String** |  | 
**original_data_id** | Option<**String**> | Passing in just the original data id will attempt to update a node with the same composite id (data source id, metatype id, and original data id). | [optional]
**data_source_id** | **String** |  | 
**metatype_id** | **String** |  | 
**properties** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


