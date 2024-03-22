# PolicyStoreDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<[**models::PolicyStoreTypes**](PolicyStoreTypes.md)> | the type of policy store, currently only OPA is officially supported | [optional][default to Opa]
**url** | **String** | the url that OPA can be found in. if localhost is the host - it means OPA is on the same hostname as OPAL client. | 
**token** | Option<**String**> | optional access token required by the policy store | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


