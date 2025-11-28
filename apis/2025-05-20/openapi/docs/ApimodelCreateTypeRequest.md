# ApimodelCreateTypeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**icon** | Option<[**models::ApimodelIcon**](apimodel.Icon.md)> |  | [optional]
**key** | Option<**String**> | The key of the type; should always be snake_case, otherwise it will be converted to snake_case | [optional]
**layout** | [**models::ApimodelTypeLayout**](apimodel.TypeLayout.md) |  | 
**name** | **String** | The name of the type | 
**plural_name** | **String** | The plural name of the type | 
**properties** | Option<[**Vec<models::ApimodelPropertyLink>**](apimodel.PropertyLink.md)> | ⚠ Warning: Properties are experimental and may change in the next update. ⚠ The properties linked to the type | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


