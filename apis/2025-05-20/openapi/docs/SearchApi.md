# \SearchApi

All URIs are relative to *http://127.0.0.1:31009*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_global**](SearchApi.md#search_global) | **POST** /v1/search | Search objects across all spaces
[**search_space**](SearchApi.md#search_space) | **POST** /v1/spaces/{space_id}/search | Search objects within a space



## search_global

> models::PaginationPeriodPaginatedResponseApimodelObject search_global(anytype_version, apimodel_period_search_request, offset, limit)
Search objects across all spaces

Executes a global search over all spaces accessible to the authenticated user. The request body must specify the `query` text (currently matching only name and snippet of an object), optional filters on types (e.g., \"page\", \"task\"), and sort directives (default: descending by last modified date). Pagination is controlled via `offset` and `limit` query parameters to facilitate lazy loading in client UIs. The response returns a unified list of matched objects with their metadata and properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**apimodel_period_search_request** | [**ApimodelPeriodSearchRequest**](ApimodelPeriodSearchRequest.md) | The search parameters used to filter and sort the results | [required] |
**offset** | Option<**i32**> | The number of items to skip before starting to collect the result set |  |[default to 0]
**limit** | Option<**i32**> | The number of items to return |  |[default to 100]

### Return type

[**models::PaginationPeriodPaginatedResponseApimodelObject**](pagination.PaginatedResponse-apimodel_Object.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_space

> models::PaginationPeriodPaginatedResponseApimodelObject search_space(anytype_version, space_id, apimodel_period_search_request, offset, limit)
Search objects within a space

Performs a search within a single space (specified by the `space_id` path parameter). Like the global search, it accepts pagination parameters and a JSON payload containing the search `query`, `types`, and sorting preferences. The search is limited to the provided space and returns a list of objects that match the query. This allows clients to implement space‑specific filtering without having to process extraneous results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to search in; must be retrieved from ListSpaces endpoint | [required] |
**apimodel_period_search_request** | [**ApimodelPeriodSearchRequest**](ApimodelPeriodSearchRequest.md) | The search parameters used to filter and sort the results | [required] |
**offset** | Option<**i32**> | The number of items to skip before starting to collect the result set |  |[default to 0]
**limit** | Option<**i32**> | The number of items to return |  |[default to 100]

### Return type

[**models::PaginationPeriodPaginatedResponseApimodelObject**](pagination.PaginatedResponse-apimodel_Object.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

