fn return_right_first_sentence(mut sentence : String, iterator : u32) -> String {
            
        let iterator_ok :usize = iterator as usize;    
        let first_variable = String::from ("first");
        let second_variable = String::from("second");
        let third_variable =String::from("third");
        let fourth_variable = String::from("fourth");
    
        let variables : [String ; 4] = [first_variable, second_variable, third_variable, fourth_variable];

        let variable_iteration = &variables[iterator_ok];
        sentence = format!("On the {} day of Christmas, my true love sent to me ", variable_iteration);
    sentence
}

fn get_right_paragraph (mut paragraph : String, iterator : u32) -> String {
        let last_sentence = String::from("a patridge in a pear tree.");
        let first_increment = String::from("two turtle doves, and ");
        let second_increment = String::from("three french hens, ");
        let third_increment = String::from("four calling birds, ");

            if iterator==0 {
                paragraph = String::from(last_sentence);
            } else if iterator==1 {
                paragraph = format!("{}{}", first_increment, last_sentence);
            } else if iterator==2 {
                paragraph = format!("{}{}{}", second_increment, first_increment, last_sentence);
            } else {
                paragraph = format!("{}{}{}{}", third_increment, second_increment, first_increment, last_sentence);
            }
            paragraph
        } 


fn main() {
    
    for i in 0..4 {
        
        let begginning_sentence : String = String::new();
        let right_first_sentence = return_right_first_sentence(begginning_sentence, i);

        let verse = String::new();
        let right_paragraph = get_right_paragraph(verse, i);
        let y = format!("{}{}", right_first_sentence, right_paragraph);

        println!("{}", y)
    }  
}