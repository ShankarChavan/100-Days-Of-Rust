
fn find_veg_nonveg(grill :&[&str])->[i32;2]{
    let mut counts=[0,0];
    for skewer in grill{
        if skewer.contains('x'){
            counts[1]+=1;
        }
        else{
            counts[0]+=1;
        }
    }
    counts
}


fn main() {
        let grill=[
            "--xo--x--ox--",
            "--xx--x--xx--",
            "--oo--o--oo--",      
            "--xx--x--ox--",
            "--xx--x--ox--"
        ];
        let grill1=[
            "--oooo-ooo--",
            "--xx--x--xx--",
            "--o---o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"
        ];
        let grill2=[
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----"
        ];
        let counter=find_veg_nonveg(&grill);
        let counter1=find_veg_nonveg(&grill1);
        let counter2=find_veg_nonveg(&grill2);
        println!("{:?}", counter);
        println!("{:?}", counter1);
        println!("{:?}", counter2);
    
}
