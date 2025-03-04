# \DeploymentStatesApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_deployment_states**](DeploymentStatesApi.md#list_deployment_states) | **GET** /api/v1/workspaces/{workspace_id}/applications/{application_id}/deployments/{deployment_id}/states | 
[**refresh_deployment_state**](DeploymentStatesApi.md#refresh_deployment_state) | **PATCH** /api/v1/workspaces/{workspace_id}/applications/{application_id}/deployments/{deployment_id}/states/{state_id}/refresh | 
[**update_deployment_state**](DeploymentStatesApi.md#update_deployment_state) | **PATCH** /api/v1/workspaces/{workspace_id}/applications/{application_id}/deployments/{deployment_id}/states/{state_id} | 



## list_deployment_states

> models::ListDeploymentStatesSuccess list_deployment_states(workspace_id, application_id, deployment_id, status)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **uuid::Uuid** | The workspace's id | [required] |
**application_id** | **uuid::Uuid** | The application's id | [required] |
**deployment_id** | **u64** | The deployment's id | [required] |
**status** | [**DeploymentStateStatus**](.md) | An optional status to filter by | [required] |

### Return type

[**models::ListDeploymentStatesSuccess**](ListDeploymentStatesSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_deployment_state

> models::UpdateDeploymentStateSuccess refresh_deployment_state(workspace_id, application_id, deployment_id, state_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **uuid::Uuid** | The workspace's id | [required] |
**application_id** | **uuid::Uuid** | The application's id | [required] |
**deployment_id** | **u64** | The deployment's id | [required] |
**state_id** | **u64** | The deployment state's id | [required] |

### Return type

[**models::UpdateDeploymentStateSuccess**](UpdateDeploymentStateSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_deployment_state

> models::UpdateDeploymentStateSuccess update_deployment_state(workspace_id, application_id, deployment_id, state_id, update_deployment_state_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **uuid::Uuid** | The workspace's id | [required] |
**application_id** | **uuid::Uuid** | The application's id | [required] |
**deployment_id** | **u64** | The deployment's id | [required] |
**state_id** | **u64** | The deployment state's id | [required] |
**update_deployment_state_request** | [**UpdateDeploymentStateRequest**](UpdateDeploymentStateRequest.md) |  | [required] |

### Return type

[**models::UpdateDeploymentStateSuccess**](UpdateDeploymentStateSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

