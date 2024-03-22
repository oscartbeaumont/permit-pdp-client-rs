# \CallbacksApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_callback_by_key_callbacks_key_delete**](CallbacksApi.md#get_callback_by_key_callbacks_key_delete) | **DELETE** /callbacks/{key} | Get Callback By Key
[**get_callback_by_key_callbacks_key_get**](CallbacksApi.md#get_callback_by_key_callbacks_key_get) | **GET** /callbacks/{key} | Get Callback By Key
[**list_callbacks_callbacks_get**](CallbacksApi.md#list_callbacks_callbacks_get) | **GET** /callbacks | List Callbacks
[**register_callback_callbacks_post**](CallbacksApi.md#register_callback_callbacks_post) | **POST** /callbacks | Register Callback



## get_callback_by_key_callbacks_key_delete

> get_callback_by_key_callbacks_key_delete(key, authorization)
Get Callback By Key

unregisters a callback identified by its key (if such callback is indeed registered).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**authorization** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_callback_by_key_callbacks_key_get

> models::CallbackEntry get_callback_by_key_callbacks_key_get(key, authorization)
Get Callback By Key

get a callback by its key (if such callback is indeed registered).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**authorization** | Option<**String**> |  |  |

### Return type

[**models::CallbackEntry**](CallbackEntry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_callbacks_callbacks_get

> Vec<models::CallbackEntry> list_callbacks_callbacks_get(authorization)
List Callbacks

list all the callbacks currently registered by OPAL client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | Option<**String**> |  |  |

### Return type

[**Vec<models::CallbackEntry>**](CallbackEntry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_callback_callbacks_post

> models::CallbackEntry register_callback_callbacks_post(callback_entry, authorization)
Register Callback

register a new callback by OPAL client, to be called on OPA state updates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**callback_entry** | [**CallbackEntry**](CallbackEntry.md) |  | [required] |
**authorization** | Option<**String**> |  |  |

### Return type

[**models::CallbackEntry**](CallbackEntry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

