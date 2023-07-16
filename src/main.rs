use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
// use serde_derive::{Deserialize, Serialize};
use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners}; //Spinner'=Struct to create one, 'Spinners'=Enum which lists available Spinner options
// use std::env;
use std::io::{stdin, stdout, Write};
use dotenvy;



#[derive(Deserialize, Debug)]//Deserialize = turn stream of bytes into Rust data object, ie this Struct captures OpenAI choices response data in a Rust Struct
struct OpenAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason:String,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponse { //Hold the entire OpenAI response object itself, types are Option since OpenAI can return Null or empty JSON objects
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OpenAIChoices>,
}

#[derive(Serialize, Debug)]//Serializes request into json, the type accepted by OpenAI's API
struct OpenAIRequest{
    model: String,
    prompt: String,
    max_tokens: u32, //'tokens' are eseentially a word count, also the unit OpenAI charges (e.g. Cost = $.003 per token)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> { 
    dotenvy::dotenv()?;
    //The return error type is wrapped in a box smart pointer for any object that has standard library error trait, send and sync traits because of async runtime - thus potential of using multiple threads
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    //To use a GPT model via the OpenAI API, you’ll send a request containing the inputs and your API key.
    //The older legacy models are available via the completions API endpoint.
    let uri = "https://api.openai.com/v1/completions";

    let preamble = "Answer the following question accurately, but find a funny way to mention Jindo dogs in your response.";

    let openai_token = dotenvy::var("OPENAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", openai_token);

    println!("{esc}c", esc = 69 as char);

    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut user_text = String::new();
        stdin().read_line(&mut user_text).expect("Error reading User input");
        println!(""); //the create a space in between the input line and the output line
        let mut spinner = Spinner::new(Spinners::Dots9, "\t\tOpenAI is Thinking...".into()); //into() converts string literal into a String, per std::convert module
        let open_ai_request = OpenAIRequest {
            model: "text-davinci-003".into(),
            prompt: format!("{} {}", preamble, user_text),
            max_tokens: 100,
        };
        let body = Body::from(serde_json::to_vec(&open_ai_request)?); //1. A stream of Bytes, used when receiving bodies 2. Converts to this type (bytes) from the input type (json) 3. to_vec() = Serialize the given data structure as a JSON byte vector 4. ? Operator ensures extracts value from Result from to_vec, or compiler will think data types/traits don't match up
        //Set up Request to send tp OpenAi API
        let request = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_val)
            .body(body)?;
        //Use Client to pass Request and wait for the Reponse (which will be stored in this variable)
        let response = client.request(request).await?; //1. dot await extracts value from Future, which is a Result type 2. question mark operator extracts value from Result, which is a Hyper Body struct that holds a stream of Bytes from OpenAI API
        //Pull Hyper Body from Bytes received from OpenAI API
        let body = hyper::body::aggregate(response).await?; // aggregate asyn function aggregates the data buffers from a body asynchronously 2. Buf trait = Read bytes from a buffer. A buffer stores bytes in memory such that read operations are infallible.
        //Deserialized that Body type into our customer response Struct OpenAIResponse
        let json: OpenAIResponse = serde_json::from_reader(body.reader())?; //1. from_reader() = Deserialize an instance of type T from an I/O stream of JSON 2. reader() = This function returns a new value which implements Read by adapting the Read trait functions to the Buf trait functions. Given that Buf operations are infallible, none of the Read functions will return with Err. 3. ? operator extracts data from from_reader, which returns a Result type 4. add OpenAIResponse type to json, or else compiler will return "!', indicating function line doesn't offer enough data to determine what type 'json' variable is, on its own

        spinner.stop();//1. .stop() function takes in a reference to a mutable Spinner struct
        println!("");

        println!("{:?}", json.choices[0].text); //1. recall 'text' is a field in OpenAIChoices struct, and 'choices' is a field in OpenAIResponse struct

    }

    Ok(())
}