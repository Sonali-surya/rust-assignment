use clap::{Parser, arg};
use calamine::Reader;
use calamine::open_workbook;
use calamine::Xlsx;
use calamine::DataType;
use std::fs;
use std::collections::HashMap;
use crate::*;

    pub fn read_dept_excel(path:String)->HashMap<i32,String>{
    
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
                       let dept_id=match row[0]{
                        DataType::Float(v)=>v,
                        _ =>0_f64,
                       };
                      let dept_title = match &row[1]{
                        DataType::String(v)=>v,
                        _ => "",
                       };
                       let deptval=dept_title.to_string();
                       dept_info.insert(dept_id as i32,deptval);
                    }
                    println!("  ");
                }
            },
            _ => (),
        }
        return dept_info;
    }
