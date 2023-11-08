pub struct Scrum {
    master: String,
}

impl Scrum {
    pub fn new(master: &str) -> Scrum {
        if master == "Pete" {
            panic!("{} is killing the fun.", master);
        } else if master != "Pete" {
            panic!("Does {} one has certificate?", master);
        }

        Self {
            master: master.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Rosi")]
    fn who_is_the_scrum_master() {
        Scrum::new("Pete");
    }
}
