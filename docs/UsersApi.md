# \UsersApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_container_invite**](UsersApi.md#accept_container_invite) | **GET** /users/invite | Accept Container Invite
[**assign_user_role**](UsersApi.md#assign_user_role) | **POST** /containers/{container_id}/users/roles | Assign User Role
[**create_service_user**](UsersApi.md#create_service_user) | **POST** /containers/{container_id}/service-users | Create Service User
[**create_service_user_key_pair**](UsersApi.md#create_service_user_key_pair) | **POST** /containers/{container_id}/service-users/{service_user_id}/keys | Create Service User KeyPair
[**delete_service_user**](UsersApi.md#delete_service_user) | **DELETE** /containers/{container_id}/service-users/{service_user_id} | Delete Service User
[**delete_service_user_key_pair**](UsersApi.md#delete_service_user_key_pair) | **DELETE** /containers/{container_id}/service-users/{service_user_id}/keys/{key_id} | Delete Service User KeyPair
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /users/{user_id} | Delete User
[**get_service_user_permissions**](UsersApi.md#get_service_user_permissions) | **GET** /containers/{container_id}/service-users/{service_user_id}/permissions | Get Service User Permissions
[**invite_user_to_container**](UsersApi.md#invite_user_to_container) | **POST** /containers/{container_id}/users/invite | Invite User to Container
[**list_invited_users_for_container**](UsersApi.md#list_invited_users_for_container) | **GET** /containers/{container_id}/users/invite | List Invited Users for Container
[**list_outstanding_invites**](UsersApi.md#list_outstanding_invites) | **GET** /users/invites | List Outstanding Invites
[**list_service_user_key_pairs**](UsersApi.md#list_service_user_key_pairs) | **GET** /containers/{container_id}/service-users/{service_user_id}/keys | List Service User KeyPairs
[**list_service_users**](UsersApi.md#list_service_users) | **GET** /containers/{container_id}/service-users | List Service Users
[**list_user_permissions**](UsersApi.md#list_user_permissions) | **GET** /users/permissions | List User Permissions
[**list_users**](UsersApi.md#list_users) | **GET** /users | List Users
[**list_users_for_container**](UsersApi.md#list_users_for_container) | **GET** /containers/{container_id}/users | List Users for Container
[**list_users_roles**](UsersApi.md#list_users_roles) | **GET** /containers/{container_id}/users/{user_id}/roles | List User's Roles
[**retrieve_user**](UsersApi.md#retrieve_user) | **GET** /containers/{container_id}/users/{user_id} | Retrieve User
[**set_service_user_permissions**](UsersApi.md#set_service_user_permissions) | **PUT** /containers/{container_id}/service-users/{service_user_id}/permissions | Set Service User Permissions
[**update_user**](UsersApi.md#update_user) | **PUT** /users/{user_id} | Update User



## accept_container_invite

> crate::models::Generic200Response accept_container_invite(token)
Accept Container Invite

Accepts a container invite for the current user. The token received in the container invite previously must be attached to this request as a query parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | the token supplied in the container invite | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assign_user_role

> crate::models::Generic200Response assign_user_role(container_id, body)
Assign User Role

Assign a role to a user, roles must consist of role name and domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**body** | [**AssignRoleRequest**](AssignRoleRequest.md) |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_service_user

> create_service_user(container_id, create_service_user)
Create Service User

Creates a new service user for container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**create_service_user** | Option<[**CreateServiceUser**](CreateServiceUser.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_service_user_key_pair

> create_service_user_key_pair(container_id, service_user_id)
Create Service User KeyPair

Creates a new api/secret keypair. This will return the secret as well - this is the only time that you will be able to see the secret.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**service_user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_user

> delete_service_user(container_id, service_user_id)
Delete Service User

Deletes a service user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**service_user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_user_key_pair

> delete_service_user_key_pair(container_id, service_user_id, key_id)
Delete Service User KeyPair

Delete a service user keypair.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**service_user_id** | **String** |  | [required] |
**key_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> crate::models::Generic200Response delete_user(user_id)
Delete User

Deletes the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_user_permissions

> get_service_user_permissions(container_id, service_user_id)
Get Service User Permissions

Get service user permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**service_user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite_user_to_container

> crate::models::Generic200Response invite_user_to_container(container_id, container_invite)
Invite User to Container

Create a new user using the username_password identity type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**container_invite** | Option<[**ContainerInvite**](ContainerInvite.md)> |  |  |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_invited_users_for_container

> crate::models::ListContainerInvitesResponse list_invited_users_for_container(container_id)
List Invited Users for Container

List all invitations to container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |

### Return type

[**crate::models::ListContainerInvitesResponse**](ListContainerInvitesResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_outstanding_invites

> crate::models::ListUserInvitesResponse list_outstanding_invites()
List Outstanding Invites

Lists the outstanding container invites for the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListUserInvitesResponse**](ListUserInvitesResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_service_user_key_pairs

> list_service_user_key_pairs(container_id, service_user_id)
List Service User KeyPairs

Lists a service user's api/secret keypairs. This lists only the key, not the secret.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**service_user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_service_users

> crate::models::ListServiceUserResponse list_service_users(container_id)
List Service Users

Retrieve a list of all service users for container

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |

### Return type

[**crate::models::ListServiceUserResponse**](ListServiceUserResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_permissions

> crate::models::ListUserPermissionsResponse list_user_permissions()
List User Permissions

List permissions for the user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListUserPermissionsResponse**](ListUserPermissionsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users

> crate::models::ListUsersResponse list_users(count, load_keys, limit, offset, sort_by, sort_desc)
List Users

List users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**bool**> | boolean indicating if the return value should be a count only |  |
**load_keys** | Option<**bool**> | if supplied, the API keys for the user will also be returned |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**sort_by** | Option<**String**> | column to sort results by |  |
**sort_desc** | Option<**bool**> | boolean indicating if results should be in descending order |  |

### Return type

[**crate::models::ListUsersResponse**](ListUsersResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users_for_container

> crate::models::ListUsersForContainerResponse list_users_for_container(container_id, limit, offset)
List Users for Container

List Users for container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**crate::models::ListUsersForContainerResponse**](ListUsersForContainerResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users_roles

> crate::models::ListUserRoles list_users_roles(container_id, user_id)
List User's Roles

List Users' roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::ListUserRoles**](ListUserRoles.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_user

> crate::models::GetUserResponse retrieve_user(container_id, user_id)
Retrieve User

Retrieve a user by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::GetUserResponse**](GetUserResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_service_user_permissions

> set_service_user_permissions(container_id, service_user_id, set_service_user_permissions_request)
Set Service User Permissions

Set service user permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** |  | [required] |
**service_user_id** | **String** |  | [required] |
**set_service_user_permissions_request** | Option<[**SetServiceUserPermissionsRequest**](SetServiceUserPermissionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::Generic200Response update_user(user_id, user)
Update User

Updates the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**user** | Option<[**User**](User.md)> |  |  |

### Return type

[**crate::models::Generic200Response**](Generic200Response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

