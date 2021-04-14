use std::io::prelude::*;
use std::fs::File;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::collections::HashMap;
use std::error::Error;
use tokio::time::{sleep, Duration};
use serde_json::{Result, Value, json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)] // Derive is cool, I have no idea how it works!
struct School {
    school_name: String,
    subjects: Vec<Subject>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)] // Derive is cool, I have no idea how it works!
struct Subject {
    name: String,
    code: String,
    courses: Vec<Course>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)] // Derive is cool, I have no idea how it works!
struct Course {
    course_code: String, 
    course_title: String, 
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {

    let path = "https://classes.usc.edu/term-20211/";
    let mut schools: HashMap<String, Vec<Subject>> = HashMap::new();
    let mut json_schools: Vec<School> = Vec::new();

    match reqwest::get(path).await {
        Ok(resp) => {
            match resp.text().await {
                Ok(text) => {
                    let t_slice: &str = &text[..];  // take a full slice of the string

                    let document = Document::from(t_slice);

                    println!("# Schools");
                    for node in document.find(Attr("data-type", "department")) {
                        let school_selector = node.attr("data-school").unwrap();
                        let mut school_name: String = String::from("empty");

                        let school_code = node.attr("data-code").unwrap().trim();
                        let school_title = node.attr("data-title").unwrap().trim();

                        let mut name_element = document.find(Attr("data-title", school_selector).descendant(Name("a")));

                        match name_element.next() {
                            Some(value) => { 
                                school_name = value.text();
                            }
                            None => {}
                        }

                        let sub = Subject { 
                            name: school_title.to_string(), 
                            code: school_code.to_string(),
                            courses: Vec::new(),
                        };

                        schools
                            .entry(school_name.clone())
                            .or_insert_with(Vec::new)
                            .push(sub);

                    }
                    for (key, value) in &schools {
                        println!("=== {} ===", key);

                        let mut school: School = School::default();
                        school.school_name = key.clone();
                        let mut new_sub: Subject = Subject::default();

                        for subject in value {
                            new_sub.name = subject.name.clone();
                            new_sub.code = subject.code.clone();
                            let new_path = format!("{}classes/{}", path, &subject.code);

                            // get request to fetch all the courses from a specific subject
                            match reqwest::get(new_path).await {
                                Ok(resp) => {
                                    match resp.text().await {
                                        Ok(text) => {
                                            let c_slice: &str = &text[..];  // take a full slice of the string
                                            let classes = Document::from(c_slice);

                                            for node in classes.find(Class("course-table").descendant(Class("course-info"))) {
                                                let mut new_course = Course::default();

                                                match node.attr("id") {
                                                    Some(value) => { 
                                                        new_course.course_code = value.to_string();
                                                    }
                                                    None => {}
                                                }
                                                let mut title = node
                                                    .find(Name("h3").descendant(Name("a")))
                                                    .next()
                                                    .unwrap()
                                                    .text();

                                                // wonky string manipulation to unfuck the course title
                                                let f_index = title.find(":").unwrap_or(title.len());
                                                title.replace_range(..f_index + 2, "");
                                                let e_index = title.find('(').unwrap_or(title.len());
                                                title.replace_range(e_index..title.len(), "");

                                                new_course.course_title = title.to_string();
                                                new_sub.courses.push(new_course.clone());
                                            }
                                        }
                                        Err(_) => println!("ERROR reading {}", path),
                                    }
                                }
                                Err(_) => println!("ERROR downloading {}", path),
                            }
                            school.subjects.push(new_sub.clone());
                            sleep(Duration::from_millis(1000)).await;
                            println!("[ {} ] has been read...", new_sub.name);
                            break; 
                        }
                        json_schools.push(school.clone());
                        break;
                    }
                }
                Err(_) => println!("ERROR reading {}", path),
            }
        }
        Err(_) => println!("ERROR downloading {}", path),
    }

    let mut buffer = File::create("usc.json")?;
    let serialized = serde_json::to_writer_pretty(buffer, &json_schools).unwrap();
    Ok(())
}

