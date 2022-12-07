mod aoc;

fn main() {
    let filename = "./test";
    let total = aoc::file::load_as(filename, |x| Ok(x.parse::<i32>().unwrap()))
        .unwrap()
        .reduce(|x, y| x + y)
        .unwrap();
    println!("{:#?}", total);
}
