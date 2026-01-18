pub struct Config {
    pub day: String,
    pub iteration: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let day = args[1].clone();
        let iteration = args[2].clone();

        Ok(Config { day, iteration })
    }
}
