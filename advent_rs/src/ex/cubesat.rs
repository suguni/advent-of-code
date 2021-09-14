#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let sat_a = CubeSat { id: 0 };
        let sat_b = CubeSat { id: 1 };
        let sat_c = CubeSat { id: 2 };

        let a_status = check_status(sat_a);
        println!("a: {:?}", a_status);

        // let a_status = check_status(sat_a);
        // println!("a: {:?}", a_status);
    }
}
