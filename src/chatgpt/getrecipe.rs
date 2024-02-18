use openai_rust;
use warp::http;
use crate::Store;

pub async fn get_api_resp(
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let items = store.grocery_list.read().to_owned();
    let ingredients: Vec<_> = items.keys().map(|k| k.to_string()).collect();
    let query_template = "Suggest me a recipe, I have the following ingredients: ";
    let keys_string = ingredients.join(", ");
    println!("{}", query_template.to_owned() + &*keys_string);
    let client = openai_rust::Client::new(&std::env::var("OPENAI_API_KEY").unwrap());
    let args = openai_rust::chat::ChatArguments::new("gpt-3.5-turbo", vec![
        openai_rust::chat::Message {
            role: "user".to_owned(),
            content: (query_template.to_owned() + &*keys_string).to_string(),
        }
    ]);
    let res = client.create_chat(args).await;

    if res.is_err() {
        println!("{:?}", res.as_ref().err());
        Ok(warp::reply::with_status(
            res.as_ref().err().unwrap().to_string(),
            http::StatusCode::OK,
        ))
    } else {
        let choices = res.as_ref().unwrap().choices.to_owned();
        let msg = choices.get(0).unwrap();
        msg.clone().message;
        println!("{:?}", res.as_ref().unwrap().choices.get(0).unwrap().message);
        Ok(warp::reply::with_status(
            msg.to_owned().message.content.to_string(),
            http::StatusCode::CREATED,
        ))
    }
}