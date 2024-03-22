# HttpFetcherConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fetcher** | Option<**String**> | indicates to OPAL client that it should use a custom FetcherProvider to fetch the data | [optional]
**headers** | Option<[**serde_json::Value**](.md)> |  | [optional]
**is_json** | Option<**bool**> |  | [optional][default to true]
**process_data** | Option<**bool**> |  | [optional][default to true]
**method** | Option<[**models::HttpMethods**](HttpMethods.md)> |  | [optional][default to Get]
**data** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


