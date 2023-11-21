pub mod ji
{
    use printers::printer::{Job, Printer};

    pub struct Impress {
        printers: Vec<Printer>,
    }

    impl Impress {
        pub fn new() -> Impress
        {
            Self {
                printers: printers::get_printers(),
            }
        }

        pub fn print(self, filename: &str) -> Job
        {
            let p = self.printers.first().expect("failed to fin printer");
            printers::print_file(p, filename)
        }
    }
}