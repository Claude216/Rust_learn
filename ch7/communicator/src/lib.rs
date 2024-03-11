pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// ------------------ part I -----------------------
// mod network {
//     fn connect() {

//     }
// }

// mod client {
//     fn connect() {
//     }
// }

// embedded modules
// mod network {
//     network::connect
//     fn connect() { 
//     }
//     mod client {
//         network::client::connect 
//         fn connect() {
//         }
//     }
// }
// -------------End of Part I -------------------------


// ----------------- Part II --------------------------
mod client; // Extracting the contents of the client module but leaving the declaration in scr/lib.rs
// mod network {
//     fn connect() {
//     }
//     mod server {
//         fn connect() {
//         }
//     }
// }
// -------------End of Part II ------------------------

// -------------Part III-------------------------------
mod network;
// -------------End of Part III------------------------



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
