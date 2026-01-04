# \ObjectsApi

All URIs are relative to *http://127.0.0.1:31009*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_object**](ObjectsApi.md#create_object) | **POST** /v1/spaces/{space_id}/objects | Create object
[**delete_object**](ObjectsApi.md#delete_object) | **DELETE** /v1/spaces/{space_id}/objects/{object_id} | Delete object
[**get_object**](ObjectsApi.md#get_object) | **GET** /v1/spaces/{space_id}/objects/{object_id} | Get object
[**list_objects**](ObjectsApi.md#list_objects) | **GET** /v1/spaces/{space_id}/objects | List objects
[**update_object**](ObjectsApi.md#update_object) | **PATCH** /v1/spaces/{space_id}/objects/{object_id} | Update object



## create_object

> models::ObjectResponse create_object(anytype_version, space_id, create_object_request)
Create object

Creates a new object in the specified space using a JSON payload. The creation process is subject to rate limiting. The payload must include key details such as the object name, icon, description, body content (which may support Markdown), source URL (required for bookmark objects), template identifier, and the type_key (which is the non-unique identifier of the type of object to create). Post-creation, additional operations (like setting featured properties or fetching bookmark metadata) may occur. The endpoint then returns the full object data, ready for further interactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-11-08]
**space_id** | **String** | The ID of the space in which to create the object; must be retrieved from ListSpaces endpoint | [required] |
**create_object_request** | [**CreateObjectRequest**](CreateObjectRequest.md) | The object to create | [required] |

### Return type

[**models::ObjectResponse**](ObjectResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_object

> models::ObjectResponse delete_object(anytype_version, space_id, object_id)
Delete object

This endpoint “deletes” an object by marking it as archived. The deletion process is performed safely and is subject to rate limiting. It returns the object’s details after it has been archived. Proper error handling is in place for situations such as when the object isn’t found or the deletion cannot be performed because of permission issues.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-11-08]
**space_id** | **String** | The ID of the space in which the object exists; must be retrieved from ListSpaces endpoint | [required] |
**object_id** | **String** | The ID of the object to delete; must be retrieved from ListObjects, SearchSpace or GlobalSearch endpoints or obtained from response context | [required] |

### Return type

[**models::ObjectResponse**](ObjectResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object

> models::ObjectResponse get_object(anytype_version, space_id, object_id, format)
Get object

Fetches the full details of a single object identified by the object ID within the specified space. The response includes not only basic metadata (ID, name, icon, type) but also the complete set of blocks (which may include text, files, properties and dataviews) and extra details (such as timestamps and linked member information). This endpoint is essential when a client needs to render or edit the full object view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-11-08]
**space_id** | **String** | The ID of the space in which the object exists; must be retrieved from ListSpaces endpoint | [required] |
**object_id** | **String** | The ID of the object to retrieve; must be retrieved from ListObjects, SearchSpace or GlobalSearch endpoints or obtained from response context | [required] |
**format** | Option<**String**> | The format to return the object body in |  |[default to "md"]

### Return type

[**models::ObjectResponse**](ObjectResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_objects

> models::PaginatedResponseObject list_objects(anytype_version, space_id, offset, limit)
List objects

Retrieves a paginated list of objects in the given space. The endpoint takes query parameters for pagination (offset and limit) and returns detailed data about each object including its ID, name, icon, type information, a snippet of the content (if applicable), layout, space ID, blocks and details. It is intended for building views where users can see all objects in a space at a glance. Supports dynamic filtering via query parameters (e.g., ?done=false, ?created_date[gte]=2024-01-01, ?tags[in]=urgent,important). For select/multi_select properties you can use either tag keys or tag IDs, for object properties use object IDs. See FilterCondition enum for available conditions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-11-08]
**space_id** | **String** | The ID of the space in which to list objects; must be retrieved from ListSpaces endpoint | [required] |
**offset** | Option<**i32**> | The number of items to skip before starting to collect the result set |  |[default to 0]
**limit** | Option<**i32**> | The number of items to return |  |[default to 100]

### Return type

[**models::PaginatedResponseObject**](PaginatedResponse-Object.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_object

> models::ObjectResponse update_object(anytype_version, space_id, object_id, update_object_request)
Update object

This endpoint updates an existing object in the specified space using a JSON payload. The update process is subject to rate limiting. The payload must include the details to be updated. The endpoint then returns the full object data, ready for further interactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-11-08]
**space_id** | **String** | The ID of the space in which the object exists; must be retrieved from ListSpaces endpoint | [required] |
**object_id** | **String** | The ID of the object to update; must be retrieved from ListObjects, SearchSpace or GlobalSearch endpoints or obtained from response context | [required] |
**update_object_request** | [**UpdateObjectRequest**](UpdateObjectRequest.md) | The details of the object to update | [required] |

### Return type

[**models::ObjectResponse**](ObjectResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

