

                  fn main(){
                    let mut name = String::new();
                    let mut name1 = String::new();
                    let mut uname=String::new();
                    let mut num= 0;
                    let mut upcu = 0;
                    let mut s_char=0;
                    let mut lowcu = 0;
                    
                    
                    println!("Enter username:");
                    std::io::stdin().read_line(&mut uname).unwrap();

                    println!("Enter Password:");
                    std::io::stdin().read_line(&mut name).expect("failed to read input");
                    let password = name.trim();
                        for n in password.chars(){
                         if n.is_uppercase(){
                             upcu+=1;
                        } else if n.is_lowercase(){
                            lowcu+=1;
                        } else if n.is_numeric(){
                            num+=1;
                        } else{
                            s_char+=1;
                        }
                    }
                    println!("Enter conform Password:");
                    std::io::stdin().read_line(&mut name1).expect("failed to read input");
                    let co_pass=name1.trim();
                        if password== co_pass{
                            if upcu>=1{
                                if lowcu>=1{
                                    if num>=1{
                                        if s_char>=1{
                                            if password.len()>=12{
                                                println!(" Strong Password");
                                            } else if password.len()>=10{
                                                println!("Good Password ");
                                            } else if password.len()>=7{
                                                println!("Weak Password");
                                            } else{
                                                println!("Very Weak Password");
                                            }
                                        }else {
                                            println!("Enter minimum 1 special character");
                                        } 
                                    } else{
                                        println!("Enter minimum 1 digit");
                                    }
                                        
                                    }else {
                                        println!("Enter minimum 1 lower case");
                                    }
                                        
                            } else{
                                println!("Enter minimum 1 uppercase case");
                            }
                            println!("match password");
                        }else{
                            println!("not match password");
                        }        
                       
                            

}                        





                
            


