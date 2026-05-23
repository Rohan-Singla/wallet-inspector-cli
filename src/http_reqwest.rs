use reqwest::blocking::{Client,ClientBuilder};

pub fn send_request (){
    let myclient = Client::new();

    let result = myclient.get("https://google.com").send();

    if(result.is_ok()){
        println!("{:#?}",result.ok().unwrap().text().unwrap());
    }else if (result.is_err()){
        println!("eRROR Occured : {:#?}",result.err());
    }

    // let post_result = myclient.post( )
}