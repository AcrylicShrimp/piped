# piped

## Functions

In this document we're going to describe all functions.

---

### `get(array_or_dict, index)`

#### Summary

Gets the inner value from the given array or dictionary.

#### Parameters

- `array_or_dict`: An array or a dictionary value to be used as source.
- `index`: An index value. Must be a integer if `array_or_dict` is array. Otherwise string.

#### Return value

An extracted value from the `array_or_dict`.

#### Description

This function can be transformed as below on other popular languages.

```
array_or_dict[index]
```

If `array_or_dict` is an array, it returns `index`th element of the `array_or_dict`. The `index` should be a integer. If the `index` is negative, it indicates the index is started from the end. The `piped` will be panic if the `index` is out of range.

If `array_or_dict` is an dictionary, it returns an item marked as `index` as a key. The `index` should be a string. The `piped` will be panic if the `array_or_dict` not contains `index` as a key.

#### Example

```
@print get([1, 2, 3], -1);		// 3
```

---

### `equals(lhs, rhs)`

#### Summary

Checks if given values are equal.

#### Parameters

- `lhs`: An left side value to be tested.
- `rhs`: An right side value to be tested.

#### Return value

`true` if two given values are equal. `false` otherwise.

#### Description

It first tests the types of the values. After, it checks lengths of the values if the `lhs` and the `rhs` are array or dictionary. The order of items is irrelevant if they are dictionary. If they are bool, integer or string, it tries to match their values directly.

#### Example

```
@print equals([], "");														// false
@print equals({foo: "value", bar: true}, {bar: true, foo: "value"});		// true
```

---

### `is_exists(variable_name)`

#### Summary

Checks there is a variable with the given name.

#### Parameters

- `variable_name`: A name of a variable.

#### Return value

`true` if there is a variable with the given name `variable_name`. `false` otherwise.

#### Description

It returns `true` if there is a variable with the given name `variable_name` in this execution context. It returns `false` if not. It is useful when you are writing custom pipelines that requires some arguments as parameter.

#### Example

```
@set foo="hello, world!";

@print is_exists("foo");		// true
@print is_exists("bar");		// false
```

---

### `len(array_or_dict)`

#### Summary

Get length of the given array or dictionary.

#### Parameters

- `array_or_dict`: An array or a dictionary value to be counted.

#### Return value

An number of items of the given array of the `array_or_dict`.

#### Description

This function counts and returns a number of items that the given `array_or_dict` holds. The `array_or_dict` should be one of a array or dictionary.

#### Example

```
@print len([0, 1, 2]);		// 3
```

---

### `typeof(value)`

#### Summary

Gets the type name of the given value.

#### Parameters

- `value`: An value to be tested.

#### Return value

A type name of the `value`.

#### Description

This function returns one of the type name that the `value` has.

| Type of the `value` | Return value |
| ------------------- | ------------ |
| `Array`             | array        |
| `Dictionary`        | dictionary   |
| `Bool`              | bool         |
| `Integer`           | integer      |
| `String`            | string       |

#### Example

```
@print typeof(["a"]);		// array
@print typeof(-1);			// integer
@print typeof("hello!");	// string
```
