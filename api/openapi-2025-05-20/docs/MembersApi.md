# \MembersApi

All URIs are relative to *http://127.0.0.1:31009*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_member**](MembersApi.md#get_member) | **GET** /v1/spaces/{space_id}/members/{member_id} | Get member
[**list_members**](MembersApi.md#list_members) | **GET** /v1/spaces/{space_id}/members | List members



## get_member

> models::ApimodelPeriodMemberResponse get_member(anytype_version, space_id, member_id)
Get member

Fetches detailed information about a single member within a space. The endpoint returns the member’s identifier, name, icon, identity, global name, status and role. The member_id path parameter can be provided as either the member's ID (starting  with `_participant`) or the member's identity. This is useful for user profile pages, permission management, and displaying member-specific information in collaborative environments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to get the member from; must be retrieved from ListSpaces endpoint | [required] |
**member_id** | **String** | Member ID or Identity; must be retrieved from ListMembers endpoint or obtained from response context | [required] |

### Return type

[**models::ApimodelPeriodMemberResponse**](apimodel.MemberResponse.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_members

> models::PaginationPeriodPaginatedResponseApimodelMember list_members(anytype_version, space_id, offset, limit)
List members

Returns a paginated list of members belonging to the specified space. Each member record includes the member’s profile ID, name, icon (which may be derived from an emoji or image), network identity, global name, status (e.g. joining, active) and role (e.g. Viewer, Editor, Owner). This endpoint supports collaborative features by allowing clients to show who is in a space and manage access rights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anytype_version** | **String** | The version of the API to use | [required] |[default to 2025-05-20]
**space_id** | **String** | The ID of the space to list members for; must be retrieved from ListSpaces endpoint | [required] |
**offset** | Option<**i32**> | The number of items to skip before starting to collect the result set |  |[default to 0]
**limit** | Option<**i32**> | The number of items to return |  |[default to 100]

### Return type

[**models::PaginationPeriodPaginatedResponseApimodelMember**](pagination.PaginatedResponse-apimodel_Member.md)

### Authorization

[bearerauth](../README.md#bearerauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

