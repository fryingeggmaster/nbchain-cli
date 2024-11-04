use hex_literal::hex;
use subxt::{
    sp_core::{sr25519,Pair},
    ClientBuilder,
    DefaultConfig,
    DefaultExtra,
    PairSigner,
    sp_runtime::AccountId32
};
use subxt::sp_runtime::MultiAddress;

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod node_runtime { }

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let seed:[u8;32] = hex!["5eb00bbddcf069084889a8ab9155568165f5c453ccb85e70811aaed6f6da5fc1"];
    let seed_account:sr25519::Pair = Pair::from_seed(&seed);

    // let seed_account_id: AccountId32 = seed_account.public().into();
    println!("{:?}",seed_account.public());
    let seed_account_signer =
        PairSigner::<DefaultConfig,DefaultExtra<DefaultConfig>,sr25519::Pair>::new(
            seed_account.clone(),
        );
    let public_key_u8:[u8;32] = hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"];
    let account = AccountId32::from(public_key_u8);
    let api = ClientBuilder::new()
        .set_url(String::from("ws://localhost:9944"))
        .build()
        .await?
        .to_runtime_api::<node_runtime::RuntimeApi<DefaultConfig,DefaultExtra<DefaultConfig>>>();

    let sign:[u8;64] = hex!["2cfbf98f0a26f1b382cac18b2e0e25d1d12e260c63a7118c6433bfc8513ff81f821655682d95690ecbac608cd13396a8353ca4ac79918d7b0dfafb9f7abc2283"];
    let msg = String::from("hello");
    let message = msg.as_bytes().to_vec();
    let s = sign.to_vec();
    let hash = api
        .tx()
        .serve()
        .check(account, message,s)
        .sign_and_submit_then_watch(&seed_account_signer)
        .await?
        .wait_for_finalized()
        .await?;
    // let hash = api
    //     .tx()
    //     .balances()
    //     .transfer(MultiAddress::Id(account), 100_000_000_000)
    //     .sign_and_submit_then_watch(&seed_account_signer)
    //     .await?
    //     .wait_for_finalized()
    //     .await?;

    println!("Balance transfer extrinsic submitted: {:?}", hash);
    Ok(())
}