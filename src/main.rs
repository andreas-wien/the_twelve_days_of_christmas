/*
On the first day of Christmas, my true love gave to me
A partridge in a pear tree
On the second day of Christmas, my true love gave to me
Two turtle doves and a partridge in a pear tree
On the third day of Christmas, my true love gave to me three French hens
Two turtle doves and a partridge in a pear tree
On the fourth day of Christmas, my true love gave to me
Four calling birds, three French hens, two turtle doves
And a partridge in a pear tree
On the fifth day of Christmas, my true love gave to me
Five golden rings, four calling birds, three French hens
Two turtle doves and a partridge in a pear tree
On the sixth day of Christmas, my true love gave to me
Six geese a-layin', five golden rings, four calling birds
Three French hens, two turtle doves and a partridge in a pear tree
On the seventh day of Christmas, my true love gave to me
Seven swans a-swimmin', six geese a-layin', five golden rings
Four calling birds, three French hens, two turtle doves
And a partridge in a pear tree
On the eighth day of Christmas, my true love gave to me
Eight maids a-milkin', seven swans a-swimmin'
Six geese a-layin', five golden rings, four calling birds, three French hens
Two turtle doves and a partridge in a pear tree
On the ninth day of Christmas, my true love gave to me
Nine lords a-leapin', eight maids a-milkin', seven swans a-swimmin'
Six geese a-layin', five golden rings, four calling birds, three French hens
Two turtle doves and a partridge in a pear tree
On the tenth day of Christmas, my true love gave to me
Ten ladies dancin', nine lords a-leapin', eight maids a-milkin'
Seven swans a-swimmin', six geese a-layin', five golden rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree
On the eleventh day of Christmas, my true love gave to me
Eleven pipers pipin', ten ladies dancin', nine lords a-leapin'
Eight maids a-milkin', seven swans a-swimmin'
Six geese a-layin', five golden rings, four calling birds, three French hens
Two turtle doves and a partridge in a pear tree
On the twelfth day of Christmas, my true love gave to me
Twelve drummers drummin', eleven pipers pipin', ten ladies dancin'
Nine lords a-leapin', eight maids milkin', seven swans a-swimmin'
Six geese a-layin' five golden rings, four calling birds, three French hens
Two turtle doves and a partridge in a pear tree
 */

fn main() {
    let counters = [
        ("a", "first"),
        ("two", "second"),
        ("three", "third"),
        ("four", "fourth"),
        ("five", "fifth"),
        ("six", "sixth"),
        ("seven", "seventh"),
        ("eight", "eighth"),
        ("nine", "ninth"),
        ("ten", "tenth"),
        ("eleven", "eleventh"),
        ("twelve", "twelfth"),
    ];

    let things_to_give = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "golden rings",
        "geese a-layin'",
        "swans a-swimmin'",
        "maids milkin'",
        "lords a-leapin'",
        "ladies dancin'",
        "pipers pipin'",
        "drummers drummin'",
    ];

    for (i, counter) in counters.iter().enumerate() {
        let mut lyrics = format!(
            "On the {} day of Christmas, my true love gave to me",
            counter.1
        );
        let things = &things_to_give[..=i];
        for (j, thing) in things.iter().enumerate().rev() {
            let con = if j == 0 && j != things.len() - 1 {
                " and"
            } else {
                ","
            };
            let line_break = if j % 2 == 1 { "\n" } else { " " };
            lyrics += &format!("{2}{line_break}{0} {1}", counters[j].0, thing, con);
        }
        println!("{lyrics}");
    }
}
