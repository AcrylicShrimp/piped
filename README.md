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
