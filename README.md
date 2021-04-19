# USC Course Collector
A lightweight rust program that downloads all of the courses for a given semester at USC in a simple JSON format.

![gif](course-collector.gif) 

## Output Format
As shown below the courses output is organized by 'Subjects' which are in turn organized into 'Schools'.
```
[ { "name": "Computer Science", "code": "CSCI", "courses": [ { "course_code": "CSCI-102", "course_title": "Fundamentals of Computation " }, { "course_code": "CSCI-103", "course_title": "Introduction to Programming " }, { "course_code": "CSCI-104", "course_title": "Data Structures and Object Oriented Design " }, ...  ] }, { "name": "Data Science", "code": "DSCI", "courses": [ { "course_code": "DSCI-351", "course_title": "Foundations of Data Management " }, ...  ] }, ...  ]
```

## Installation
- Requires [rust]() to be installed.


##### Project Dependencies
- [tokio](https://github.com/tokio-rs/tokio) for asynchronous web requests.
- [reqwest](https://github.com/seanmonstar/reqwest) for downloading html.
- [select.rs](https://github.com/utkarshkukreti/select.rs) for parsing html.
- [serde & serde_json](https://github.com/serde-rs/json) for json serialization of the course data.

