use chrono::Duration;
use clap::{Parser, arg};
use calamine::Reader;
use calamine::open_workbook;
use calamine::Xlsx;
use calamine::DataType;
use std::fs;
use std::collections::HashMap;
use crate::*;
use chrono::NaiveDate;
use chrono::{NaiveDateTime, DateTime, Utc, Datelike};


pub fn date_from_float(dte:i64)->NaiveDate {
    let start = NaiveDate::from_ymd_opt(1900, 1, 1).expect("DATE");
    let date = start.checked_add_signed(Duration::days(dte-2));
    date.unwrap()

}
pub fn get_days_from_month(year: i32, month: u32) -> i64 {
  NaiveDate::from_ymd(
      match month {
          12 => year + 1,
          _ => year,
      },
      match month {
          12 => 1,
          _ => month + 1,
      },
      1,
  )
  .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
  .num_days()
}


    pub fn read_leave_excel(path:String)->HashMap<i32,i32>{
    
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Open Failed");
    let mut rows: usize = 0;
    let mut cols: usize = 0;
    let mut leave_info: HashMap<i32, i32> = HashMap::new();
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
                      let leave_from = match row[2]{
                        DataType::DateTime(v)=>{
                          v
                        },
                        _ =>0_f64,
                       };
                       let leave_to=match row[3]{
                         DataType::DateTime(v)=>{
                         
                          v
                         },
                         _=>0_f64,
                       };
                      
                      //  let leave_count=(leave_to-leave_from) as i32;
                       
                      //  leave_info.insert(emp_id as i32,leave_to as i32);
                      // let years = Utc::now().year() - leave_from.year();
                      // let months = Utc::now().month() as i64 - leave_from.month() as i64;
                      // let days = Utc::now().day() as i64 - leave_from.day() as i64;
                       
                      
                       let leave_from2=date_from_float(leave_from as i64);
                       let leave_to2=date_from_float(leave_to as i64);
                       let curr_month = Utc::now().month();
                       let leave_from_month=leave_from2.month();
                

                       let mut leave_count=0;
                       if(curr_month==leave_from_month && curr_month==leave_to2.month()){
                        leave_count=leave_to2.day()-leave_from2.day()+1;
                       }
                       if(curr_month != leave_from_month && curr_month == leave_to2.month()){
                         leave_count=leave_to2.day();
                       }
                       if(curr_month == leave_from_month && curr_month != leave_to2.month()){
                           let curr_year=Utc::now().year();
                           let day_in_month=get_days_from_month(curr_year, curr_month);
                           leave_count=(day_in_month as u32)-leave_from2.day();
                       }
                       println!("{}",leave_count);
                      //  println!("{:?}\n{:?}",leave_from2,leave_to2);
                       
                       leave_info.insert(emp_id as i32,leave_count as i32);
                       

                      //  println!("{}*",);
                      //  println!("{:?}",ele);
                    }
                }
            },
            _ => (),
        }
        return leave_info;
    }
