# Roadmap

## Unplanned

- [ ] Improved summary output
- [ ] Structured summary output
- [ ] Shell completion
- [ ] Git aware
  - [ ] Git sub-module support
- [ ] Ignore files when linking

## `v0.0.4`

- [ ] Package management
  - [ ] List all packages
  - [ ] Add package
  - [ ] Deliberately don't provide delete package functionality, instead points
        to the directory to delete
- [ ] Pickup files into package

## `v0.0.3`

- [ ] Select script to run
- [ ] Select files to link

## `v0.0.2`

- [x] Flag-able built info display `dottie info -tv`
- [x] Version display `dottie -V`
- [x] Improved logging
  - [x] Toggle-able logging, configured via `RUST_LOG`, disabled by default
  - [x] Logging to file by piping the `stderr`

## `v0.0.1`

- [x] Package based dotfile management
- [x] `dottie link` for linking package files
- [x] `dottie run` for running package scripts
- [x] `dottie info` for built info display
