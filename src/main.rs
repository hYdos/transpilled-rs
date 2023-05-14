pub fn main() {
    println!("Hell.")
}

#[cfg(test)]
mod tests {
    use nx_utils::SwitchExecutable;
    use super::*;

    #[test]
    fn read_nro_file() {
        let bytes = include_bytes!("../test/hello-world.nro").to_vec();
        let nro = SwitchExecutable::read_nro(bytes);
        println!("e {}", 1);
    }
}