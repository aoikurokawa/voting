use std::net::TcpStream;

struct Connection {
    name: String,
    stream: TcpStream,
}

struct Request {}

impl Connection {
    /// Sends a request over the connection.
    ///
    /// # Example
    /// ```no_run
    /// # // Boilerplate are required to get an example working
    /// # let stream = TcpStream::connect("127.0.0.1:34254");
    /// # let connection = Connection { name: "foo".to_owned(), stream };
    /// # let request = Request::new("RequestId", RequestType::Get, "payload")
    /// let response = connection.send_request(request);
    /// assert!(response.is_ok());
    fn send_request(&self, request: Request) -> Result<Status, SendErr> {
        //..
        todo!()
    }

    fn check_status(&self) -> Status {}
}
