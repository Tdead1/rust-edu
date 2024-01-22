use std::{collections::HashMap, fs, path::Path};
use uid;
use serde::{Deserialize, Serialize};
use nannou::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
struct CourseData
{
	name: String,
	description: String,
	attachments: Vec<String>,
	course_requirements: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[allow(dead_code)]
struct CourseMapData
{
	courses: Vec<CourseData>
}

#[allow(dead_code)]
#[derive(Debug)]
struct Course
{
	id: UID,
	required_courses: Vec<UID>,
	data: CourseData
}

#[allow(dead_code)]
struct CourseMap
{
	map: HashMap<UID, Course>,
	data: CourseMapData
}

// Need to think how this access pattern should be setup
static mut COURSE_DATABASE : Option<CourseMap> = None;

type UID = uid::Id<u64>;
pub fn main() 
{
	// Preperation:
	let mut string_ids: HashMap<UID, String> = HashMap::new();
	let _default_coursedata = CourseData {
		name: String::from("invalid"),
		description: String::from("invalid"),
		attachments: Vec::new(),
		course_requirements: Vec::new(),
	};
	
	// Serialization:
	let courses_file = Path::new("data/courses.json");
	let data_string = fs::read_to_string(courses_file).expect("Unable to read or find courses.json!");
	let course_data : CourseMapData =  match serde_json::from_str(data_string.as_str()) 
	{
		Ok(value) => value,
		_error => CourseMapData {courses: Vec::new()},
	};

	// Resolving:
	// String ID creation...
	let mut course_names : Vec<String> = Vec::new();
	for course in &course_data.courses
	{
		course_names.push(course.name.to_lowercase());
	}
	course_names.sort();
	course_names.dedup();
	for name in course_names
	{
		string_ids.insert(UID::new(), name);
	}
	// Course creation...
	let mut course_map: CourseMap = CourseMap {map: HashMap::new(), data: course_data}; 
	for course in &course_map.data.courses
	{
		let id = UID::new();
		let mut required_courses = Vec::new();
		for name in &course.course_requirements
		{
			let lowercase_name = name.to_lowercase();
			string_ids.iter().for_each(|(key, value)| 
			if value.to_string() == lowercase_name 
			{ 
				required_courses.push(key.to_owned()); 
			});
		}
		course_map.map.insert(id, Course { id, required_courses, data: course.clone() });
	}

	// Course resolving so the id's don't point to string names anymore...
	for course in &course_map.map
	{
		for mut requirement in &course.1.required_courses
		{
			let dependency_name = string_ids.get(&requirement).expect("Large problem with string ID indexing! Contact your nearest programmer.").clone();
			let mut found_dependency: bool = false;
			course_map.map.iter().for_each(|(id, course)| if course.data.name.to_lowercase() == dependency_name.to_lowercase()
			{ 
				requirement = id; 
				found_dependency = true; 
			});	
			if !found_dependency
			{
				println!("Dependency {} could not be found while resolving the names!", dependency_name);
			}
		}
	}

	// Debug:
	println!("Constant data after serialization:");
	for i in &course_map.data.courses
	{
		println!("{:?}", i);
	}
	println!();
	println!("Runtime data after resolving:");
	for i in &course_map.map
	{
		println!("{:?}", i);
	}

	// Storing our resolved data into a global variable
	//unsafe { COURSE_DATABASE = Option::Some(course_map) };
	// Not the rust way :(
	nannou::sketch(view).run()
}

fn draw_course(draw :&Draw, uid :&UID, x :f32, y :f32)
{
	draw.quad()
	.x_y(x,y);

	draw.text("Hello!")
	.x_y(x,y)
	.color(BLACK);
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(CORNFLOWERBLUE);

    //// Draw a quad that follows the inverse of the ellipse.
	//let t = app.time;
	//for i in &(COURSE_DATABASE.unwrap()).map
	//{
	//	draw_course(&draw, &i.0, 50.0, 50.0);
	//}

    
	// Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}