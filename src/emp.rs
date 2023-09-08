use std::io::prelude::*;
use std::fs;
use std::io;
use std::collections::HashMap;
use chrono::format::format;

use crate::*;



pub fn read_emp_txt(path1:String,path2:String,path3:String,path4:String)  {
    // open the db file
    let mut final_content:String=String::new();
   
            let record=format!("EmpId#EmpName#DeptTitle#Mobile_no#Email#Salary_status#leave_status\n");
            final_content.push_str(&record);

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(path1).expect("file not found");
    // read its content into a new string   
    let mut content = String::new();
    f.read_to_string(&mut content).expect("file not found");
    
    // allocate an empty HashMap
    // let mut map = HashMap::new();
    let mut flag=false;
    let mut dept_info: HashMap<i32, String> = HashMap::new();
    dept_info=read_dept_excel(path2);
    let mut f2=false;
    // loop over each lines of the file

    let mut sal_info:HashMap<i32,String>=HashMap::new();
    sal_info=read_sal_excel(path3);
   
    let mut leave_info:HashMap<i32,i32>=HashMap::new();
    leave_info=read_leave_excel(path4);
    println!("{:?}",leave_info);

    for entries in content.lines() {

        if(flag==false){
            flag=true;
            continue;
        }

        // split and bind values
        let mut values = entries.split('|');
        let emp_id = values.next().expect("No Key");
        let emp_name = values.next().expect("No Value");
        let dept_id=values.next().expect("No value");
        let mobile_number=values.next().expect("No value");
        let email=values.next().expect("No value");
        let dep_id2:i32=dept_id.to_string().parse().unwrap();
        let emp_id2:i32=emp_id.to_string().parse().unwrap();
        let dept_title=dept_info.get(&dep_id2).unwrap();
        let sal_status=match sal_info.get(&emp_id2){
               Some(val)=>val,
               _=>"None"

        };

        let val:Emp=Emp{
            emp_name:emp_name.to_string(),
            dept_title:dept_title.to_string(),
            mobile_no:mobile_number.to_string(),
            email:email.to_string(),
           
        };
        let emp_id:i32=emp_id.to_string().parse().unwrap();
        let leave_count=match leave_info.get(&emp_id){
            Some(val)=>val,
            _=>&0_i32,
        };
        // map.insert(emp_id,val);
           
            let record=format!("{}#{}#{}#{}#{}#{}#{}\n",emp_id,val.emp_name,val.dept_title,val.mobile_no,val.email,sal_status,leave_count);
            final_content.push_str(&record);
        
         }
         println!("{}",final_content);
         
         std::fs::write("Output.txt", final_content);
         
    }
    // Return Ok
    
