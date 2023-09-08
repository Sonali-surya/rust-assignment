


pub fn read_leave_data(file_path:String, id : i32, temp :&mut HashMap<String,Leave>) -> Result<(),io::Error>{
    let mut excel = Excel::open(file_path).unwrap();
    let r = excel.worksheet_range("Sheet1").unwrap();
    let mut flag =true;
    for row in r.rows() {
        if file_name == "leave"{
            let emp_id = match row[0] {
                DataType::Float(v) => v.to_string().parse::<i32>().unwrap(),
                _ => 0,

            };

            if emp_id == id{
                let leave_id = match row[1] {
                    DataType::Float(v) => v.to_string().parse::<i32>().unwrap(),
                    _ => 0,
                };
                let leave_from = match row[2]{
                  DataType::Float(v) => v,
                    _ => 0_f64,

                };
                let leave_to = match row[3]{
                    DataType::Float(v) => v,
                    _ => 0_f64,
                };

                let leave_type = match &row[4]{
                    DataType::String(v) => v,
                    _ => "",

                };

                let leave_count = (leave_to - leave_from) as i32;
                let leave_from1 = date_from_float(leave_to);
                println!("leave_from1 {:?}",leave_from1);
                let leave_from2 = date_from_float(leave_from);
                println!("leave_from2 {:?}",leave_from2);
                
                temp.insert(emp_id.to_string(), leave);

            }

        }

        

    }

    Ok(())

}