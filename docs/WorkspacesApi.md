# \WorkspacesApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_workspace**](WorkspacesApi.md#create_workspace) | **POST** /api/v1/workspaces | 
[**list_workspaces**](WorkspacesApi.md#list_workspaces) | **GET** /api/v1/workspaces | 
[**read_workspace**](WorkspacesApi.md#read_workspace) | **GET** /api/v1/workspaces/{id} | 
[**update_workspace**](WorkspacesApi.md#update_workspace) | **PATCH** /api/v1/workspaces/{id} | 



## create_workspace

> models::CreateWorkspaceSuccess create_workspace(create_workspace_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_workspace_request** | [**CreateWorkspaceRequest**](CreateWorkspaceRequest.md) |  | [required] |

### Return type

[**models::CreateWorkspaceSuccess**](CreateWorkspaceSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workspaces

> models::ListWorkspaceSuccess list_workspaces(display_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**display_name** | Option<**String**> | An optional workspace name to filter by |  |

### Return type

[**models::ListWorkspaceSuccess**](ListWorkspaceSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_workspace

> models::WorkspaceReadSuccess read_workspace(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **u32** | The workspace's id | [required] |

### Return type

[**models::WorkspaceReadSuccess**](WorkspaceReadSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workspace

> models::UpdateWorkspaceSuccess update_workspace(id, update_workspace_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **u32** | The workspace's id | [required] |
**update_workspace_request** | [**UpdateWorkspaceRequest**](UpdateWorkspaceRequest.md) |  | [required] |

### Return type

[**models::UpdateWorkspaceSuccess**](UpdateWorkspaceSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

