use hello::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

// Include the generated code
mod company_pb {
    include!("company.rs");
}

use company_pb::Company;
use company_pb::Employee;

#[cfg(test)]
mod test_grpc {
    use super::*;

    #[test]
    fn test_grpc() {
        let mut employee1 = Employee::new();
        employee1.set_name("Alice".to_string());
        employee1.set_id(1);
        employee1.set_role(Employee::Role::SOFTWARE_ENGINEER);

        let mut employee2 = Employee::new();
        employee2.set_name("Bob".to_string());
        employee2.set_id(2);
        employee2.set_role(Employee::Role::PRODUCT_MANAGER);

        // Create a company
        let mut company = Company::new();
        company.set_name("Acme Inc.".to_string());
        company.mut_employees().push(employee1);
        company.mut_employees().push(employee2);

        // Serialize the company
        let serialized_company = company.write_to_bytes().unwrap();

        // Deserialize the company
        let deserialized_company = Company::parse_from_bytes(&serialized_company).unwrap();

        // Print the deserialized company
        println!("{:?}", deserialized_company);

    }
}