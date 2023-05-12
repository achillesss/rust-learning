const part_head: &str = "On the {day} day of Christmas, my true love sent to me";
const days: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const part_from_day_one: &str = "A partridge in a pear tree";

fn main() {
    for day in (0..12) {
        let mut l = part_head.replace("{day}", days[day]);
        println!("{}", l)
    }
}
