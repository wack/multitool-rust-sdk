# \RolloutStatesApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_rollout_state**](RolloutStatesApi.md#create_rollout_state) | **POST** /api/v1/workspaces/{workspace_id}/applications/{application_id}/rollouts/{rollout_id}/state | 
[**list_rollout_states**](RolloutStatesApi.md#list_rollout_states) | **GET** /api/v1/workspaces/{workspace_id}/applications/{application_id}/rollouts/{rollout_id}/states | 
[**refresh_rollout_state**](RolloutStatesApi.md#refresh_rollout_state) | **POST** /api/v1/workspaces/{workspace_id}/applications/{application_id}/rollouts/{rollout_id}/states/{state_id}/refresh | 
[**update_rollout_state**](RolloutStatesApi.md#update_rollout_state) | **PATCH** /api/v1/workspaces/{workspace_id}/applications/{application_id}/rollouts/{rollout_id}/states/{state_id} | 



## create_rollout_state

> models::CreateRolloutStateSuccess create_rollout_state(workspace_id, application_id, rollout_id, create_rollout_state_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **u32** | The workspace's id | [required] |
**application_id** | **u32** | The application's id | [required] |
**rollout_id** | **u64** | The rollout's id to add the state to | [required] |
**create_rollout_state_request** | [**CreateRolloutStateRequest**](CreateRolloutStateRequest.md) |  | [required] |

### Return type

[**models::CreateRolloutStateSuccess**](CreateRolloutStateSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_rollout_states

> models::ListRolloutStatesSuccess list_rollout_states(workspace_id, application_id, rollout_id, status)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **u32** | The workspace's id | [required] |
**application_id** | **u32** | The application's id | [required] |
**rollout_id** | **u64** | The rollout's id | [required] |
**status** | Option<[**RolloutStateStatus**](.md)> | An optional status to filter by |  |

### Return type

[**models::ListRolloutStatesSuccess**](ListRolloutStatesSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_rollout_state

> models::UpdateRolloutStateSuccess refresh_rollout_state(workspace_id, application_id, rollout_id, state_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **u32** | The workspace's id | [required] |
**application_id** | **u32** | The application's id | [required] |
**rollout_id** | **u64** | The rollout's id | [required] |
**state_id** | **u64** | The rollout state's id | [required] |

### Return type

[**models::UpdateRolloutStateSuccess**](UpdateRolloutStateSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_rollout_state

> models::UpdateRolloutStateSuccess update_rollout_state(workspace_id, application_id, rollout_id, state_id, update_rollout_state_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **u32** | The workspace's id | [required] |
**application_id** | **u32** | The application's id | [required] |
**rollout_id** | **u64** | The rollout's id | [required] |
**state_id** | **u64** | The rollout state's id | [required] |
**update_rollout_state_request** | [**UpdateRolloutStateRequest**](UpdateRolloutStateRequest.md) |  | [required] |

### Return type

[**models::UpdateRolloutStateSuccess**](UpdateRolloutStateSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

