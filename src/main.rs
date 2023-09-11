use clap::Parser;
use std::io::Read;

use std::fs::OpenOptions;
use std::io::Write;

use protobuf::Message;
include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
use example::Person;

#[derive(Debug, Parser)]
pub struct Inp {
    #[clap(short)]
    pub i: String,

    #[clap(short)]
    pub o: String,
}
fn main() {
    let path = Inp::parse();
    let mut input_file = OpenOptions::new()
        .read(true)
        .open(path.i)
        .expect("Cant open input file");
    let mut protobuff_file = OpenOptions::new()
        .append(true)
        .open(path.o)
        .expect("cannot open protobuff  file");
    let mut final_output = OpenOptions::new()
        .append(true)
        .open("./final_output.txt")
        .expect("cannot open final output file");

    let mut msg = String::new();
    input_file
        .read_to_string(&mut msg)
        .expect("Cant read content");

    for line in msg.lines() {
        let mut values = line.split(',');
        let lastname = values.next().expect("No lastname found");
        let firstname = values.next().expect("No firstname found");
        let date_of_birth = values.next().expect("No date of birth");

        let mut output_msg = Person::new();
        output_msg.last_name = lastname.to_string();
        output_msg.first_name = firstname.to_string();
        output_msg.date_of_birth = date_of_birth.to_string();

        
        let protobuff_bytes: Vec<u8> = output_msg.write_to_bytes().unwrap_or("can't write the file into bytes".to_string().into());
        let data_length = protobuff_bytes.len() as u64;
     
        let mut buffer = Vec::new();
        protobuf::CodedOutputStream::vec(&mut buffer)
            .write_raw_varint64(data_length)
            .expect("Failed to write encoded code");

        protobuff_file.write(&buffer).expect("protobuff file write failed");

        let mut output_msg = String::new();
        let record = format!("{:?}\n", protobuff_bytes);
        output_msg.push_str(&record);

        protobuff_file.write(output_msg.as_bytes()).expect("writing of protobuff file failed");

        let in_msg = Person::parse_from_bytes(&protobuff_bytes).unwrap();

        let mut content = String::new();
        content.push_str(in_msg.last_name.as_str());
        content.push_str(",");
        content.push_str(in_msg.first_name.as_str());
        content.push_str(",");
        content.push_str(in_msg.date_of_birth.as_str());
        content.push_str("\n");

        final_output.write(content.as_bytes()).expect("writing of output file failed");
    }
   
}
