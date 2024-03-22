# \LocalQueriesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_role_by_id_local_roles_role_id_get**](LocalQueriesApi.md#get_role_by_id_local_roles_role_id_get) | **GET** /local/roles/{role_id} | Get Role By Id
[**get_role_by_name_local_roles_by_name_role_name_get**](LocalQueriesApi.md#get_role_by_name_local_roles_by_name_role_name_get) | **GET** /local/roles/by-name/{role_name} | Get Role By Name
[**get_user_local_users_user_id_get**](LocalQueriesApi.md#get_user_local_users_user_id_get) | **GET** /local/users/{user_id} | Get User
[**get_user_permissions_local_users_user_id_permissions_get**](LocalQueriesApi.md#get_user_permissions_local_users_user_id_permissions_get) | **GET** /local/users/{user_id}/permissions | Get User Permissions
[**get_user_roles_local_users_user_id_roles_get**](LocalQueriesApi.md#get_user_roles_local_users_user_id_roles_get) | **GET** /local/users/{user_id}/roles | Get User Roles
[**get_user_tenants_local_users_user_id_tenants_get**](LocalQueriesApi.md#get_user_tenants_local_users_user_id_tenants_get) | **GET** /local/users/{user_id}/tenants | Get User Tenants
[**list_roles_local_roles_get**](LocalQueriesApi.md#list_roles_local_roles_get) | **GET** /local/roles | List Roles
[**list_users_local_users_get**](LocalQueriesApi.md#list_users_local_users_get) | **GET** /local/users | List Users



## get_role_by_id_local_roles_role_id_get

> models::SyncedRole get_role_by_id_local_roles_role_id_get(role_id, authorization)
Get Role By Id

Get role (by the role id) directly from OPA cache.  If role is not found, returns 404. Possible reasons are either:  - role was never created via SDK or via the cloud console. - role was (very) recently created and the policy update did not propagate yet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**models::SyncedRole**](SyncedRole.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_by_name_local_roles_by_name_role_name_get

> models::SyncedRole get_role_by_name_local_roles_by_name_role_name_get(role_name, authorization)
Get Role By Name

Get role (by the role name - case sensitive) directly from OPA cache.  If role is not found, returns 404. Possible reasons are either:  - role with such name was never created via SDK or via the cloud console. - role was (very) recently created and the policy update did not propagate yet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_name** | **String** |  | [required] |
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**models::SyncedRole**](SyncedRole.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_local_users_user_id_get

> models::SyncedUser get_user_local_users_user_id_get(user_id, authorization)
Get User

Get user data directly from OPA cache.  If user does not exist in OPA cache (i.e: not synced), returns 404.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**models::SyncedUser**](SyncedUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_permissions_local_users_user_id_permissions_get

> std::collections::HashMap<String, serde_json::Value> get_user_permissions_local_users_user_id_permissions_get(user_id, authorization)
Get User Permissions

Get roles **assigned to user** directly from OPA cache.  If user does not exist in OPA cache (i.e: not synced), returns 404.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_roles_local_users_user_id_roles_get

> Vec<models::SyncedRole> get_user_roles_local_users_user_id_roles_get(user_id, authorization)
Get User Roles

Get roles **assigned to user** directly from OPA cache.  If user does not exist in OPA cache (i.e: not synced), returns 404.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**Vec<models::SyncedRole>**](SyncedRole.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_tenants_local_users_user_id_tenants_get

> Vec<String> get_user_tenants_local_users_user_id_tenants_get(user_id, authorization)
Get User Tenants

Get tenants **assigned to user** directly from OPA cache. This endpoint only returns tenants that the user **has an assigned role in**. i.e: if the user is assigned to tenant \"tenant1\" but has no roles in that tenant, \"tenant1\" will not be returned by this endpoint.  If user does not exist in OPA cache (i.e: not synced), returns 404.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_roles_local_roles_get

> Vec<models::SyncedRole> list_roles_local_roles_get(authorization)
List Roles

Get all roles stored in OPA cache.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**Vec<models::SyncedRole>**](SyncedRole.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users_local_users_get

> Vec<models::SyncedUser> list_users_local_users_get(authorization)
List Users

Get all users stored in OPA cache.  Be advised, if you have many (i.e: few hundreds or more) users this query might be expensive latency-wise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**Vec<models::SyncedUser>**](SyncedUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

