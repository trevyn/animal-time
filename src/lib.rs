pub fn now() -> String {
    let minutes = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 60;
    let animal = minutes % (60 * 24); // minute of the day
    let adjective = (minutes / (60 * 24)) - 19022; // day number
    format!(
        "{}-{}",
        include_str!("adjectives.txt")
            .split('\n')
            .nth(adjective as usize)
            .unwrap(),
        include_str!("animals.txt")
            .split('\n')
            .nth(animal as usize)
            .unwrap()
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::now().split('-').collect::<Vec<&str>>().len(), 2);
    }
}
