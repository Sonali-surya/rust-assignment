use chrono::Utc;
use clap::{Parser, arg};
use calamine::Reader;
use calamine::open_workbook;
use calamine::Xlsx;
use calamine::DataType;
use std::fs;
use std::collections::HashMap;
use crate::*;
use chrono::Datelike;


 
   
pub fn read_sal_excel(path:String)->HashMap<i32,String>{
  let dt = Utc::now();
  let mut month:HashMap<i32,String>=HashMap::new();
    month.insert(1,"Jan".to_string());
    month.insert(2,"Feb".to_string());
    month.insert(3,"Mar".to_string());
    month.insert(4,"Apr".to_string());
    month.insert(5,"May".to_string());
    month.insert(6,"Jun".to_string());
    month.insert(7,"Jul".to_string());
    month.insert(8,"Aug".to_string());
    month.insert(9,"Sep".to_string());
    month.insert(10,"Oct".to_string());
    month.insert(11,"Nov".to_string());
    month.insert(1,"Dec".to_string());
    let mon=month.get(&(dt.month() as i32));
    let mon2=mon.unwrap();
    // println!("{}",mon2);

    let mut workbook: Xlsx<_> = open_workbook(path).expect("Open Failed");
    let mut rows: usize = 0;
    let mut cols: usize = 0;
    let mut dept_info: HashMap<i32, String> = HashMap::new();
      let mut f=false;
        match workbook.worksheet_range("Sheet1") {
            Some(Ok(range)) => {
                rows = range.get_size().0;
                cols = range.get_size().1;
                 
                for row in range.rows() {
                    if f==false{
                        f=true;
                        continue;
                      }
                    for ele in row {
                       let emp_id=match row[0]{
                        DataType::Float(v)=>v,
                        _ =>0_f64,
                       };
                       let sal_month=match row[2].clone(){
                           DataType::String(v)=>v,
                           _=>"".to_string(),
                       };
                      let month_val:Vec<String> = sal_month.split(" ").map(|s| s.to_string()).collect();
                      
                      let mut sal_status = match row[4].clone(){
                        DataType::String(v)=>v.clone(),
                        _ =>"".to_string(),
                       };
                      //  if(&month_val[0]!=mon2){
                      //   sal_status="Not_credited".to_string();
                      //  }
                       dept_info.insert(emp_id as i32,sal_status);
                    }
                    println!("  ");
                }
            },
            _ => (),
        }
        return dept_info;
    }
