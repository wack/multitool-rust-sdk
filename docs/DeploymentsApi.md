# \DeploymentsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_deployment**](DeploymentsApi.md#create_deployment) | **POST** /api/v1/workspaces/{workspace_id}/applications/{application_id}/deployments | 
[**list_deployments**](DeploymentsApi.md#list_deployments) | **GET** /api/v1/workspaces/{workspace_id}/applications/{application_id}/deployments | 
[**read_deployment**](DeploymentsApi.md#read_deployment) | **GET** /api/v1/workspaces/{workspace_id}/applications/{application_id}/deployments/{deployment_id} | 



## create_deployment

> models::CreateDeploymentSuccess create_deployment(workspace_id, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **uuid::Uuid** | The workspace's id | [required] |
**application_id** | **uuid::Uuid** | The application's id | [required] |

### Return type

[**models::CreateDeploymentSuccess**](CreateDeploymentSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_deployments

> models::ListDeploymentsSuccess list_deployments(workspace_id, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **uuid::Uuid** | The workspace's id | [required] |
**application_id** | **uuid::Uuid** | The application's id | [required] |

### Return type

[**models::ListDeploymentsSuccess**](ListDeploymentsSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_deployment

> models::ReadDeploymentSuccess read_deployment(workspace_id, application_id, deployment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **uuid::Uuid** | The workspace's id | [required] |
**application_id** | **uuid::Uuid** | The application's id | [required] |
**deployment_id** | **i64** | The deployment's id | [required] |

### Return type

[**models::ReadDeploymentSuccess**](ReadDeploymentSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

