# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [src/protocols/v1/todos.proto](#src_protocols_v1_todos-proto)
    - [CreateTodoRequest](#example-v1-todos-CreateTodoRequest)
    - [CreateTodoResponse](#example-v1-todos-CreateTodoResponse)
    - [DeleteTodoRequest](#example-v1-todos-DeleteTodoRequest)
    - [DeleteTodoResponse](#example-v1-todos-DeleteTodoResponse)
    - [GetTodoRequest](#example-v1-todos-GetTodoRequest)
    - [GetTodoResponse](#example-v1-todos-GetTodoResponse)
    - [ListTodosRequest](#example-v1-todos-ListTodosRequest)
    - [ListTodosResponse](#example-v1-todos-ListTodosResponse)
    - [Todo](#example-v1-todos-Todo)
    - [UpdateTodoRequest](#example-v1-todos-UpdateTodoRequest)
    - [UpdateTodoResponse](#example-v1-todos-UpdateTodoResponse)
  
    - [TodoService](#example-v1-todos-TodoService)
  
- [Scalar Value Types](#scalar-value-types)



<a name="src_protocols_v1_todos-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## src/protocols/v1/todos.proto



<a name="example-v1-todos-CreateTodoRequest"></a>

### CreateTodoRequest
Request message for CreateTodo.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| todo | [Todo](#example-v1-todos-Todo) |  | The todo to create. |






<a name="example-v1-todos-CreateTodoResponse"></a>

### CreateTodoResponse
Response message for CreateTodo.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| todo | [Todo](#example-v1-todos-Todo) |  | The created todo. |






<a name="example-v1-todos-DeleteTodoRequest"></a>

### DeleteTodoRequest
Request message for DeleteTodo.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| todo_id | [string](#string) |  | The ID of the todo to delete. |






<a name="example-v1-todos-DeleteTodoResponse"></a>

### DeleteTodoResponse
Response message for DeleteTodo. Placeholder, could contain the deleted todo
if needed, or other status information.






<a name="example-v1-todos-GetTodoRequest"></a>

### GetTodoRequest
Request message for GetTodo.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| todo_id | [string](#string) |  | The ID of the todo to retrieve. |






<a name="example-v1-todos-GetTodoResponse"></a>

### GetTodoResponse
Response message for GetTodo. Contains the todo.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| todo | [Todo](#example-v1-todos-Todo) |  | The todo that was retrieved. |






<a name="example-v1-todos-ListTodosRequest"></a>

### ListTodosRequest
Request message for ListTodos. Placeholder for filters, pagination and
ordering.






<a name="example-v1-todos-ListTodosResponse"></a>

### ListTodosResponse
Response message for ListTodos.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| todos | [Todo](#example-v1-todos-Todo) | repeated | The list of todos requested. |






<a name="example-v1-todos-Todo"></a>

### Todo
Todo message.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| todo_id | [string](#string) |  | The ID of the todo. |
| title | [string](#string) |  | The title of the todo. |
| description | [string](#string) |  | The description of the todo. |
| completed | [bool](#bool) |  | Whether the todo is completed. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | The time the todo was created. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | The time the todo was last updated. |






<a name="example-v1-todos-UpdateTodoRequest"></a>

### UpdateTodoRequest
Request message for UpdateTodo. Contains the todo to update and a field mask
indicating which fields should be updated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| todo | [Todo](#example-v1-todos-Todo) |  | The todo to update. |
| update_mask | [google.protobuf.FieldMask](#google-protobuf-FieldMask) |  | The field mask indicating which fields should be updated. |






<a name="example-v1-todos-UpdateTodoResponse"></a>

### UpdateTodoResponse
Response message for UpdateTodo.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| todo | [Todo](#example-v1-todos-Todo) |  | The updated todo. |





 

 

 


<a name="example-v1-todos-TodoService"></a>

### TodoService
Service for managing todos,

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| ListTodos | [ListTodosRequest](#example-v1-todos-ListTodosRequest) | [ListTodosResponse](#example-v1-todos-ListTodosResponse) | List all todos |
| GetTodo | [GetTodoRequest](#example-v1-todos-GetTodoRequest) | [GetTodoResponse](#example-v1-todos-GetTodoResponse) | Geta a single todo by its ID |
| CreateTodo | [CreateTodoRequest](#example-v1-todos-CreateTodoRequest) | [CreateTodoResponse](#example-v1-todos-CreateTodoResponse) | Create a new todo |
| UpdateTodo | [UpdateTodoRequest](#example-v1-todos-UpdateTodoRequest) | [UpdateTodoResponse](#example-v1-todos-UpdateTodoResponse) | Update an existing todo by its ID |
| DeleteTodo | [DeleteTodoRequest](#example-v1-todos-DeleteTodoRequest) | [DeleteTodoResponse](#example-v1-todos-DeleteTodoResponse) | Delete an existing todo by its ID |

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

