fn main() {
    println!("Hello, world!");
}

// unrecoverable error

fn connect_to_database(host: Option<String>){
    match host {
        None => {
            panic!("No database host provided");
        }
        Some(host) =>{
            println!("Connect to database {}", host)
        }
    }
}

#[test]
fn test_panic(){
    connect_to_database(Some(String::from("database")));
    connect_to_database(None);
}

// recoverable error (menggunakan Result)

fn connect_cache(host: Option<String>) -> Result<String, String> {
    match host{
        None => {
            Err("No cache host provided".to_string())
        }
        Some(host) => {
            Ok(host)
        }
    }
}

fn connect_email(host: Option<String>) -> Result<String, String> {
    match host{
        None => {
            Err("No cache host provided".to_string())
        }
        Some(host) => {
            Ok(host)
        }
    }
}

#[test]
fn test_recoverable_error(){
    // let cache = connect_cache(Some("localhost".to_string()));
    let cache = connect_cache(None);


    match cache{
        Ok(host) =>{
            println!("success to connect to host : {}", host)
        }
        Err(error) => {
            println!("error with message : {}", error)
        }
    }
}

//  question operator (?)

fn connect_application(host: Option<String>) -> Result<String, String>  {
    /* (TANPA ?)

    let connect_cache = connect_cache(host.clone());
    match connect_cache {
        Ok(_) => {}
        Err(err) => {
            return Err(err)
        }
    }

    let connect_email = connect_email(host.clone());
    match connect_email{
        Ok(_) => {}
        Err(err) => {
            return Err(err)
        }
    }
    Ok("Connect to application"/to_string());
     */

//  MENGGUNAKAN ?

    connect_cache(host.clone())?;
    connect_email(host.clone())?;
    Ok("Connect to application".to_string())
}

#[test]
fn test_application_error(){
    let result = connect_application(Some("localhost".to_string()));
    // let result = connect_application(None);


    match result{
        Ok(host) => {println!("Success to connect to host : {}", host)}
        Err(err) => {println!("Error with message: {}", err)}
    }
}
