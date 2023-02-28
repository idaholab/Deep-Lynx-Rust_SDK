# \AuthenticationApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**exchange_o_auth_token**](AuthenticationApi.md#exchange_o_auth_token) | **POST** /oauth/exchange | Exchange OAuth Token
[**retrieve_o_auth_token**](AuthenticationApi.md#retrieve_o_auth_token) | **GET** /oauth/token | Retrieve OAuth Token
[**rsa_cancel**](AuthenticationApi.md#rsa_cancel) | **POST** /rsa/cancel | RSA Cancel
[**rsa_initialize**](AuthenticationApi.md#rsa_initialize) | **POST** /rsa/initialize | RSA Initialize
[**rsa_status**](AuthenticationApi.md#rsa_status) | **POST** /rsa/status | RSA Status
[**rsa_verify**](AuthenticationApi.md#rsa_verify) | **POST** /rsa/verify | RSA Verify



## exchange_o_auth_token

> String exchange_o_auth_token(token_exchange_request)
Exchange OAuth Token

Exchanges credentials for a JSON Web Token (JWT). Multiple authentication flows are supported, see Deep Lynx documentation for details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_exchange_request** | Option<[**TokenExchangeRequest**](TokenExchangeRequest.md)> |  |  |

### Return type

**String**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_o_auth_token

> String retrieve_o_auth_token(x_api_key, x_api_secret, x_api_expiry)
Retrieve OAuth Token

Returns an OAuth token. The API key and secret must be supplied.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** | The API key | [required] |
**x_api_secret** | **String** | The API secret | [required] |
**x_api_expiry** | Option<**String**> | The API expiry date |  |

### Return type

**String**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rsa_cancel

> crate::models::RsaResponse rsa_cancel(rsa_cancel_request)
RSA Cancel

Cancels an RSA authentication attempt

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rsa_cancel_request** | Option<[**RsaCancelRequest**](RsaCancelRequest.md)> |  |  |

### Return type

[**crate::models::RsaResponse**](RSAResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rsa_initialize

> crate::models::RsaResponse rsa_initialize(rsa_init_request)
RSA Initialize

Used to begin (and optionally complete) an RSA authentication. Either a user's ID may be provided and the SecurID provided in a later `verify` request,  or else the user may provide both the user ID (`subjectName`) and `securID` at once to `initialize` to complete the authentication request.  The `securID` is the combination of the user's memorized token and 6 digit temporary RSA pin (with no spaces or characters between them).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rsa_init_request** | Option<[**RsaInitRequest**](RsaInitRequest.md)> |  |  |

### Return type

[**crate::models::RsaResponse**](RSAResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rsa_status

> crate::models::RsaStatusResponse rsa_status(rsa_status_request)
RSA Status

Returns the status of an RSA authentication attempt

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rsa_status_request** | Option<[**RsaStatusRequest**](RsaStatusRequest.md)> |  |  |

### Return type

[**crate::models::RsaStatusResponse**](RSAStatusResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rsa_verify

> crate::models::RsaResponse rsa_verify(rsa_verify_request)
RSA Verify

Provides RSA with the user's SecurID to complete authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rsa_verify_request** | Option<[**RsaVerifyRequest**](RsaVerifyRequest.md)> |  |  |

### Return type

[**crate::models::RsaResponse**](RSAResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

