# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task**](DefaultApi.md#create_task) | **Post** /api/v1/tasks | 
[**get_task**](DefaultApi.md#get_task) | **Get** /api/v1/tasks/{tid} | 
[**list_tasks**](DefaultApi.md#list_tasks) | **Get** /api/v1/tasks | 
[**update_task**](DefaultApi.md#update_task) | **Post** /api/v1/tasks/{tid} | 



## create_task

> models::Task create_task(title, done, label, new_task)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**title** | **String** |  | [required] |
**done** | Option<**bool**> |  | [required] |
**label** | Option<**String**> |  | [required] |
**new_task** | [**NewTask**](NewTask.md) |  | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

[bearerTokenAuth](../README.md#bearerTokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task

> models::Task get_task(tid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **i32** |  | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

[bearerTokenAuth](../README.md#bearerTokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tasks

> Vec<models::Task> list_tasks()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Task>**](Task.md)

### Authorization

[bearerTokenAuth](../README.md#bearerTokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task

> update_task(done, label, tid, update_task)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**done** | Option<**bool**> |  | [required] |
**label** | Option<**String**> |  | [required] |
**tid** | **i32** |  | [required] |
**update_task** | [**UpdateTask**](UpdateTask.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerTokenAuth](../README.md#bearerTokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

