# piped

A pipeline orchestrator for everyone to build softwares easier and faster.

> Warning: This project is on early stage of development now. Any breaking changes can be made.

## What is it?

Basically `piped` is a build system such as the `cmake`, `MSBuild` or `bazel`. But you can define custom pipelines to build or process assets in any formats with any tools.

## Usage

`piped <input>`

The `<input>` is a valid pipeline file path.

## Examples

See the [`examples`](https://github.com/AcrylicShrimp/piped/tree/master/examples) directory.

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

#### Pipeline invocations

Pipeline is a main concept of the `piped` - it is some logics that can be executed in parallel. You can do many things with them such as downloading, unzipping, clonning and compiling. To invoke them, just type its name and feed some arguments.

```
copy src="my_file.txt" dst="./some/path";
```

##### Background invocations

If it is ok to run a pipeline in background and execute next lines immediately, mark them as `nonblock`. The `piped` will execute them on background worker thread in parallel.

```
// It will not block execution!
@nonblock copy src="my_file.txt" dst="./some/path";
```

Also you can give it a name to refer it later.

```
@nonblock "my-copy" copy src="my_file.txt" dst="./some/path";
```

##### Awaiting invocations

Use `await` to wait for background invocations. There are 3 different types to do so.

```
@await;				// This will wait for all previous "unnamed" background invocations.
@await "my-copy";	// This will wait for all previous "named" background invocations.
@await all;			// This will wait for all previous background invocations regardless of named or unnamed.
```

##### Importing pipelines

Any pipelines can be imported by `import` statements.

#### If statements

`if` statements are quite typical.

```
@if equals(hostOS, "windows") {
	@print "Hello, windows!";
} @else if equals(hostOS, "macos") {
	@print "Hello, macos!";
} @else {
	@print "Hello, unknown!";
}
```

Since the `piped` does not support any operator in syntax, you should use functions for real world applications. Please refer [functions](https://github.com/AcrylicShrimp/piped#functions) for more details.

### Functions

Functions are callable logics that always return a value.

```
some_function(1, 2, 3);
```

Unlike pipeline invocations, functions have positional parameters. It means position of each arguments are important.

### Built-ins

See [here](docs/built-ins.md).
