fn progress_days(days :&[i32])->i32{
    let mut counter :i32=0;
    let mut curr_val;
    let mut next=0;
    let mut index=0;

    while index<days.len(){
        curr_val=days[index];

        if index<days.len()-1{
            next=days[index+1];
        }
        
        if next>curr_val{
            counter+=1;
        }
        index+=1;
    }
    counter

}

fn main() {
    
    let prog_days_list :[i32;4]=[3, 4, 1, 2];
    let prog_days_list_1 :[i32;5]=[10, 11, 12, 9, 10];
    let prog_days_list_2 :[i32;6]=[6, 5, 4, 3, 2, 9];
    let prog_days_list_3 :[i32;2]=[9, 9];

    let result=progress_days(&prog_days_list);

    let result_1=progress_days(&prog_days_list_1);
    let result_2=progress_days(&prog_days_list_2);
    let result_3=progress_days(&prog_days_list_3);


    println!("Progress is {}",result);
    println!("Progress is {}",result_1);
    println!("Progress is {}",result_2);
    println!("Progress is {}",result_3);

}
