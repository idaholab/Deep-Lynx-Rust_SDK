# \TasksApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task**](TasksApi.md#create_task) | **POST** /containers/{container_id}/task | Create Task
[**get_task**](TasksApi.md#get_task) | **GET** /containers/{container_id}/task/{task_id} | Get Task
[**list_tasks**](TasksApi.md#list_tasks) | **GET** /containers/{container_id}/task | List Tasks
[**update_task**](TasksApi.md#update_task) | **PUT** /containers/{container_id}/task/{task_id} | Update Task



## create_task

> crate::models::CreateTaskResponse create_task(container_id, task)
Create Task

Creates a new task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**task** | Option<[**Task**](Task.md)> |  |  |

### Return type

[**crate::models::CreateTaskResponse**](CreateTaskResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task

> crate::models::GetTaskResponse get_task(container_id, task_id)
Get Task

Retrieves a specific task by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**task_id** | **String** |  | [required] |

### Return type

[**crate::models::GetTaskResponse**](GetTaskResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tasks

> crate::models::ListTasksResponse list_tasks(container_id)
List Tasks

Lists all tasks with a \"ready\" status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |

### Return type

[**crate::models::ListTasksResponse**](ListTasksResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task

> crate::models::UpdateTaskResponse update_task(container_id, task_id, task)
Update Task

Updates a task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**task_id** | **String** |  | [required] |
**task** | Option<[**Task**](Task.md)> |  |  |

### Return type

[**crate::models::UpdateTaskResponse**](UpdateTaskResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

