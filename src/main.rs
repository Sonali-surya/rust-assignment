use clap::{Parser, arg};
use calamine::Reader;
use calamine::open_workbook;
use calamine::Xlsx;
use calamine::DataType;
use std::fs;
use std::collections::HashMap;
mod leave;
use leave::read_leave_excel;


use crate::dept::read_dept_excel;
use crate::emp::read_emp_txt;
// use crate::leave::read_leave_excel;
use crate::salary::read_sal_excel;
mod dept;
mod emp;
// mod leave2;
mod salary;



#[derive(Parser)]
struct Input{
    #[clap(long)]
       e:String,
    #[clap(long)]
        d:String,
    #[clap(long)]
        s:String,
    #[clap(long)]
        l:String,
    #[clap(long)]
        o:String,
    
}
#[derive(Debug)]
pub struct Emp{
     emp_name:String,
     dept_title:String,
     mobile_no:String,
     email:String,
}
pub struct Output_data{
    emp_id:i32,
    emp_name:String,
    dept_title:String,
    mob_number:String,
    email:String,
    sal_stat:String,
}
fn main() {
    let Input_file:Input=Input::parse();
    println!("{},{},{},{},{}",Input_file.e,Input_file.d,Input_file.s,Input_file.l,Input_file.o);
    // .txt
        //   let mut emp_info:HashMap<i32,&Emp>=HashMap::new();
          read_emp_txt(Input_file.e,Input_file.d,Input_file.s,Input_file.l);
    
       
        //    let mut sal_info:HashMap<i32,String>=HashMap::new();
        //    sal_info=read_sal_excel(Input_file.s);
            // println!("{:#?}",sal_info);
       
}
