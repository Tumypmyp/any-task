# \AuthApi

All URIs are relative to *http://127.0.0.1:31009*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](AuthApi.md#create_api_key) | **POST** /v1/auth/api_keys | Create API Key
[**create_auth_challenge**](AuthApi.md#create_auth_challenge) | **POST** /v1/auth/challenges | Create Challenge



## create_api_key

> models::CreateApiKeyResponse create_api_key(anytype_version, create_api_key_request)
Create API Key

After receiving a `challenge_id` from the `/v1/auth/challenges` endpoint, the client calls this endpoint to provide the corresponding 4-digit code along with the challenge ID. The endpoint verifies that the challenge solution is correct and, if it is, returns an `api_key`. This endpoint is central to the authentication process, as it validates the user's identity and issues a key that can be used for further interactions with the API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-11-08]
**create_api_key_request** | [**CreateApiKeyRequest**](CreateApiKeyRequest.md) | The request body containing the challenge ID and code | [required] |

### Return type

[**models::CreateApiKeyResponse**](CreateApiKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_auth_challenge

> models::CreateChallengeResponse create_auth_challenge(anytype_version, create_challenge_request)
Create Challenge

Generates a one-time authentication challenge for granting API access to the user's vault. Upon providing a valid `app_name`, the server issues a unique `challenge_id` and displays a 4-digit code within the Anytype Desktop. The `challenge_id` must then be used with the `/v1/auth/api_keys` endpoint to solve the challenge and retrieve an authentication token. This mechanism ensures that only trusted applications and authorized users gain access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-11-08]
**create_challenge_request** | [**CreateChallengeRequest**](CreateChallengeRequest.md) | The request body containing the app name | [required] |

### Return type

[**models::CreateChallengeResponse**](CreateChallengeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

