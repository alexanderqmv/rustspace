struct Person {
    job: Option<Job>,
}

struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job.as_ref().and_then(|j| j.phone_number.as_ref()).and_then(|p| p.area_code)
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(205),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(205));
}
