mod auth_err;
mod succes_search;
mod search_struct_final;


fn main() {
    // This is a 403 error and is expected the reason for this is just that I wanted to check if it works
    let auth_err_test = auth_err::a();

    // this on will print the json and the status
    // have your query here
    let query = "Python";
    let search_success = succes_search::search(query);

    // Final search with formatted results in a struct
    let final_res = search_struct_final::search(query);

    // final result with the title, description, and thumbnail formatted using a struct
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(final_res);


}