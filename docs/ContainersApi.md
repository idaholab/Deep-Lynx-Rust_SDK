# \ContainersApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**acknowledge_container_alert**](ContainersApi.md#acknowledge_container_alert) | **POST** /containers/{container_id}/alerts/{alert_id} | Acknowledge Container Alert
[**approve_ontology_version**](ContainersApi.md#approve_ontology_version) | **PUT** /containers/{container_id}/ontology/versions/{ontology_version_id}/approve | Approve Ontology Version
[**archive_container**](ContainersApi.md#archive_container) | **DELETE** /containers/{container_id} | Archive Container
[**container_batch_update**](ContainersApi.md#container_batch_update) | **PUT** /containers | Container Batch Update
[**create_container**](ContainersApi.md#create_container) | **POST** /containers | Create Container
[**import_container**](ContainersApi.md#import_container) | **POST** /containers/import | Import Container
[**list_container_alerts**](ContainersApi.md#list_container_alerts) | **GET** /containers/{container_id}/alerts | List Container Alerts
[**list_containers**](ContainersApi.md#list_containers) | **GET** /containers | List Containers
[**list_ontology_versions**](ContainersApi.md#list_ontology_versions) | **GET** /containers/{container_id}/ontology/versions | List Ontology Versions
[**publish_ontology_version**](ContainersApi.md#publish_ontology_version) | **POST** /containers/{container_id}/ontology/versions/{ontology_version_id}/publish | Publish Ontology Version
[**reject_ontology_version_approval**](ContainersApi.md#reject_ontology_version_approval) | **DELETE** /containers/{container_id}/ontology/versions/{ontology_version_id}/approve | Reject Ontology Version Approval
[**repair_container_permissions**](ContainersApi.md#repair_container_permissions) | **POST** /containers/{container_id}/permissions | Repair Container Permissions
[**retrieve_container**](ContainersApi.md#retrieve_container) | **GET** /containers/{container_id} | Retrieve Container
[**retrieve_ontology_version**](ContainersApi.md#retrieve_ontology_version) | **GET** /containers/{container_id}/ontology/versions/{version_id} | Retrieve Ontology Version
[**rollback_ontology_version**](ContainersApi.md#rollback_ontology_version) | **POST** /containers/{container_id}/ontology/versions/{version_id}/rollback | Rollback Ontology Version
[**send_ontology_version_for_approval**](ContainersApi.md#send_ontology_version_for_approval) | **POST** /containers/{container_id}/ontology/versions/{ontology_version_id}/approve | Send Ontology Version for Approval
[**set_container_active**](ContainersApi.md#set_container_active) | **POST** /containers/{container_id}/active | Set Container Active
[**update_container**](ContainersApi.md#update_container) | **PUT** /containers/{container_id} | Update Container
[**update_container_import**](ContainersApi.md#update_container_import) | **PUT** /containers/import/{container_id} | Update Container Import



## acknowledge_container_alert

> acknowledge_container_alert(container_id, alert_id)
Acknowledge Container Alert

Post with no body to acknowledge an alert and remove it from the active alerts list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**alert_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## approve_ontology_version

> approve_ontology_version(container_id, ontology_version_id)
Approve Ontology Version

Approves an ontology version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**ontology_version_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## archive_container

> crate::models::Generic200Response archive_container(container_id, permanent)
Archive Container

Archives a Container. This is preferred over deletion as deletion has a cascading effect on the deleted type's keys, relationships, and relationship keys. When in doubt, archive over delete. We'd rather have tombstones than cremating the type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**permanent** | Option<**bool**> | If true, permanently deletes the container |  |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## container_batch_update

> crate::models::BatchUpdateContainerResponse container_batch_update(body)
Container Batch Update

Accepts an array of container objects - will attempt to update all of them in a single transaction. If the update fails, none of them will go through.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<crate::models::BatchContainerUpdateRequestInner>**](BatchContainerUpdateRequest_inner.md) |  | [required] |

### Return type

[**crate::models::BatchUpdateContainerResponse**](BatchUpdateContainerResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_container

> crate::models::CreateContainerResponse create_container(body)
Create Container

Creates a new container object. Containers are the root level object and are considered to contain both the ontology(in form of Metatypes, Metatype Keys, and MetatypeRelationships) as well as the data stored under that ontology. Endpoint will accept both a single container request object, or an array of container request objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateContainerRequest**](CreateContainerRequest.md) |  | [required] |

### Return type

[**crate::models::CreateContainerResponse**](CreateContainerResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_container

> crate::models::ContainerImportResponse import_container(name, description, data_versioning_enabled, dryrun, path, file)
Import Container

An optional query param `dryrun` may be included with a value of `true` in order to return a HTML formatted string explaining the name and description of the container along with the number of metatypes, metatype relationships, and metatype keys to be created. This request uses a form-data body. If the ontology to be imported is being referenced via url, provide the url via a `path` field. Otherwise a local file may be provided. A file takes precedence over a `path` value if both are provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**description** | **String** |  | [required] |
**data_versioning_enabled** | **bool** |  | [required] |
**dryrun** | Option<**bool**> | If true returns a description of the container that will be created and its contents. |  |
**path** | Option<**String**> |  |  |
**file** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**crate::models::ContainerImportResponse**](ContainerImportResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_container_alerts

> list_container_alerts(container_id)
List Container Alerts

List all active alerts for a container by ID.

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


## list_containers

> crate::models::ListContainerResponse list_containers()
List Containers

List all containers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListContainerResponse**](ListContainerResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ontology_versions

> crate::models::ListOntologyVersions200Response list_ontology_versions(container_id)
List Ontology Versions

Lists all versions of the ontology for a container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |

### Return type

[**crate::models::ListOntologyVersions200Response**](ListOntologyVersions_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_ontology_version

> publish_ontology_version(container_id, ontology_version_id)
Publish Ontology Version

Publishes an ontology version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**ontology_version_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reject_ontology_version_approval

> reject_ontology_version_approval(container_id, ontology_version_id)
Reject Ontology Version Approval

Rejects an ontology version (either in a pending status or after it has been approved).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**ontology_version_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repair_container_permissions

> crate::models::Generic200Response repair_container_permissions(container_id)
Repair Container Permissions

Repairs a container's permission set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_container

> crate::models::GetContainerResponse retrieve_container(container_id)
Retrieve Container

Retrieve container by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |

### Return type

[**crate::models::GetContainerResponse**](GetContainerResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_ontology_version

> crate::models::RetrieveOntologyVersion200Response retrieve_ontology_version(container_id, version_id)
Retrieve Ontology Version

Retreives details on a single ontology version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |

### Return type

[**crate::models::RetrieveOntologyVersion200Response**](RetrieveOntologyVersion_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rollback_ontology_version

> rollback_ontology_version(container_id, version_id)
Rollback Ontology Version

Rolls back the ontology to the selected version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**version_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_ontology_version_for_approval

> send_ontology_version_for_approval(container_id, ontology_version_id)
Send Ontology Version for Approval

Sends an ontology version to be approved by a container admin

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**ontology_version_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_container_active

> crate::models::Generic200Response set_container_active(container_id)
Set Container Active

Unarchives a Container. This is the only way to update this value of a container via API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_container

> crate::models::UpdateContainerResponse update_container(container_id, body)
Update Container

Updates the container. This will fail if a container already exists with the proposed updated name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**body** | [**UpdateContainerRequest**](UpdateContainerRequest.md) |  | [required] |

### Return type

[**crate::models::UpdateContainerResponse**](UpdateContainerResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_container_import

> crate::models::ContainerImportUpdateResponse update_container_import(container_id, name, description, data_versioning_enabled, path, file)
Update Container Import

Updates an existing container via an ontology file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**name** | **String** |  | [required] |
**description** | **String** |  | [required] |
**data_versioning_enabled** | **bool** |  | [required] |
**path** | Option<**String**> |  |  |
**file** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**crate::models::ContainerImportUpdateResponse**](ContainerImportUpdateResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

