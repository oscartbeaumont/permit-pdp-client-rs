# \AuthorizationApiApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_permissions_user_permissions_post**](AuthorizationApiApi.md#get_user_permissions_user_permissions_post) | **POST** /user-permissions | Get User Permissions
[**get_user_tenants_user_tenants_post**](AuthorizationApiApi.md#get_user_tenants_user_tenants_post) | **POST** /user-tenants | Get User Tenants
[**is_allowed_all_tenants_allowed_all_tenants_post**](AuthorizationApiApi.md#is_allowed_all_tenants_allowed_all_tenants_post) | **POST** /allowed/all-tenants | Is Allowed All Tenants
[**is_allowed_allowed_post**](AuthorizationApiApi.md#is_allowed_allowed_post) | **POST** /allowed | Is Allowed
[**is_allowed_bulk_allowed_bulk_post**](AuthorizationApiApi.md#is_allowed_bulk_allowed_bulk_post) | **POST** /allowed/bulk | Is Allowed Bulk
[**is_allowed_kong_kong_post**](AuthorizationApiApi.md#is_allowed_kong_kong_post) | **POST** /kong | Is Allowed Kong
[**is_allowed_url_allowed_url_post**](AuthorizationApiApi.md#is_allowed_url_allowed_url_post) | **POST** /allowed_url | Is Allowed Url



## get_user_permissions_user_permissions_post

> std::collections::HashMap<String, models::UserPermissionsResult> get_user_permissions_user_permissions_post(user_permissions_query, authorization, x_permit_sdk_language)
Get User Permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_permissions_query** | [**UserPermissionsQuery**](UserPermissionsQuery.md) |  | [required] |
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |
**x_permit_sdk_language** | Option<**String**> |  |  |

### Return type

[**std::collections::HashMap<String, models::UserPermissionsResult>**](_UserPermissionsResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_tenants_user_tenants_post

> Vec<models::TenantDetails> get_user_tenants_user_tenants_post(user_tenants_query, authorization, x_permit_sdk_language)
Get User Tenants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_tenants_query** | [**UserTenantsQuery**](UserTenantsQuery.md) |  | [required] |
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |
**x_permit_sdk_language** | Option<**String**> |  |  |

### Return type

[**Vec<models::TenantDetails>**](_TenantDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_allowed_all_tenants_allowed_all_tenants_post

> models::AllTenantsAuthorizationResult is_allowed_all_tenants_allowed_all_tenants_post(authorization_query, authorization, x_permit_sdk_language)
Is Allowed All Tenants

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization_query** | [**AuthorizationQuery**](AuthorizationQuery.md) |  | [required] |
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |
**x_permit_sdk_language** | Option<**String**> |  |  |

### Return type

[**models::AllTenantsAuthorizationResult**](AllTenantsAuthorizationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_allowed_allowed_post

> models::AuthorizationResult is_allowed_allowed_post(query, authorization, x_permit_sdk_language)
Is Allowed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | [**Query**](Query.md) |  | [required] |
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |
**x_permit_sdk_language** | Option<**String**> |  |  |

### Return type

[**models::AuthorizationResult**](AuthorizationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_allowed_bulk_allowed_bulk_post

> models::BulkAuthorizationResult is_allowed_bulk_allowed_bulk_post(authorization_query, authorization, x_permit_sdk_language)
Is Allowed Bulk

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization_query** | [**Vec<models::AuthorizationQuery>**](AuthorizationQuery.md) |  | [required] |
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |
**x_permit_sdk_language** | Option<**String**> |  |  |

### Return type

[**models::BulkAuthorizationResult**](BulkAuthorizationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_allowed_kong_kong_post

> models::KongAuthorizationResult is_allowed_kong_kong_post(kong_authorization_query)
Is Allowed Kong

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kong_authorization_query** | [**KongAuthorizationQuery**](KongAuthorizationQuery.md) |  | [required] |

### Return type

[**models::KongAuthorizationResult**](KongAuthorizationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_allowed_url_allowed_url_post

> models::AuthorizationResult is_allowed_url_allowed_url_post(url_authorization_query, authorization, x_permit_sdk_language)
Is Allowed Url

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_authorization_query** | [**UrlAuthorizationQuery**](UrlAuthorizationQuery.md) |  | [required] |
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |
**x_permit_sdk_language** | Option<**String**> |  |  |

### Return type

[**models::AuthorizationResult**](AuthorizationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

