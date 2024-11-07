# Why?
- Its a personal learning project

## Does this Server does anything different than other servers?
- Nope
## Info
- Runs on localhost:8080 per default
## Features
- Support for a basic website with certain file types
- Only supports GET right now
- Host Header gets resolve
  - They are part of the Path. The server looks for a directory corresponding to the Host requested in it's  working directory, be careful to include the port when using non default ports, see the localhost:8080 example

## ToDo
- Configuration for root directories of different hosts (make multiple Websites possible)
- More file types
- Support other Http Request types

## Repo Structure
- Develop: Current develompemnt, untested might not compile at all
- master: Compiles and works as expected
