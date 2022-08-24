// enum DatabaseError {
//     IsReadOnly = 1,    // user attempted a write operation
//     IOError = 2,       // user should the C errno() for what it was
//     FileCorrupted = 3, // user should run a repair tool to recover it
// }

// impl From<DatabaseError> for libc::c_int {
//     fn from(e: DatabaseError) -> libc::c_int {
//         (e as i8).into()
//     }
// }

pub mod errors {
    pub enum DatabaseError {
        IsReadOnly,
        IOError(std::io::Error),
        FileCorrupted(String),
    }

    impl From<DatabaseError> for libc::c_int {
        fn from(e: DatabaseError) -> libc::c_int {
            match e {
                DatabaseError::IsReadOnly => 1,
                DatabaseError::IOError(_) => 2,
                DatabaseError::FileCorrupted(_) => 3,
            }
        }
    }
}

pub mod c_api {
    use super::errors::DatabaseError;

    #[no_mangle]
    pub extern "C" fn db_error_description(e: *const DatabaseError) -> *mut libc::c_char {
        let error: &DatabaseError = unsafe {
            // SAFETY: pointer lifetime is greater than the current stack frame
            &*e
        };

        let error_str: String = match error {
            DatabaseError::IsReadOnly => {
                format!("cannot write to read-only database")
            }
            DatabaseError::IOError(e) => {
                format!("I/O Error: {}", e)
            }
            DatabaseError::FileCorrupted(s) => {
                format!("File Corrupted, run repair: {}", &s)
            }
        };

        let c_error = unsafe {
            let mut malloc: *mut u8 = libc::malloc(error_str.len() + 1) as *mut _;

            if malloc.is_null() {
                return std::ptr::null_mut();
            }

            let src = error_str.as_bytes().as_ptr();

            std::ptr::copy_nonoverlapping(src, malloc, error_str.len());

            std::ptr::write(malloc.add(error_str.len()), 0);

            malloc as *mut libc::c_char
        };

        c_error
    }
}
