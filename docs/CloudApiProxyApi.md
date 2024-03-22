# \CloudApiProxyApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**health_opa_healthcheck_healthchecks_opa_healthy_get**](CloudApiProxyApi.md#health_opa_healthcheck_healthchecks_opa_healthy_get) | **GET** /healthchecks/opa/healthy | Proxy healthy healthcheck -  OPAL_OPA_HEALTH_CHECK_POLICY_ENABLED must be set to True
[**ready_opa_healthcheck_healthchecks_opa_ready_get**](CloudApiProxyApi.md#ready_opa_healthcheck_healthchecks_opa_ready_get) | **GET** /healthchecks/opa/ready | Proxy ready healthcheck - OPAL_OPA_HEALTH_CHECK_POLICY_ENABLED must be set to True
[**system_opa_healthcheck_healthchecks_opa_system_get**](CloudApiProxyApi.md#system_opa_healthcheck_healthchecks_opa_system_get) | **GET** /healthchecks/opa/system | Proxy system data -  OPAL_OPA_HEALTH_CHECK_POLICY_ENABLED must be set to True



## health_opa_healthcheck_healthchecks_opa_healthy_get

> serde_json::Value health_opa_healthcheck_healthchecks_opa_healthy_get(authorization)
Proxy healthy healthcheck -  OPAL_OPA_HEALTH_CHECK_POLICY_ENABLED must be set to True

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ready_opa_healthcheck_healthchecks_opa_ready_get

> serde_json::Value ready_opa_healthcheck_healthchecks_opa_ready_get(authorization)
Proxy ready healthcheck - OPAL_OPA_HEALTH_CHECK_POLICY_ENABLED must be set to True

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_opa_healthcheck_healthchecks_opa_system_get

> serde_json::Value system_opa_healthcheck_healthchecks_opa_system_get(authorization)
Proxy system data -  OPAL_OPA_HEALTH_CHECK_POLICY_ENABLED must be set to True

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

