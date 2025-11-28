# \TagsApi

All URIs are relative to *http://127.0.0.1:31009*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tag**](TagsApi.md#create_tag) | **POST** /v1/spaces/{space_id}/properties/{property_id}/tags | Create tag
[**delete_tag**](TagsApi.md#delete_tag) | **DELETE** /v1/spaces/{space_id}/properties/{property_id}/tags/{tag_id} | Delete tag
[**get_tag**](TagsApi.md#get_tag) | **GET** /v1/spaces/{space_id}/properties/{property_id}/tags/{tag_id} | Get tag
[**list_tags**](TagsApi.md#list_tags) | **GET** /v1/spaces/{space_id}/properties/{property_id}/tags | List tags
[**update_tag**](TagsApi.md#update_tag) | **PATCH** /v1/spaces/{space_id}/properties/{property_id}/tags/{tag_id} | Update tag



## create_tag

> models::ApimodelTagResponse create_tag(anytype_version, space_id, property_id, apimodel_create_tag_request)
Create tag

This endpoint creates a new tag for a given property id in a space. The creation process is subject to rate limiting. The tag is identified by its unique identifier within the specified space. The request must include the tag's name and color. The response includes the tag's details such as its ID, name, and color. This is useful for clients when users want to add new tag options to a property.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to create the tag in; must be retrieved from ListSpaces endpoint | [required] |
**property_id** | **String** | The ID of the property to create the tag for; must be retrieved from ListProperties endpoint or obtained from response context | [required] |
**apimodel_create_tag_request** | [**ApimodelCreateTagRequest**](ApimodelCreateTagRequest.md) | The tag to create | [required] |

### Return type

[**models::ApimodelTagResponse**](apimodel.TagResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tag

> models::ApimodelTagResponse delete_tag(anytype_version, space_id, property_id, tag_id)
Delete tag

This endpoint “deletes” a tag by marking it as archived. The deletion process is performed safely and is subject to rate limiting. It returns the tag’s details after it has been archived. Proper error handling is in place for situations such as when the tag isn’t found or the deletion cannot be performed because of permission issues.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to delete the tag from; must be retrieved from ListSpaces endpoint | [required] |
**property_id** | **String** | The ID of the property to delete the tag for; must be retrieved from ListProperties endpoint or obtained from response context | [required] |
**tag_id** | **String** | The ID of the tag to delete; must be retrieved from ListTags endpoint or obtained from response context | [required] |

### Return type

[**models::ApimodelTagResponse**](apimodel.TagResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag

> models::ApimodelTagResponse get_tag(anytype_version, space_id, property_id, tag_id)
Get tag

This endpoint retrieves a tag for a given property id. The tag is identified by its unique identifier within the specified space. The response includes the tag's details such as its ID, name, and color. This is useful for clients to display or when editing a specific tag option.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to retrieve the tag from; must be retrieved from ListSpaces endpoint | [required] |
**property_id** | **String** | The ID of the property to retrieve the tag for; must be retrieved from ListProperties endpoint or obtained from response context | [required] |
**tag_id** | **String** | The ID of the tag to retrieve; must be retrieved from ListTags endpoint or obtained from response context | [required] |

### Return type

[**models::ApimodelTagResponse**](apimodel.TagResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tags

> models::PaginationPaginatedResponseApimodelTag list_tags(anytype_version, space_id, property_id)
List tags

This endpoint retrieves a paginated list of tags available for a specific property within a space. Each tag record includes its unique identifier, name, and color. This information is essential for clients to display select or multi-select options to users when they are creating or editing objects. The endpoint also supports pagination through offset and limit parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to list tags for; must be retrieved from ListSpaces endpoint | [required] |
**property_id** | **String** | The ID of the property to list tags for; must be retrieved from ListProperties endpoint or obtained from response context | [required] |

### Return type

[**models::PaginationPaginatedResponseApimodelTag**](pagination.PaginatedResponse-apimodel_Tag.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tag

> models::ApimodelTagResponse update_tag(anytype_version, space_id, property_id, tag_id, apimodel_update_tag_request)
Update tag

This endpoint updates a tag for a given property id in a space. The update process is subject to rate limiting. The tag is identified by its unique identifier within the specified space. The request must include the tag's name and color. The response includes the tag's details such as its ID, name, and color. This is useful for clients when users want to edit existing tags for a property.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to update the tag in; must be retrieved from ListSpaces endpoint | [required] |
**property_id** | **String** | The ID of the property to update the tag for; must be retrieved from ListProperties endpoint or obtained from response context | [required] |
**tag_id** | **String** | The ID of the tag to update; must be retrieved from ListTags endpoint or obtained from response context | [required] |
**apimodel_update_tag_request** | [**ApimodelUpdateTagRequest**](ApimodelUpdateTagRequest.md) | The tag to update | [required] |

### Return type

[**models::ApimodelTagResponse**](apimodel.TagResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

