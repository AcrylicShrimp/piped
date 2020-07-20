# piped

A pipeline orchestrator for everyone to build softwares easier and faster.

> Warning: This project is on early stage of development now. Any breaking changes can be made.

## What is it?

Basically `piped` is a build system such as the `cmake`, `MSBuild` or `bazel`. But you can define custom pipelines to build or process assets in any formats with any tools.

## Docs

### Predefined variables

| Name       | Type   | Description                                          | Possible values             |
| ---------- | ------ | ---------------------------------------------------- | --------------------------- |
| `hostArch` | String | Indicates the CPU's architecture of the host system. | `x86`, `x86_64`, `arm`      |
| `hostOS`   | String | Indicates the CPU's architecture of the host system. | `linux`, `macos`, `windows` |

### Syntax

#### Comments

Comments start by double slashes. They include all characters until the end of line.

```
// One-line comment
```

#### Variables & Set statements

Variables can be one of types of below.

| Type       | Description                                                |
| ---------- | ---------------------------------------------------------- |
| Bool       | An boolean type that can be `true` or `false` at a time.   |
| Integer    | A 64-bit signed integer.                                   |
| String     | A null-terminated variable length string.                  |
| Array      | An array that contains variable length of other values.    |
| Dictionary | An collection of key-value pairs. Keys are always strings. |

String literals can be escaped by `\`(back slash). Here're all escape sequences supported.

| Escape sequence | Description                |
| --------------- | -------------------------- |
| `\n`            | Line feed character.       |
| `\r`            | Carriage return character. |
| `\t`            | Tap character.             |
| `\\`            | Back space character.      |
| `\0`            | Null character.            |
| `\'`            | Single quote character.    |
| `\"`            | Double quote character.    |
| `` \` ``        | Grave character.           |

Variables can be set by `set` statements.

```
@set bar=0;
@set bar="Some string value";
@set foo=[bar, baz];
```

#### Print statements

You can write anything to the `stdout` or `stderr` as you need.

```
@print "A value of 'foo' is " foo;
@printErr "Error! the given path '" path "' is not valid!";
```
