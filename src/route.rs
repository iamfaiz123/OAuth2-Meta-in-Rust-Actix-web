use actix_web::web;
use serde::Deserialize;

pub mod get_credentials{
    use std::env;
    use super::*;
    
    

#[derive(Deserialize)]
pub struct Params{
    code : String,
    #[serde(skip_deserializing)]
    #[serde(default)]
    _state : String
}
#[derive(Deserialize,Debug)]
//struct to store Access credentials
pub struct Access{
    access_token:String,
    #[serde(skip_deserializing)]
    #[serde(default)]
    _token_type:String,
    #[serde(skip_deserializing)]
    #[serde(default)]
    _expires_in:f64
}

//struct to user use credentaials
#[derive(Debug,Deserialize)]
#[allow(unused)]
pub struct Credentials{
    id:String,
    name:String
}

#[actix_web::get("/callback")]
pub async fn get_credentials_fn(params:web::Query<Params>)->impl actix_web::Responder{
           let params:Params = params.into_inner();

           //get required credentials
           let client_id:String = env::var("FACEBOOK_APP_ID").unwrap();
           let client_secret:String = env::var("FACEBOOK_APP_SECRET").unwrap();
           // Set up the redirect URL
           let redirect_uri :String= "http://localhost:8080/users/callback".into();
           let code =  params.code;

           let access_token_endpoint = format!("https://graph.facebook.com/v16.0/oauth/access_token?client_id={client_id}&redirect_uri={redirect_uri}&client_secret={client_secret}&code={code}");

           //exchange token
      
           let token_exchange_response  = reqwest::Client::new().get(&access_token_endpoint).header("Accept", "application/json").send().await.unwrap();
           
            //if request gets failed in single time, dont continue
           if token_exchange_response.status() != 200{
                return "login expire, retry"
           }  

           let token:Access = token_exchange_response.json().await.unwrap();
           let token = token.access_token;
      
         //get crdentials of user using access token
         //follow meta graph query to get better idea of the url
        let credententails_end_points = format!("https://graph.facebook.com/me?fields=id,name&access_token={}",token);
        let user_credentails = reqwest::get(&credententails_end_points).await.unwrap();

        //parse json into rust native struct
        let credentials:Credentials = user_credentails.json().await.unwrap();
        println!("{:#?}",credentials);
        "check console, printed successfully"
           
  }
}



pub mod facebook_oauth{

    use oauth2::{AuthUrl,ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl};
use oauth2::basic::{BasicClient};
use std::env;

    #[actix_web::get("/login")]
    pub async fn login()->impl actix_web::Responder{
        
        // Define the Facebook OAuth2 endpoints
        let auth_url = AuthUrl::new("https://www.facebook.com/v15.0/dialog/oauth".into()).unwrap();
        let token_url = TokenUrl::new("https://graph.facebook.com/v15.0/oauth/access_token".into()).unwrap();
    
        // Set up the client credentials
        let client_id = ClientId::new(env::var("FACEBOOK_APP_ID").unwrap());
        let client_secret = ClientSecret::new(env::var("FACEBOOK_APP_SECRET").unwrap());
    
        // Set up the redirect URL
        let redirect_uri = RedirectUrl::new("http://localhost:8080/users/callback".into()).unwrap();
    
        // Set up the client
        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_uri);
    
        // Generate the authorization URL and redirect the user to Facebook
        let (auth_url, _csrf_token) = client.authorize_url(CsrfToken::new_random).add_scope(Scope::new("email".to_string())).url();


        //return thye url to the user, open this into your browser
        auth_url.to_string()
    }
}