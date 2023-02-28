# \EventsApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_event**](EventsApi.md#create_event) | **POST** /events | Create Event
[**create_event_action**](EventsApi.md#create_event_action) | **POST** /event_actions | Create Event Action
[**delete_event_action**](EventsApi.md#delete_event_action) | **DELETE** /event_actions/{action_id} | Delete Event Action
[**list_event_action_statuses**](EventsApi.md#list_event_action_statuses) | **GET** /event_action_status | List Event Action Statuses
[**list_event_actions**](EventsApi.md#list_event_actions) | **GET** /event_actions | List Event Actions
[**retrieve_event_action**](EventsApi.md#retrieve_event_action) | **GET** /event_actions/{action_id} | Retrieve Event Action
[**retrieve_event_action_status**](EventsApi.md#retrieve_event_action_status) | **GET** /event_action_status/{status_id} | Retrieve Event Action Status
[**update_event_action**](EventsApi.md#update_event_action) | **PUT** /event_actions/{action_id} | Update Event Action
[**update_event_action_status**](EventsApi.md#update_event_action_status) | **PUT** /event_action_status/{status_id} | Update Event Action Status



## create_event

> crate::models::CreateEventResponse create_event(body)
Create Event

Create new event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateEventRequest**](CreateEventRequest.md) |  | [required] |

### Return type

[**crate::models::CreateEventResponse**](CreateEventResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_event_action

> crate::models::CreateEventActionResponse create_event_action(create_event_action_request)
Create Event Action

Create an event action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_event_action_request** | [**CreateEventActionRequest**](CreateEventActionRequest.md) |  | [required] |

### Return type

[**crate::models::CreateEventActionResponse**](CreateEventActionResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_event_action

> crate::models::Generic200Response delete_event_action(action_id)
Delete Event Action

Delete an event action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_event_action_statuses

> crate::models::ListEventActionStatusResponse list_event_action_statuses(event_id)
List Event Action Statuses

List all event action statuses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | Option<**String**> | Filter returned statuses by the event ID |  |

### Return type

[**crate::models::ListEventActionStatusResponse**](ListEventActionStatusResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_event_actions

> crate::models::ListEventActionResponse list_event_actions()
List Event Actions

Lists all event actions

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListEventActionResponse**](ListEventActionResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_event_action

> crate::models::GetEventActionResponse retrieve_event_action(action_id)
Retrieve Event Action

Retrieve an event action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** |  | [required] |

### Return type

[**crate::models::GetEventActionResponse**](GetEventActionResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_event_action_status

> crate::models::GetEventActionStatusResponse retrieve_event_action_status(status_id)
Retrieve Event Action Status

Retrieve an event action status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status_id** | **String** |  | [required] |

### Return type

[**crate::models::GetEventActionStatusResponse**](GetEventActionStatusResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_event_action

> crate::models::UpdateEventActionResponse update_event_action(action_id, active, create_event_action_request)
Update Event Action

Update an event action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** |  | [required] |
**active** | Option<**bool**> | Sets the event action active or inactive |  |
**create_event_action_request** | Option<[**CreateEventActionRequest**](CreateEventActionRequest.md)> |  |  |

### Return type

[**crate::models::UpdateEventActionResponse**](UpdateEventActionResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_event_action_status

> crate::models::UpdateEventActionStatusResponse update_event_action_status(status_id, update_event_action_status_request)
Update Event Action Status

Update an event action status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status_id** | **String** |  | [required] |
**update_event_action_status_request** | Option<[**UpdateEventActionStatusRequest**](UpdateEventActionStatusRequest.md)> |  |  |

### Return type

[**crate::models::UpdateEventActionStatusResponse**](UpdateEventActionStatusResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

