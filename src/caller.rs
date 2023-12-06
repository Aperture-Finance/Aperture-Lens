/// Call an ephemeral contract and return the decoded data
#[macro_export]
macro_rules! call_ephemeral_contract {
    ($contract:ty, $constructor_args:expr, $decode:ty, $client:expr, $block_id:expr) => {{
        let deployer = <$contract>::deploy($client.clone(), $constructor_args)?;
        let res = $client
            .call(&deployer.deployer.tx, $block_id)
            .await
            .map_err(ContractError::<M>::from_middleware_error);
        match res {
            Err(ContractError::Revert(data)) => match <$decode>::decode(data) {
                Ok(ok) => Ok(ok),
                Err(err) => Err(ContractError::from(err)),
            },
            Err(err) => Err(err),
            Ok(_) => panic!("deployment should revert"),
        }
    }};
}
