# \ApplicationsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_application**](ApplicationsApi.md#create_application) | **POST** /api/v1/workspaces/{workspace_id}/applications | 
[**delete_application**](ApplicationsApi.md#delete_application) | **DELETE** /api/v1/workspaces/{workspace_id}/applications/{application_id} | 
[**get_application**](ApplicationsApi.md#get_application) | **GET** /api/v1/workspaces/{workspace_id}/applications/{application_id} | 
[**list_applications**](ApplicationsApi.md#list_applications) | **GET** /api/v1/workspaces/{workspace_id}/applications | 
[**update_application**](ApplicationsApi.md#update_application) | **PATCH** /api/v1/workspaces/{workspace_id}/applications/{application_id} | 



## create_application

> models::CreateApplicationSuccess create_application(workspace_id, create_application_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **u32** | The workspace's id | [required] |
**create_application_request** | [**CreateApplicationRequest**](CreateApplicationRequest.md) |  | [required] |

### Return type

[**models::CreateApplicationSuccess**](CreateApplicationSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application

> serde_json::Value delete_application(workspace_id, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **u32** | The workspace's id | [required] |
**application_id** | **u32** | The application's id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application

> models::ReadApplicationSuccess get_application(workspace_id, application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **u32** | The workspace's id | [required] |
**application_id** | **u32** | The application's id | [required] |

### Return type

[**models::ReadApplicationSuccess**](ReadApplicationSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_applications

> models::ListApplicationsSuccess list_applications(workspace_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **u32** | The workspace's id | [required] |

### Return type

[**models::ListApplicationsSuccess**](ListApplicationsSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application

> models::UpdateApplicationSuccess update_application(workspace_id, application_id, update_application_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **u32** | The workspace's id | [required] |
**application_id** | **u32** | The application's id | [required] |
**update_application_request** | [**UpdateApplicationRequest**](UpdateApplicationRequest.md) |  | [required] |

### Return type

[**models::UpdateApplicationSuccess**](UpdateApplicationSuccess.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

