# \ListsApi

All URIs are relative to *http://127.0.0.1:31009*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_list_objects**](ListsApi.md#add_list_objects) | **POST** /v1/spaces/{space_id}/lists/{list_id}/objects | Add objects to list
[**get_list_objects**](ListsApi.md#get_list_objects) | **GET** /v1/spaces/{space_id}/lists/{list_id}/views/{view_id}/objects | Get objects in list
[**get_list_views**](ListsApi.md#get_list_views) | **GET** /v1/spaces/{space_id}/lists/{list_id}/views | Get list views
[**remove_list_object**](ListsApi.md#remove_list_object) | **DELETE** /v1/spaces/{space_id}/lists/{list_id}/objects/{object_id} | Remove object from list



## add_list_objects

> String add_list_objects(anytype_version, space_id, list_id, apimodel_add_objects_to_list_request)
Add objects to list

Adds one or more objects to a specific list (collection only) by submitting a JSON array of object IDs. Upon success, the endpoint returns a confirmation message. This endpoint is vital for building user interfaces that allow drag‑and‑drop or multi‑select additions to collections, enabling users to dynamically manage their collections without needing to modify the underlying object data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to which the list belongs; must be retrieved from ListSpaces endpoint | [required] |
**list_id** | **String** | The ID of the list to which objects will be added; must be retrieved from SearchSpace endpoint with types: ['collection', 'set'] | [required] |
**apimodel_add_objects_to_list_request** | [**ApimodelAddObjectsToListRequest**](ApimodelAddObjectsToListRequest.md) | The list of object IDs to add to the list; must be retrieved from SearchSpace or GlobalSearch endpoints or obtained from response context | [required] |

### Return type

**String**

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list_objects

> models::PaginationPaginatedResponseApimodelObject get_list_objects(anytype_version, space_id, list_id, view_id, offset, limit)
Get objects in list

Returns a paginated list of objects associated with a specific list (query or collection) within a space. When a view ID is provided, the objects are filtered and sorted according to the view's configuration. If no view ID is specified, all list objects are returned without filtering and sorting. This endpoint helps clients to manage grouped objects (for example, tasks within a list) by returning information for each item of the list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to which the list belongs; must be retrieved from ListSpaces endpoint | [required] |
**list_id** | **String** | The ID of the list to retrieve objects for; must be retrieved from SearchSpace endpoint with types: ['collection', 'set'] | [required] |
**view_id** | **String** | The ID of the view to retrieve objects for; must be retrieved from ListViews endpoint or omitted if you want to get all objects in the list | [required] |
**offset** | Option<**i32**> | The number of items to skip before starting to collect the result set |  |[default to 0]
**limit** | Option<**i32**> | The number of items to return |  |

### Return type

[**models::PaginationPaginatedResponseApimodelObject**](pagination.PaginatedResponse-apimodel_Object.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list_views

> models::PaginationPaginatedResponseApimodelView get_list_views(anytype_version, space_id, list_id, offset, limit)
Get list views

Returns a paginated list of views defined for a specific list (query or collection) within a space. Each view includes details such as layout, applied filters, and sorting options, enabling clients to render the list according to user preferences and context. This endpoint is essential for applications that need to display lists in various formats (e.g., grid, table) or with different sorting/filtering criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to which the list belongs; must be retrieved from ListSpaces endpoint | [required] |
**list_id** | **String** | The ID of the list to retrieve views for; must be retrieved from SearchSpace endpoint with types: ['collection', 'set'] | [required] |
**offset** | Option<**i32**> | The number of items to skip before starting to collect the result set |  |[default to 0]
**limit** | Option<**i32**> | The number of items to return |  |

### Return type

[**models::PaginationPaginatedResponseApimodelView**](pagination.PaginatedResponse-apimodel_View.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_list_object

> String remove_list_object(anytype_version, space_id, list_id, object_id)
Remove object from list

Removes a given object from the specified list (collection only) in a space. The endpoint takes the space, list, and object identifiers as path parameters and is subject to rate limiting. It is used for dynamically managing collections without affecting the underlying object data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to which the list belongs; must be retrieved from ListSpaces endpoint | [required] |
**list_id** | **String** | The ID of the list from which the object will be removed; must be retrieved from SearchSpace endpoint with types: ['collection', 'set'] | [required] |
**object_id** | **String** | The ID of the object to remove from the list; must be retrieved from SearchSpace or GlobalSearch endpoints or obtained from response context | [required] |

### Return type

**String**

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

