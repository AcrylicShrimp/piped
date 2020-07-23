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
