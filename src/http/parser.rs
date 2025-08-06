pub enum StartLine {
    RequestLine(RequestLine),
    StatusLine(StatusLine)
}

pub struct RequestLine {
    pub method: Method,
    pub request_target: String,
    pub http_version: HttpVersion,
}

pub struct StatusLine { 
    http_version: HttpVersion,
    status_code: u16
    // RFC9112  defines reason-phrase as OPTIONAL
}

enum HttpVersion {
    Http11 // Only HTTP/1.1
}


/*
1xx (Information): the request was received, continuing process
2xx (Successful): the request was received, understood, and accepted
3xx (Redirection): further action needs to be taken in order to complete the request
4xx (Client Error): the request contains bad syntax or cannot be fulfilled
5xx (Server Error): the server failed to fulfill an apparently valid request
*/
pub enum StatusCode {
    BadRequest = 400, // RFC9112 Section 3.2
    MethodNotAllowed = 405, // RFC9110 Section 9.1
    URITooLong = 414, // RFC9112 Section 3
    NotImplemented = 501, // RFC9112 Section 3
    GatewayTimeout = 504 // RFC9111 Section 5.2.2.2
}

#[derive(Debug)]
pub enum Method {
    GET,
    HEAD,
    // RFC9110 mentions that "servers MUST support the methods GET and HEAD. All other methods are OPTIONAL."
}

pub fn parse_start_line(line: &str) -> Result<StartLine, StatusCode> {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();

    if parts.len() < 2 || parts.len() > 3{
        return Err(StatusCode::BadRequest);
    }

    if parts[0].starts_with("HTTP/") {
        let http_version = parse_http_version(parts[0])?;
        let status_code = parts[1].parse::<u16>().map_err(|_| StatusCode::BadRequest)?;

        return Ok(StartLine::StatusLine(StatusLine { http_version, status_code }))
    } else {
        if parts.len() != 3 {
            return Err(StatusCode::BadRequest);
        }

    let method = parse_method(parts[0])?;
    let request_target = parts[1].to_string();
    let http_version = parse_http_version(parts[2])?;

    return Ok(StartLine::RequestLine(RequestLine { method, request_target, http_version }))
    }
}

fn parse_method(s: &str) -> Result<Method, StatusCode> {
    match s {
        "GET" => Ok(Method::GET),
        "HEAD" => Ok(Method::HEAD),
        "POST" | "PUT" | "DELETE" | "PATCH" | "OPTIONS" | "TRACE" | "CONNECT" => {
            Err(StatusCode::NotImplemented)
        },
        _ => Err(StatusCode::BadRequest),
    }
}

fn parse_http_version(s: &str) -> Result<HttpVersion, StatusCode> {
    match s {
        "HTTP/1.1" => Ok(HttpVersion::Http11),
        _ => Err(StatusCode::BadRequest)
    }
}