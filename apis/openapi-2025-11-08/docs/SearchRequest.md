# SearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filters** | Option<[**models::FilterExpression**](FilterExpression.md)> |  | [optional]
**query** | Option<**String**> | The text to search within object names and content; use types field for type filtering | [optional]
**sort** | Option<[**models::SortOptions**](SortOptions.md)> |  | [optional]
**types** | Option<**Vec<String>**> | The types of objects to include in results (e.g., \"page\", \"task\", \"bookmark\"); see ListTypes endpoint for valid values | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


