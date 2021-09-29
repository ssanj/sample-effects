use std::io;

trait IO {
    fn println(&mut self, message: &str);
    fn readln(&mut self) -> io::Result<String>;
}

struct Prod;


impl IO for Prod {
    fn println(&mut self, message: &str) {
        println!("{}", message)
    }

    fn readln(&mut self) -> io::Result<String> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let line = buffer.lines().next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not read stdin line"))?;
        Ok(line.to_owned())
    }
}

fn program<T>(t: &mut T) -> io::Result<()>
  where T: IO
  {
    t.println("What is your name?");
    let name = t.readln()?;
    t.println(&format!("Hello {}", name));
    Ok(())
  }


fn main() {
    let result = program(&mut Prod);
    result.expect("Got an error running program");
    println!("{}", "done")
}

#[cfg(test)]
mod tests {
    use crate::{program, IO};
    use std::io;


    struct Testing {
        output: Vec<String>,
        input: Vec<String>
    }

    impl IO for Testing {
        fn println(&mut self, message: &str) {
            self.output.push(message.to_owned());
        }

        fn readln(&mut self) -> io::Result<String> {
            Ok(self.input.get(0).map(|s| s.to_string()).unwrap())
        }
    }

    fn vec_eq<T:PartialEq + std::fmt::Debug>(vec1: &Vec<T>, vec2: &Vec<T>) -> bool {
        let match_count = vec1.iter().zip(vec2.iter()).filter(|&(a, b)| a == b).count();
        if match_count == vec1.len() && vec1.len() == vec2.len() {
            true
        } else {
            eprintln!("vec_eq failure: vec1: {:?} != vec2 {:?}", vec1, vec2);
            false
        }
    }


    #[test]
    fn it_works() {
        let mut data = Testing { output: vec![], input: vec!["Blee".to_owned()] };
        let result = program(&mut data);
        let _ = result.unwrap();


        assert_eq!(vec_eq(&vec!["What is your xame?".to_owned(), "Hello Blee".to_owned()], &data.output), true);
        assert_eq!(vec_eq(&vec!["Blee".to_owned()], &data.input), true)
    }
}
