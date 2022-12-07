use std::{
    fs::File,
    io::{self, BufRead, Error},
    path::Path,
};
fn read_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    return Ok(io::BufReader::new(File::open(filename)?).lines());
}

pub(crate) fn load_as<T, F>(filename: &str, parser_fn: F) -> Result<impl Iterator<Item = T>, Error>
where
    F: Fn(&str) -> Result<T, Error> + 'static,
{
    let items = read_lines(filename)
        .unwrap()
        .map(move |x| -> T { parser_fn(&x.unwrap()).unwrap() })
        .into_iter();
    return Ok(items);
}
