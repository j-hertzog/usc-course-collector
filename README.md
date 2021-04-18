# USC Course Collector
A lightweight rust program that downloads all of the courses for a given semester at USC in JSON format.

##### Dependencies
- [tokio](https://github.com/tokio-rs/tokio) for asynchronous web requests.
- [reqwest](https://github.com/seanmonstar/reqwest) for downloading html.
- [select.rs](https://github.com/utkarshkukreti/select.rs) for parsing html.
- [serde & serde_json](https://github.com/serde-rs/json) for json serialization of the course data.

