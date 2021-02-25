pub trait Animal {
    fn talk(&self) {
        println!("souzou ");
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Status <'a> {
    pub hash: &'a str,
    pub state: f64,
}

impl Status <'_> {
    pub fn test(&self) -> f64 {
        return 0.1
    }

    pub fn getkey(&self) -> f64 {
        return self.state;
    }

    pub fn gethash(&self) -> &str {
        return self.hash;
    }
}

impl Animal for Status <'_> {
    fn talk(&self) {}
}


#[derive(Clone, PartialEq, Debug)]
pub struct Controller <'a> {
    pub key: &'a str,
    pub obj: Box<Animal>
}