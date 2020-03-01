A rust learning experience/utility to quickly check the build status of the current branch

Notes:
- `reqwest` seems to be the http library to use
  - how to get json responses?
    - integrates with serde?
  - need to write a test to get it running
    - can't write async tests?
      - need to figure out tokio and use `tokio_macros::test`?
      - can maybe write sync tests with `futures::executor::block_on`?
- need to figure out how to do a windows build from the docker container
  - publish to github releases