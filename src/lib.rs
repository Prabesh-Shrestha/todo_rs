pub mod helper;
#[derive(Debug)]
pub struct Work {
    pub id: i32,
    pub created: String,
    pub dead: String,
    pub detail: String,
}

impl Work {
    pub fn new(id: i32, created: String, dead: String, detail: String) -> Work {
        Work {
            id,
            created,
            dead,
            detail,
        }
    }
}

#[derive(Debug)]
pub struct Group {
    pub name: String,
    pub works: Vec<Work>,
}

impl Group {
    pub fn new(name: String) -> Group {
        Group {
            name,
            works: Vec::new(),
        }
    }
    pub fn show_works(&self) {
        println!("{:#?}", &self.works);
    }
    pub fn add_work(&mut self, new_work: Work) {
        self.works.push(new_work)
    }
    pub fn remove_work(&mut self, id: i32) {
        self.works.retain(|x| x.id != id);
    }
}

pub fn list_group(groups:&Vec<Group>) {
    for group in  groups{
        println!("{}", group.name);
    }
}

pub fn list_all(groups:&Vec<Group>) {
    for group in groups {
        println!("group name: {}", group.name);
        println!("works:");
        for work in &group.works {
            println!("{}", work.detail);
        }
        println!("********************************************")
    }
}
pub fn list_work(groups: &Vec<Group>, name : &str) {
    for group in groups {
        if group.name.to_string() == name.to_string() {
            for work in &group.works {
                println!("{}", work.detail)
            }
        }
    }
}

pub fn add_work_in_group(groups: &mut Vec<Group>, name: &str, new_work: &str) {
    for group in groups {
        if group.name.to_string() == name.to_string() {
            group.add_work(Work::new(
                1,
                String::from("sussy baka"),
                String::from("sussy baka"),
                new_work.to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn adding_work() {
        let mut test_group = Group::new("Test".to_string());
        test_group.add_work(Work::new(
            1,
            0.to_string(),
            20.to_string(),
            "this is ur mum's work".to_string(),
        ));
        assert_eq!(test_group.works.len(), 1);
    }

    #[test]
    fn removing_work() {
        let mut test_group = Group::new("Test".to_string());
        test_group.add_work(Work::new(
            1,
            0.to_string(),
            20.to_string(),
            "this is ur mum's work".to_string(),
        ));

        test_group.add_work(Work::new(
            2,
            0.to_string(),
            20.to_string(),
            "this is ur mum's work".to_string(),
        ));
        test_group.show_works();
        test_group.remove_work(2);
        test_group.show_works();
        assert_eq!(test_group.works.len(), 1);
    }

    // #[test]
    // fn adding_work() {
    //     assert_eq!(result, 4);
    // }
}
