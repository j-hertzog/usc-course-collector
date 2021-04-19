# USC Course Collector
A lightweight rust program that downloads all of the courses offered in a USC semester in a simple JSON format.

![gif](course-collector.gif) 

## Output Format
As shown below the courses output is organized by *subjects* which are in turn organized into *schools*.
```
[
      {
        "name": "Computer Science",
        "code": "CSCI",
        "courses": [
          {
            "course_code": "CSCI-102",
            "course_title": "Fundamentals of Computation "
          },
          {
            "course_code": "CSCI-103",
            "course_title": "Introduction to Programming "
          },
          ...
        ]
      },
      {
        "name": "Data Science",
        "code": "DSCI",
        "courses": [
          {
            "course_code": "DSCI-351",
            "course_title": "Foundations of Data Management "
          },
          ...
        ]
      },
      ...
]
```

## Installation
- Requires [rust](https://www.rust-lang.org/tools/install) to be installed.
- '**cargo run**' to start the scraping.

##### Project Dependencies
- [tokio](https://github.com/tokio-rs/tokio) for asynchronous web requests.
- [reqwest](https://github.com/seanmonstar/reqwest) for downloading html.
- [select.rs](https://github.com/utkarshkukreti/select.rs) for parsing html.
- [serde & serde_json](https://github.com/serde-rs/json) for json serialization of the course data.

