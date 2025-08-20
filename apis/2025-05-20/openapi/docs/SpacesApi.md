# \SpacesApi

All URIs are relative to *http://127.0.0.1:31009*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_space**](SpacesApi.md#create_space) | **POST** /v1/spaces | Create space
[**get_space**](SpacesApi.md#get_space) | **GET** /v1/spaces/{space_id} | Get space
[**list_spaces**](SpacesApi.md#list_spaces) | **GET** /v1/spaces | List spaces
[**update_space**](SpacesApi.md#update_space) | **PATCH** /v1/spaces/{space_id} | Update space



## create_space

> models::ApimodelPeriodSpaceResponse create_space(anytype_version, apimodel_period_create_space_request)
Create space

Creates a new space based on a supplied name and description in the JSON request body. The endpoint is subject to rate limiting and automatically applies default configurations such as generating a random icon and initializing the workspace with default settings (for example, a default dashboard or home page). On success, the new space’s full metadata is returned, enabling the client to immediately switch context to the new internal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**apimodel_period_create_space_request** | [**ApimodelPeriodCreateSpaceRequest**](ApimodelPeriodCreateSpaceRequest.md) | The space to create | [required] |

### Return type

[**models::ApimodelPeriodSpaceResponse**](apimodel.SpaceResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_space

> models::ApimodelPeriodSpaceResponse get_space(anytype_version, space_id)
Get space

Fetches full details about a single space identified by its space ID. The response includes metadata such as the space name, icon, and various workspace IDs (home, archive, profile, etc.). This detailed view supports use cases such as displaying space-specific settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to retrieve; must be retrieved from ListSpaces endpoint | [required] |

### Return type

[**models::ApimodelPeriodSpaceResponse**](apimodel.SpaceResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_spaces

> models::PaginationPeriodPaginatedResponseApimodelSpace list_spaces(anytype_version, offset, limit)
List spaces

Retrieves a paginated list of all spaces that are accessible by the authenticated user. Each space record contains detailed information such as the space ID, name, icon (derived either from an emoji or image URL), and additional metadata. This endpoint is key to displaying a user’s workspaces.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**offset** | Option<**i32**> | The number of items to skip before starting to collect the result set |  |[default to 0]
**limit** | Option<**i32**> | The number of items to return |  |[default to 100]

### Return type

[**models::PaginationPeriodPaginatedResponseApimodelSpace**](pagination.PaginatedResponse-apimodel_Space.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_space

> models::ApimodelPeriodSpaceResponse update_space(anytype_version, space_id, apimodel_period_update_space_request)
Update space

Updates the name or description of an existing space. The request body should contain the new name and/or description in JSON format. This endpoint is useful for renaming or rebranding a workspace without needing to recreate it. The updated space’s metadata is returned in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to update; must be retrieved from ListSpaces endpoint | [required] |
**apimodel_period_update_space_request** | [**ApimodelPeriodUpdateSpaceRequest**](ApimodelPeriodUpdateSpaceRequest.md) | The space details to update | [required] |

### Return type

[**models::ApimodelPeriodSpaceResponse**](apimodel.SpaceResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

