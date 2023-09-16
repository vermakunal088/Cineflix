#![allow(unused)]
use std::{io, ops::Add};
use chrono::prelude::*;
use std::fs::File;
use chrono::{NaiveDate, NaiveDateTime,Duration};
use rand::Rng;
use bincode::{serialize, deserialize};

enum Status{
    TODO,
    PICKEDUP,
    BLOCKED,
    DONE,
}

#[derive(Serialize, Deserialize)]
struct Task{
   title:String,description:String,notify:bool,time:i64,temp:bool,status:Status,task_id:i64,
}

impl Task{
    
}

impl Default for Task{
    fn default() -> Self {
        Task{
            title: String::from("Placeholder-title"),
            description: String::from("Placeholder-description"),
            notify: false,
            time:0,
            temp:false,
            task_id:get_task_id(),
            status:Status::TODO,
        }
    }
}

fn get_dates_obj(succ:i64)->(i32, i32, i32){
    let cd = Utc::now()+Duration::days(succ);
    return (cd.year(),cd.month() as i32,cd.day() as i32)
}

fn get_task_id()->i64 {
    return rand::thread_rng().gen();
}

fn get_timestamp(year :i32,month :i32,day :i32,hour :i32,minute :i32,second :i32)->i64 {
    let dt=NaiveDate::from_ymd_opt(year,month as u32,day as u32).unwrap().and_hms_opt(hour as u32,minute as u32,second as u32).unwrap();
    return dt.timestamp();
}

fn add_task(file_name:String,task :Task){
    let mut today_todo = File::create(file_name).expect("Error Creating todo file").unwrap();
    let serialized =serialize(&task).unwrap();
    today_todo.write_all(&serialized).unwrap();
    today_todo.flush().unwrap();
}


fn main() {
    
}
