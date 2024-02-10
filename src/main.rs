use goose::prelude::*;

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("LoadtestTransaction")
                .register_transaction(transaction!(loadtest_index))
                .register_transaction(transaction!(loadtest_insert_person))
                .register_transaction(transaction!(loadtest_query_person)),
        )
        .execute()
        .await?;
    Ok(())
}

async fn loadtest_index(user: &mut GooseUser) -> TransactionResult {
    match user.get("/").await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

async fn loadtest_insert_person(user: &mut GooseUser) -> TransactionResult {
    match user.get("/insert_person").await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

async fn loadtest_query_person(user: &mut GooseUser) -> TransactionResult {
    match user.get("/query_person").await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
