# \ErrorsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**log_error**](ErrorsApi.md#log_error) | **POST** /api/v1/workspaces/{workspace_id}/applications/{application_id}/rollouts/{rollout_id}/metrics/errors | 



## log_error

> serde_json::Value log_error(workspace_id, application_id, rollout_id, create_error_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **u32** | The workspace's id | [required] |
**application_id** | **u32** | The application's id | [required] |
**rollout_id** | **u64** | The rollout's id | [required] |
**create_error_request** | [**CreateErrorRequest**](CreateErrorRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

