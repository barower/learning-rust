fn main() {
    let ordinals = ["zeroth", "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = [
        "Nothing",
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine drummers drumming",
        "Ten pipers piping",
        "Eleven ladies dancing",
        "Twelve Lords a leaping",
    ];

    for i in 1..12 {
        println!("On the {} day of Christmas my true love sent to me", ordinals[i]);

        match i {
            1 | 9 => println!("{}", gifts[i]),
            10 => {
                println!("{}\n", gifts[i]);
                println!("{}, {}", gifts[9], gifts[10].to_lowercase());
                println!("Drumming, piping, drumming, piping");
                for j in (2..9).rev(){
                    println!("{}", gifts[j]);
                }
                println!("And a {}", gifts[1].to_lowercase());
            }
            _ => {
                for j in (2..(i+1)).rev() {
                    println!("{}", gifts[j]);
                }
                if i != 12 {
                    println!("And a {}", gifts[1].to_lowercase());
                } else {
                    println!("And a {}, and a {}", gifts[1].to_lowercase(), gifts[1].to_lowercase());
                }
            }
        }

        if i != 9 {
            println!("");
        }
    }

}
