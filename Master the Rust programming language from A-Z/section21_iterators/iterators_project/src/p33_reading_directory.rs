use std::{fs, io};
use std::process;


fn practice_33() -> io::Result<()> {
    //simplying the code
    for entry_result in fs::read_dir("./")? {

        // using the ? to terminate the function early is an error occurs
        let entry = entry_result?;
        let entry_name = entry.path();
        let metadata = fs::metadata(&entry_name)?;

        if metadata.is_file() {
            println!("{entry_name:?}\n-------------");
            let contents = fs::read_to_string(&entry_name)?;
            println!("{contents}");
        }


        //simplying the code, skipping the error
        // this will not terminate early even if one file or folder can'be opened
        //if let Ok(entry) = entry_result {
        //    println!("{:?}", entry.path())
        //}

        //long version of the code
        //match entry_result {
        //    Ok(entry) => println!("{:?}", entry.path()),
        //    Err(error) => { eprintln!("Could not read entry: {error}");}
        //}
    }

    Ok(())



    //long version of the code
    //let directory = fs::read_dir("./").unwrap_or_else(|error| {
    //    eprintln!("Could not read directory: {error}");
    //    process::exit(1);
    //});

    //for entry_result in directory {
    //    match entry_result {
    //        Ok(entry) => println!("{:?}", entry.path()),
    //        Err(error) => { eprintln!("Could not read entry: {error}");}
    //    }
    //}
}





