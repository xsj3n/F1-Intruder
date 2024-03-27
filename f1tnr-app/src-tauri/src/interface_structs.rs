use pyo3::{pyclass, Py, PyAny, Python};



#[derive(Clone)]
pub struct RequestandPermutation
{
    pub request: Vec<String>,
    pub permutation: Vec<String>
}

impl RequestandPermutation
{
    pub fn new() -> RequestandPermutation
    {
        return RequestandPermutation
        {
            request: Vec::new(),
            permutation: Vec::new()
        };
    }
}

pub struct HttpHeadersC
{ // holds pointers to immutable data passed to C
    pub header: [String; 64],
    pub value:  [String; 64],
    pub init:   bool
}


pub struct HttpResponseDataC
{
    pub headers: HttpHeadersC,
    pub full_response: String,
    pub body: String,
    pub status_code: u16,
    pub total_response_bytes: u32 
}

pub struct HttpResponseDataKeepAliveC
{
    pub len: usize,
    pub http_response_data_c: Vec<HttpResponseDataC>,
}

