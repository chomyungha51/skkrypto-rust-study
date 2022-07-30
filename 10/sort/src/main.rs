pub trait CreditCalculator {
    fn mean(&self)->f32;
    fn best(&self)->f32;
    fn worst(&self)->f32;
    fn next_semester_availavle_credit(&self)->u32;
}

#[derive(Debug, Clone)]
struct class {
    name: String,
    grade: f32
}

struct Junior {
    credit:u32,
    classes:Vec<class>
}

impl CreditCalculator for Junior {
    fn mean(&self) -> f32 {
        let mut total:f32 = 0.;
        for element in &self.classes {
            total += &element.grade;
        }
        return total / (self.credit / 3 )as f32
    }
    fn best(&self) -> f32{
        let mut best = self.classes[0].grade;
        for element in &self.classes {
            if best < element.grade  {
                best = element.grade;
            }
        }
        return best
    }
    fn worst(&self) -> f32 {
        let mut worst = self.classes[0].grade;
        for element in &self.classes {
            if worst > element.grade {
                worst = element.grade;
            }
        }
        return worst
    }
    fn next_semester_availavle_credit(&self) -> u32{
        let mean = self.mean();
        if mean > 4.0 {
            return 21
        }
        else{
            return 18
        }
    }
}

struct Senior {
    credit:u32,
    classes:Vec<class>
}

impl CreditCalculator for Senior {
    fn mean(&self) -> f32 {
        let mut total:f32 = 0.;
        for element in &self.classes {
            total += element.grade;
        }
        return total / (self.credit / 3) as f32
    }
    fn best(&self) -> f32{
        let mut best = self.classes[0].grade;
        for element in &self.classes {
            if best < element.grade {
                best = element.grade;
            }
        }
        return best
    }
    fn worst(&self) -> f32 {
        let mut worst = self.classes[0].grade;
        for element in &self.classes {
            if worst > element.grade {
                worst = element.grade;
            }
        }
        return worst
    }
    fn next_semester_availavle_credit(&self) -> u32{
        let mean = self.mean();
        if mean > 4.0 {
            return 18
        }
        else{
            return 15
        }
    }
}

fn main() {
    const JuniorCredit:u32 = 18;
    const SeniorCredit:u32 = 15;

    let freshman = Junior{
        credit: JuniorCredit, 
        classes: vec!(
            class{
                name: String::from("English"), 
                grade: 4.5,
            },
            class{
                name: String::from("Software"), 
                grade: 4.0,
            },
            class{
                name: String::from("Writing"), 
                grade: 4.0,
            },
            class{
                name: String::from("FreshmanSeminar"), 
                grade: 2.5,
            },
            class{
                name: String::from("Confucianism"), 
                grade: 3.0,
            },
            class{
                name: String::from("Psychology"), 
                grade: 2.5,
            },
        
        )
    };

    let senior = Senior{
        credit: SeniorCredit,
        classes: vec!(
            class{
                name: String::from("Introduction to HCI"),
                grade: 3.0
            },
            class{
                name: String::from("Introduction to Algorithm"),
                grade: 4.0
            },
            class{
                name: String::from("R programming"),
                grade: 4.0
            },class{
                name: String::from("Java programming"),
                grade: 3.5
            },class{
                name: String::from("Introduction to Database"),
                grade: 4.5
            }
        )
    };

    let freshman_mean = freshman.mean();
    let freshman_best = freshman.best();
    let freshman_worst = freshman.worst();
    let freshman_next_semester_available_credit = freshman.next_semester_availavle_credit();
    println!("freshman mean {}\t best {}\t worst {}\t next semester available credit {}\n ", freshman_mean, freshman_best, freshman_worst, freshman_next_semester_available_credit);


    let senior_mean = senior.mean();
    let senior_best = senior.best();
    let senior_worst = senior.worst();
    let senior_next_semester_available_credit = senior.next_semester_availavle_credit();
    println!("senior mean {}\t best {}\t worst {}\t next semester available credit {}\n ", senior_mean, senior_best, senior_worst, senior_next_semester_available_credit);

}
