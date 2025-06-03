# \RolloutsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_rollout**](RolloutsApi.md#create_rollout) | **POST** /api/v1/workspaces/{workspace_id}/applications/{application_id}/rollouts | 
[**list_rollouts**](RolloutsApi.md#list_rollouts) | **GET** /api/v1/workspaces/{workspace_id}/applications/{application_id}/rollouts | 
[**read_rollout**](RolloutsApi.md#read_rollout) | **GET** /api/v1/workspaces/{workspace_id}/applications/{application_id}/rollouts/{rollout_id} | 



## create_rollout

> models::CreateRolloutSuccess create_rollout(workspace_id, application_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **u32** | The workspace's id | [required] |
**application_id** | **u32** | The application's id | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::CreateRolloutSuccess**](CreateRolloutSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_rollouts

> models::ListRolloutsSuccess list_rollouts(workspace_id, application_id, rollout_number)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **u32** | The workspace's id | [required] |
**application_id** | **u32** | The application's id | [required] |
**rollout_number** | Option<**u64**> | The rollout's number within a given application |  |

### Return type

[**models::ListRolloutsSuccess**](ListRolloutsSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_rollout

> models::ReadRolloutSuccess read_rollout(workspace_id, application_id, rollout_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **u32** | The workspace's id | [required] |
**application_id** | **u32** | The application's id | [required] |
**rollout_id** | **u64** | The rollout's id | [required] |

### Return type

[**models::ReadRolloutSuccess**](ReadRolloutSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

