fn main() {
    let days = [
        "first", 
        "second", 
        "third", 
        "fourth", 
        "fifth", 
        "sixth", 
        "seventh", 
        "eighth", 
        "ninth", 
        "tenth", 
        "eleventh", 
        "twelfth"
    ];

    let surprices = [
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Twelve lords-a-leaping",
        "Eleven ladies dancing",
        "Ten pipers piping",
        "Nine drummers drumming",
        "Eight maids-a-milking!",
        "Seven swans-a-swimming!",
        "Six geese-a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
    ];

    let count = days.len();

    let mut getted_surpices = String::new();

    for index in 0..count {
        let header = format!("Oh the {} day of Christmas\nMy true love gave to me\n", days[index]);
        if index == 0 {
            println!("{}A partridge in a pear tree\n", header);
            getted_surpices = format!("{}\n{}", surprices[index], getted_surpices);
        } else {
            getted_surpices = format!("{}\n{}", surprices[index], getted_surpices);
            println!("{header}{getted_surpices}");
        }
        
    }
    
}
