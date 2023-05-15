const PART_HEAD: &str = "On the {day} day of Christmas, my true love sent to me";
const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const PART_FROM_DAY_ONE: &str = "A partridge in a pear tree";
const PART_FROM_DAY_TWO: &str = "Two turtledoves";
const PART_FROM_DAY_THREE: &str = "Three French hens";
const PART_FROM_DAY_FOUR: &str = "Four calling birds";
const PART_FROM_DAY_FIVE: &str = "Five gold rings (five golden rings)";
const PART_FROM_DAY_SIX: &str = "Six geese a-laying";
const PART_FROM_DAY_SEVEN: &str = "Seven swans a-swimming";
const PART_FROM_DAY_EIGHT: &str = "Eight maids a-milking";
const PART_FROM_DAY_NINE: &str = "Nine ladies dancing";
const PART_FROM_DAY_TEN: &str = "Ten lords a-leaping";
const PART_FROM_DAY_ELEVEN: &str = "I sent eleven pipers piping";
const PART_FROM_DAY_TWELVE: &str = "Twelve drummers drumming";

fn main() {
    for day in 0..12 {
        let l = PART_HEAD.replace("{day}", DAYS[day]);
        println!("{}", l,);

        let mut day_one = PART_FROM_DAY_ONE.to_string();
        let mut day_eleven = PART_FROM_DAY_ELEVEN.to_string();

        if day > 10 {
            println!("{}", PART_FROM_DAY_TWELVE);
            day_eleven = day_eleven.replace("I sent e", "E");
            day_one = [day_one.clone(), day_one.clone()].join("\n");
        }

        if day > 9 {
            println!("{}", day_eleven);
        }

        if day > 8 {
            println!("{}", PART_FROM_DAY_TEN);
        }

        if day > 7 {
            println!("{}", PART_FROM_DAY_NINE);
        }

        if day > 6 {
            println!("{}", PART_FROM_DAY_EIGHT);
        }

        if day > 5 {
            println!("{}", PART_FROM_DAY_SEVEN);
        }

        if day > 4 {
            println!("{}", PART_FROM_DAY_SIX);
        }

        if day > 3 {
            println!("{}", PART_FROM_DAY_FIVE);
        }

        if day > 2 {
            println!("{}", PART_FROM_DAY_FOUR);
        }

        if day > 1 {
            println!("{}", PART_FROM_DAY_THREE);
        }

        if day > 0 {
            println!("{}", PART_FROM_DAY_TWO);
            day_one = day_one.replace("A", "And a");
        }

        println!("{}", day_one);
    }
}
