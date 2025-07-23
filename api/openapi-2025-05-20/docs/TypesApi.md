# \TypesApi

All URIs are relative to *http://127.0.0.1:31009*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_type**](TypesApi.md#create_type) | **POST** /v1/spaces/{space_id}/types | Create type
[**delete_type**](TypesApi.md#delete_type) | **DELETE** /v1/spaces/{space_id}/types/{type_id} | Delete type
[**get_type**](TypesApi.md#get_type) | **GET** /v1/spaces/{space_id}/types/{type_id} | Get type
[**list_types**](TypesApi.md#list_types) | **GET** /v1/spaces/{space_id}/types | List types
[**update_type**](TypesApi.md#update_type) | **PATCH** /v1/spaces/{space_id}/types/{type_id} | Update type



## create_type

> models::ApimodelPeriodTypeResponse create_type(anytype_version, space_id, apimodel_period_create_type_request)
Create type

Creates a new type in the specified space using a JSON payload. The creation process is subject to rate limiting. The payload must include type details such as the name, icon, and layout. The endpoint then returns the full type data, ready to be used for creating objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space in which to create the type; must be retrieved from ListSpaces endpoint | [required] |
**apimodel_period_create_type_request** | [**ApimodelPeriodCreateTypeRequest**](ApimodelPeriodCreateTypeRequest.md) | The type to create | [required] |

### Return type

[**models::ApimodelPeriodTypeResponse**](apimodel.TypeResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_type

> models::ApimodelPeriodTypeResponse delete_type(anytype_version, space_id, type_id)
Delete type

This endpoint “deletes” an type by marking it as archived. The deletion process is performed safely and is subject to rate limiting. It returns the type’s details after it has been archived. Proper error handling is in place for situations such as when the type isn’t found or the deletion cannot be performed because of permission issues.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space from which to delete the type; must be retrieved from ListSpaces endpoint | [required] |
**type_id** | **String** | The ID of the type to delete; must be retrieved from ListTypes endpoint or obtained from response context | [required] |

### Return type

[**models::ApimodelPeriodTypeResponse**](apimodel.TypeResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_type

> models::ApimodelPeriodTypeResponse get_type(anytype_version, space_id, type_id)
Get type

Fetches detailed information about one specific type by its ID. This includes the type’s unique key, name, icon, and layout. This detailed view assists clients in understanding the expected structure and style for objects of that type and in guiding the user interface (such as displaying appropriate icons or layout hints).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space from which to retrieve the type; must be retrieved from ListSpaces endpoint | [required] |
**type_id** | **String** | The ID of the type to retrieve; must be retrieved from ListTypes endpoint or obtained from response context | [required] |

### Return type

[**models::ApimodelPeriodTypeResponse**](apimodel.TypeResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_types

> models::PaginationPeriodPaginatedResponseApimodelType list_types(anytype_version, space_id, offset, limit)
List types

This endpoint retrieves a paginated list of types (e.g. 'Page', 'Note', 'Task') available within the specified space. Each type’s record includes its unique identifier, type key, display name, icon, and layout. While a type's id is truly unique, a type's key can be the same across spaces for known types, e.g. 'page' for 'Page'. Clients use this information when offering choices for object creation or for filtering objects by type through search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to retrieve types from; must be retrieved from ListSpaces endpoint | [required] |
**offset** | Option<**i32**> | The number of items to skip before starting to collect the result set |  |[default to 0]
**limit** | Option<**i32**> | The number of items to return |  |[default to 100]

### Return type

[**models::PaginationPeriodPaginatedResponseApimodelType**](pagination.PaginatedResponse-apimodel_Type.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_type

> models::ApimodelPeriodTypeResponse update_type(anytype_version, space_id, type_id, apimodel_period_update_type_request)
Update type

This endpoint updates an existing type in the specified space using a JSON payload. The update process is subject to rate limiting. The payload must include the name and properties to be updated. The endpoint then returns the full type data, ready for further interactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space in which the type exists; must be retrieved from ListSpaces endpoint | [required] |
**type_id** | **String** | The ID of the type to update; must be retrieved from ListTypes endpoint or obtained from response context | [required] |
**apimodel_period_update_type_request** | [**ApimodelPeriodUpdateTypeRequest**](ApimodelPeriodUpdateTypeRequest.md) | The type details to update | [required] |

### Return type

[**models::ApimodelPeriodTypeResponse**](apimodel.TypeResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

