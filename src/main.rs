
use std::io;
use std::fs::OpenOptions;
use std::io::Write;

use std::env;
use std::path::{Path, PathBuf};



fn main(){

    loop{
        println!("----------------------------------------------------");
        println!("----------------------------------------------------");
        
        println!("this is flutter test code generator program written by manoj , chandra and aishwarya.");
        println!("please press options which will generate code on the basis");
        println!("----------------------------------------------------");
        println!("1. by type");
        println!("2. by text");
        println!("3. by key");
        println!("4. by icon");
        println!("5. to exit");
        let mut  input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        match input.trim().parse::<i8>() {
            Ok(num)=>{
                if num == 5{
                    println!("program exited");
                    break;
                }else{
                    check_options(num);

                }
            },
            Err(_)=>{
                println!("i am unable to parse the input!");
                println!("please type integer number!");
            }  
        }
    }
}


fn check_options(input: i8){
    if input == 1{
        println!("please select  types:");
        println!("1. TextFormFeild");
        println!("2. TextFeild");
        println!("3. ElevetedButton");
        println!("4. DropdownButton");
        println!("5. ListView");
        println!("6. SingleChildScrollView");
        println!("7. DropdownForm");
        // update to wiget new one 
        println!("8: DropDownMenuItem");
        println!("9: Delayed time ");
        println!("10. Default (for managing custom codes)");

        let mut  ops = String::new();
        io::stdin().read_line(&mut ops).expect("failed to read your options");

        match ops.trim().parse::<i8>() {
           Ok(num)=>{

            if num < 11 {
                send_code_by_type(num);
            }else{
                println!("this must be less then 8");
            }
           }
           Err(e)=>{
            println!("sorry the error was {}",e);
           }
        }






    }else if input == 2 {

        let mut  text_input = String::new();
        println!("please enter Text Name");
        io::stdin().read_line(&mut text_input).expect("failed to read your options");
        
        send_by_text_code(&text_input.trim());
    }else if input == 3 {
        let mut  text_input = String::new();
        println!("please enter KeyName:");
        io::stdin().read_line(&mut text_input).expect("failed to read your options");
            send_by_key_code(&text_input.trim());
    }else if input == 4 {
        let mut  icon_name = String::new();
        println!("please enter Icon Name");
        io::stdin().read_line(&mut icon_name).expect("failed to read your options");
        send_code_by_icon(&icon_name.trim());
        
    }else{
        println!("there are only four options.");
        println!("please select from 1 to 4 .");   
    }
}


fn send_by_key_code(key_input: &str){
    println!("//----------------------------------------------------");
    println!("//----------------------------------------------------");
    println!("");
    println!("");

    write_code_to_file("");
    write_code_to_file("");

    let mut sender = format!("expect(find.byKey('{}'), findsOneWidget);",key_input);
    write_code_to_file(&sender);
    println!("{}",sender);

    sender = format!("await tester.tap(find.byKey('{}'));",key_input);
    write_code_to_file(&sender);
    println!("{}",sender);


    write_code_to_file("await Future.delayed(const Duration(seconds: 4));");
    println!("await Future.delayed(const Duration(seconds: 4));");

    write_code_to_file("wait tester.pumpAndSettle();");
    println!("wait tester.pumpAndSettle();");

}

fn send_by_text_code(input:&str){
    println!("//----------------------------------------------------");
    println!("//----------------------------------------------------");
    println!("");
    println!("");

    write_code_to_file("");
    write_code_to_file("");


    let mut sender = format!("expect(find.text('{}'),findsOneWidget);",input);
    write_code_to_file(&sender);
    println!("{}",sender);

    sender = format!("await tester.tap(find.text('{}'));",input);
    write_code_to_file(&sender);
    println!("{}",sender);

    sender = format!("await Future.delayed(const Duration(seconds: 4));");
    write_code_to_file(&sender);
    println!("{}",sender);

    sender = format!("await tester.pumpAndSettle();");
    write_code_to_file(&sender);
    println!("{}",sender);
}

