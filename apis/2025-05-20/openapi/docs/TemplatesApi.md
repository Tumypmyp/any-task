# \TemplatesApi

All URIs are relative to *http://127.0.0.1:31009*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_template**](TemplatesApi.md#get_template) | **GET** /v1/spaces/{space_id}/types/{type_id}/templates/{template_id} | Get template
[**list_templates**](TemplatesApi.md#list_templates) | **GET** /v1/spaces/{space_id}/types/{type_id}/templates | List templates



## get_template

> models::ApimodelTemplateResponse get_template(anytype_version, space_id, type_id, template_id)
Get template

Fetches full details for one template associated with a particular type in a space. The response provides the template’s identifier, name, icon, and any other relevant metadata. This endpoint is useful when a client needs to preview or apply a template to prefill object creation fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to which the template belongs; must be retrieved from ListSpaces endpoint | [required] |
**type_id** | **String** | The ID of the type to which the template belongs; must be retrieved from ListTypes endpoint or obtained from response context | [required] |
**template_id** | **String** | The ID of the template to retrieve; must be retrieved from ListTemplates endpoint or obtained from response context | [required] |

### Return type

[**models::ApimodelTemplateResponse**](apimodel.TemplateResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_templates

> models::PaginationPaginatedResponseApimodelObject list_templates(anytype_version, space_id, type_id, offset, limit)
List templates

This endpoint returns a paginated list of templates that are associated with a specific type within a space. Templates provide pre‑configured structures for creating new objects. Each template record contains its identifier, name, and icon, so that clients can offer users a selection of templates when creating objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to which the type belongs; must be retrieved from ListSpaces endpoint | [required] |
**type_id** | **String** | The ID of the type to retrieve templates for; must be retrieved from ListTypes endpoint or obtained from response context | [required] |
**offset** | Option<**i32**> | The number of items to skip before starting to collect the result set |  |[default to 0]
**limit** | Option<**i32**> | The number of items to return |  |[default to 100]

### Return type

[**models::PaginationPaginatedResponseApimodelObject**](pagination.PaginatedResponse-apimodel_Object.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

