# \PropertiesApi

All URIs are relative to *http://127.0.0.1:31009*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_property**](PropertiesApi.md#create_property) | **POST** /v1/spaces/{space_id}/properties | Create property
[**delete_property**](PropertiesApi.md#delete_property) | **DELETE** /v1/spaces/{space_id}/properties/{property_id} | Delete property
[**get_property**](PropertiesApi.md#get_property) | **GET** /v1/spaces/{space_id}/properties/{property_id} | Get property
[**list_properties**](PropertiesApi.md#list_properties) | **GET** /v1/spaces/{space_id}/properties | List properties
[**update_property**](PropertiesApi.md#update_property) | **PATCH** /v1/spaces/{space_id}/properties/{property_id} | Update property



## create_property

> models::ApimodelPropertyResponse create_property(anytype_version, space_id, apimodel_create_property_request)
Create property

⚠ Warning: Properties are experimental and may change in the next update. ⚠ Creates a new property in the specified space using a JSON payload. The creation process is subject to rate limiting. The payload must include property details such as the name and format. The endpoint then returns the full property data, ready for further interactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to create the property in; must be retrieved from ListSpaces endpoint | [required] |
**apimodel_create_property_request** | [**ApimodelCreatePropertyRequest**](ApimodelCreatePropertyRequest.md) | The property to create | [required] |

### Return type

[**models::ApimodelPropertyResponse**](apimodel.PropertyResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_property

> models::ApimodelPropertyResponse delete_property(anytype_version, space_id, property_id)
Delete property

⚠ Warning: Properties are experimental and may change in the next update. ⚠ This endpoint “deletes” a property by marking it as archived. The deletion process is performed safely and is subject to rate limiting. It returns the property’s details after it has been archived. Proper error handling is in place for situations such as when the property isn’t found or the deletion cannot be performed because of permission issues.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to which the property belongs; must be retrieved from ListSpaces endpoint | [required] |
**property_id** | **String** | The ID of the property to delete; must be retrieved from ListProperties endpoint or obtained from response context | [required] |

### Return type

[**models::ApimodelPropertyResponse**](apimodel.PropertyResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_property

> models::ApimodelPropertyResponse get_property(anytype_version, space_id, property_id)
Get property

⚠ Warning: Properties are experimental and may change in the next update. ⚠ Fetches detailed information about one specific property by its ID. This includes the property’s unique identifier, name and format. This detailed view assists clients in showing property options to users and in guiding the user interface (such as displaying appropriate input fields or selection options).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to which the property belongs; must be retrieved from ListSpaces endpoint | [required] |
**property_id** | **String** | The ID of the property to retrieve; must be retrieved from ListProperties endpoint or obtained from response context | [required] |

### Return type

[**models::ApimodelPropertyResponse**](apimodel.PropertyResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_properties

> models::PaginationPaginatedResponseApimodelProperty list_properties(anytype_version, space_id, offset, limit)
List properties

⚠ Warning: Properties are experimental and may change in the next update. ⚠ Retrieves a paginated list of properties available within a specific space. Each property record includes its unique identifier, name and format. This information is essential for clients to understand the available properties for filtering or creating objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to list properties for; must be retrieved from ListSpaces endpoint | [required] |
**offset** | Option<**i32**> | The number of items to skip before starting to collect the result set |  |[default to 0]
**limit** | Option<**i32**> | The number of items to return |  |[default to 100]

### Return type

[**models::PaginationPaginatedResponseApimodelProperty**](pagination.PaginatedResponse-apimodel_Property.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_property

> models::ApimodelPropertyResponse update_property(anytype_version, space_id, property_id, apimodel_update_property_request)
Update property

⚠ Warning: Properties are experimental and may change in the next update. ⚠ This endpoint updates an existing property in the specified space using a JSON payload. The update process is subject to rate limiting. The payload must include the name to be updated. The endpoint then returns the full property data, ready for further interactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to which the property belongs; must be retrieved from ListSpaces endpoint | [required] |
**property_id** | **String** | The ID of the property to update; must be retrieved from ListProperties endpoint or obtained from response context | [required] |
**apimodel_update_property_request** | [**ApimodelUpdatePropertyRequest**](ApimodelUpdatePropertyRequest.md) | The property to update | [required] |

### Return type

[**models::ApimodelPropertyResponse**](apimodel.PropertyResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

