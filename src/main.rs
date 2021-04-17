use std::fs::File;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};

//only downloads one school + one subject
const DEBUG: bool = false;

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

    /* user prompt */
    let semester_options: &str = &get_html("https://classes.usc.edu/").await;
    let semester_document = Document::from(semester_options);

    let mut semesters: Vec<(String, String)> = Vec::with_capacity(3);

    for node in semester_document.find(Class("terms").descendant(Name("li"))) {
        let semester_id = node.attr("id").unwrap();
        let mut semester_name = node.find(Name("h3")).next().unwrap().text();

        let s_index = semester_name.find("TERM").unwrap_or(semester_name.len());
        semester_name.replace_range(s_index..semester_name.len(), "");
        semesters.push((semester_id.to_string(), semester_name));
    }

    println!("\nWhat semester would you like to download?");
    println!("/---------------------------------------/");

    let mut i = 1;
    for (_, name) in &semesters {
        println!("{}. {}", i, name);
        i += 1;
    }

    let mut input = String::new();
    let index:usize;
    
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => { 
            match input.trim_end().parse() {
                Ok(result) => {
                    index = result;        
                    if index > semesters.len() || index <= 0 {
                        println!("Invalid selection, please try again");
                        return Ok(());
                    }
                    println!();
                }
                Err(_) => {
                    println!("Invalid selection, please try again");
                    return Ok(());
                }

            }
        }
        Err(_) => {
            println!("Error reading input");
            return Ok(());
        }
    }
    
    let path = format!("https://classes.usc.edu/{}/...", &semesters[index-1].0);
    println!("Downloading: {}\n", path);

    match download_courses(&path).await {
        Ok(_) => {
            println!("Finished downloading courses.")
        }
        Err(_) => { 
            println!("Error downloading courses.")
        }
    }
    Ok(())
}

async fn get_html(path: &str) -> String  {
    match reqwest::get(path).await {
        Ok(resp) => {
            match resp.text().await {
                Ok(text) => {
                    return text.clone();
                }
                Err(_) => return String::from(""),
            }
        }
        Err(_) => return String::from(""),
    }
}

async fn download_courses(term: &str) -> std::result::Result<(), Box<dyn Error>> {

    /* list of all subjects in a "school" */
    let mut schools: HashMap<String, Vec<Subject>> = HashMap::new();

    /* collection of data that gets converted to json */
    let mut json_schools: Vec<School> = Vec::new();

    /* get request to fetch all the subjects from the catalogue page */
    let catalogue_html: &str = &get_html(term).await;
    let catalogue_document = Document::from(catalogue_html);

    println!("# Schools");
    for node in catalogue_document.find(Attr("data-type", "department")) {
        let school_selector = node.attr("data-school").unwrap();
        let mut school_name: String = String::from("empty");

        let school_code = node.attr("data-code").unwrap().trim();
        let school_title = node.attr("data-title").unwrap().trim();
        let mut name_element = catalogue_document.find(Attr("data-title", school_selector).descendant(Name("a")));

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
            .entry(school_name)
            .or_insert_with(Vec::new)
            .push(sub);
    }

    for (key, value) in &schools {
        println!("[ {} ]", key);
        let mut school: School = School::default();
        school.school_name = key.clone();

        for sub in value {
            let new_path = format!("{}classes/{}", term, &sub.code);

            /* get request to fetch all the courses from a specific subject page */
            let subject_html: &str = &get_html(&new_path).await;
            let subject_document = Document::from(subject_html);


            let mut subject: Subject = sub.clone();

            for node in subject_document.find(Class("course-table").descendant(Class("course-info"))) {

                let mut new_course = Course::default();

                /* get the course code of the course */
                match node.attr("id") {
                    Some(value) => { 
                        new_course.course_code = value.to_string();
                    }
                    None => {}
                }

                /* get the title of the course */
                let mut title = node.find(Name("h3").descendant(Name("a")))
                    .next()
                    .unwrap()
                    .text();

                /* wonky string manipulation to unfuck the course title */
                let f_index = title.find(":").unwrap_or(title.len());
                title.replace_range(..f_index + 2, "");
                let e_index = title.find('(').unwrap_or(title.len());
                title.replace_range(e_index..title.len(), "");

                new_course.course_title = title.to_string();
                subject.courses.push(new_course.clone());
            }

            school.subjects.push(subject);

            println!("  - {} has been read...", sub.name);

            if DEBUG { break; }
        }

        json_schools.push(school.clone());
        println!();

        if DEBUG { break; }
    }

    /* write the json_schools vector to file */
    let buffer = File::create("usc.json")?;
    let _serialized = serde_json::to_writer_pretty(buffer, &json_schools).unwrap();

    Ok(())
}
