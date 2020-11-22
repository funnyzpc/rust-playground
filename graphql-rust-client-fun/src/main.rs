use graphql_client::{GraphQLQuery, Response};
use std::error::Error;
use reqwest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug",
)]
pub struct BookQuery;

async fn perform_my_query(variables: book_query::Variables) -> Result<(), Box<dyn Error>> {
    let request_body = BookQuery::build_query(variables);
    let client = reqwest::Client::new();
    let res = client.get("http://localhost:8080/graphiql").json(&request_body).send().await?;
    let response_body: Response<book_query::ResponseData> = res.json().await?;
    println!("{:#?}", response_body);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let q = book_query::Variables {
        book_id: Option::from("book-1".to_owned()),
   };
   let result = perform_my_query(q).await;
   println!("{:#?}", result);
   Ok(())
}