fn send_code_by_type(index :i8){
    let mut vat: &str = "<options>";
    println!("default  template is {}",vat);
    match  index {
        1 => vat = "TextFormField",
        2 => vat = "TextField",
        3 => vat = "ElevetedButton",
        4 => vat = "DropdownButton",
        5 => vat = "ListView",
        6 => vat = "SingleChildScrollView",
        7 => vat = "DropdownForm",
        8 => vat = "DropDownMenuItem",
        9 => vat = "Delayed time",
        _ => vat = "<options>"
    }


    if vat ==  "Delayed time"{
      let   sender = format!("await Future.delayed(const Duration(seconds: 2));");
        write_code_to_file(&sender);
        write_code_to_file("");
        write_code_to_file("");

        println!("await Future.delayed(const Duration(seconds: 2));");
        
    }else{

     

    let mut at_index = String::new();

    loop{
        at_index.clear();

        println!("please enter index of  of your widget.");
        io::stdin().read_line(&mut at_index).expect("failed to read your options");
        match  at_index.trim().parse::<u8>() {
            Ok(num)=>{
                println!("you've selected index {}",num);
                break;
            }
            Err(e)=>{
                println!("sorry while parsing your index we got an error {}",e);
            
            }
        }
    }
    // Trim newline characters

    println!("you have selected {}",vat);
    println!("//----------------------------------------------------");
    println!("//----------------------------------------------------");
    println!("");
    println!("");

    write_code_to_file("");
    write_code_to_file("");

    let mut sender = format!("expect(find.byType({}),findsOneWidget);",vat);


    write_code_to_file(&sender);
    println!("expect(find.byType('{}'),findsOneWidget);",vat);

    
    if vat == "TextFormField" || vat == "TextField" {
        sender = format!("await tester.enterText(find.byType({}).at({}), 'value');",vat,at_index.trim());
    write_code_to_file(&sender);
    println!("await tester.enterText(find.byType({}).at({}), 'value');",vat,at_index.trim());

    }else{
        sender = format!("await tester.tap(find.byType({}).at({}));",vat,at_index.trim());
        write_code_to_file(&sender);
        println!("await tester.tap(find.byType({}).at({}));",vat,at_index.trim());
        
    }

    


    sender = format!("await Future.delayed(const Duration(seconds: 2));");
    write_code_to_file(&sender);
    println!("await Future.delayed(const Duration(seconds: 2));");

    sender = format!("wait tester.pumpAndSettle();");
    write_code_to_file(&sender);
    println!("await tester.pumpAndSettle();");


    }
   
}

fn send_code_by_icon(icon_input: &str){
    println!("//----------------------------------------------------");
    println!("//----------------------------------------------------");
    println!("");
    println!("");

    write_code_to_file("");
    write_code_to_file("");


    let mut sender = format!("expect(find.byIcon({}), findsOneWidget);",icon_input);
    write_code_to_file(&sender);
    println!("{}",sender);

    sender = format!("await tester.tap(find.byIcon({}));",icon_input);
    write_code_to_file(&sender);
    println!("{}",sender);
    
    write_code_to_file("await Future.delayed(const Duration(seconds: 3));");
    println!("await Future.delayed(const Duration(seconds: 3));");
    
    write_code_to_file("await tester.pumpAndSettle();");
    println!("await tester.pumpAndSettle();");
}

fn write_code_to_file(code: &str){

    let data = format!("{} \n",code);
    let desktop_path = get_desktop_path();
    let file_path = desktop_path.join("generator.txt");

    let mut file = match OpenOptions::new().append(true).create(true).open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening a file : {}", e);
            return ;
        }
    };

    if let Err(e) = file.write_all(data.as_bytes()) {
        eprintln!("Error writing to file: {}", e);
    } else {
        println!("code successfully written to generator.txt on desktop.");
    }
}


fn get_desktop_path() -> PathBuf {
    let home_dir = env::var("HOME").ok(); // For Unix-like systems
    let user_profile = env::var("USERPROFILE").ok(); // For Windows

    if let Some(home) = home_dir {
        Path::new(&home).join("Desktop")
    } else if let Some(profile) = user_profile {
        Path::new(&profile).join("Desktop")
    } else {
        panic!("Unable to determine the Desktop path.")
    }
}


// project completed and ready to use 