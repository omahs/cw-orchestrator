mod common;

#[cfg(feature = "node-tests")]
mod queriers {

    use cw_orch_core::contract::interface_traits::*;
    use cw_orch_core::environment::TxHandler;
    use cw_orch_daemon::GrpcChannel;
    use cw_orch_networks::networks;
    use ibc_chain_registry::chain::Grpc;
    use ibc_relayer_types::core::ics24_host::identifier::ChainId;
    use mock_contract::InstantiateMsg;
    use speculoos::{asserting, result::ResultAssertions};
    use std::str::FromStr;

    use cw_orch_daemon::{
        queriers::StakingBondStatus,
        queriers::{Bank, CosmWasm, DaemonQuerier, Gov, Ibc, Node, Staking},
        Daemon, DaemonError,
    };
    use tokio::runtime::Runtime;

    use cosmrs::{
        cosmwasm::MsgExecuteContract,
        tx::{self, Msg},
        AccountId, Denom,
    };

    pub async fn build_channel() -> tonic::transport::Channel {
        let network = networks::LOCAL_JUNO;

        let grpcs: Vec<Grpc> = vec![Grpc {
            address: network.grpc_urls[0].into(),
            provider: None,
        }];

        let chain: ChainId = ChainId::new(network.chain_id.to_owned(), 1);

        let channel = GrpcChannel::connect(&grpcs, &chain).await;

        asserting!("channel connection is succesful")
            .that(&channel)
            .is_ok();

        channel.unwrap()
    }

    /*
        Querier - Ibc
    */
    #[test]
    fn ibc() {
        let rt = Runtime::new().unwrap();
        let channel = rt.block_on(build_channel());

        let ibc = Ibc::new(channel);

        let clients = rt.block_on(ibc.clients());
        asserting!("clients is ok").that(&clients).is_ok();
    }

    /*
        Querier - Staking
    */
    #[test]
    fn staking() {
        let rt = Runtime::new().unwrap();
        let channel = rt.block_on(build_channel());

        let staking = Staking::new(channel);

        let params = rt.block_on(staking.params());
        asserting!("params is ok").that(&params).is_ok();

        let validators = rt.block_on(staking.validators(StakingBondStatus::Bonded));
        asserting!("validators is ok").that(&validators).is_ok();
        asserting!("validators is not empty")
            .that(&validators.unwrap().len())
            .is_equal_to(1);
    }

    /*
        Querier - Gov
    */
    #[test]
    fn gov() {
        let rt = Runtime::new().unwrap();
        let channel = rt.block_on(build_channel());

        let gov = Gov::new(channel);

        let params = rt.block_on(gov.params("voting"));
        asserting!("params is ok").that(&params).is_ok();
    }

    /*
        Querier - Bank
    */
    #[test]
    fn bank() {
        let rt = Runtime::new().unwrap();
        let channel = rt.block_on(build_channel());

        let bank = Bank::new(channel);

        let params = rt.block_on(bank.params());
        asserting!("params is ok").that(&params).is_ok();

        let balances =
            rt.block_on(bank.balance("juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y", None));
        asserting!("balances is ok").that(&balances).is_ok();

        let spendable_balances =
            rt.block_on(bank.spendable_balances("juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y"));
        asserting!("spendable_balances is ok")
            .that(&spendable_balances)
            .is_ok();

        let total_supply = rt.block_on(bank.total_supply());
        asserting!("total_supply is ok").that(&total_supply).is_ok();

        let supply_of = rt.block_on(bank.supply_of("ujunox"));
        asserting!("supply_of is ok").that(&supply_of).is_ok();

        let denom_metadata = rt.block_on(bank.denom_metadata("ucosm"));
        asserting!("denom_metadata is err, should not exists")
            .that(&denom_metadata)
            .is_err();

        let denoms_metadata = rt.block_on(bank.denoms_metadata(None));
        asserting!("denoms_metadata is ok, but empty")
            .that(&denoms_metadata)
            .is_ok();
    }

    /*
        Querier - CosmWasm
    */
    #[test]
    fn cosmwasm() {
        let rt = Runtime::new().unwrap();
        let channel = rt.block_on(build_channel());

        let cw = CosmWasm::new(channel);

        let params = rt.block_on(cw.params());
        asserting!("params is ok").that(&params).is_ok();
    }

    /*
        Querier - Node
    */
    #[test]
    fn node() {
        let rt = Runtime::new().unwrap();
        let channel = rt.block_on(build_channel());

        let node = Node::new(channel);

        let block_height = rt.block_on(node.block_height());
        asserting!("block_height is ok").that(&block_height).is_ok();

        let latest_block = rt.block_on(node.latest_block());
        asserting!("latest_block is ok").that(&latest_block).is_ok();

        let block_time = rt.block_on(node.block_time());
        asserting!("block_time is ok").that(&block_time).is_ok();
    }

    #[test]
    #[serial_test::serial]
    fn simulate_tx() {
        let rt = Runtime::new().unwrap();

        let channel = rt.block_on(build_channel());

        let node = Node::new(channel);

        let exec_msg = cw20_base::msg::ExecuteMsg::Mint {
            recipient: "terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr".into(),
            amount: 128u128.into(),
        };

        let exec_msg: MsgExecuteContract = MsgExecuteContract {
            sender: AccountId::from_str(
                "terra1ygcvxp9s054q8u2q4hvl52ke393zvgj0sllahlycm4mj8dm96zjsa45rzk",
            )
            .unwrap(),
            contract: AccountId::from_str(
                "terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26",
            )
            .unwrap(),
            msg: serde_json::to_vec(&exec_msg).unwrap(),
            funds: parse_cw_coins(&[]).unwrap(),
        };

        let msgs = [exec_msg]
            .into_iter()
            .map(Msg::into_any)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let memo = String::from("");

        let body = tx::Body::new(msgs, memo, 100u32);

        let simulate_tx = rt.block_on(node.simulate_tx(body.into_bytes().unwrap()));

        asserting!("that simulate_tx worked but msg is wrong")
            .that(&simulate_tx)
            .is_err();
    }

    #[test]
    #[serial_test::serial]
    fn contract_info() {
        use crate::common::Id;
        use cw_orch_networks::networks;

        let rt = Runtime::new().unwrap();
        let channel = rt.block_on(build_channel());
        let cosm_wasm = CosmWasm::new(channel);
        let daemon = Daemon::builder()
            .chain(networks::LOCAL_JUNO)
            .handle(rt.handle())
            .build()
            .unwrap();

        let sender = daemon.sender();

        let contract = mock_contract::MockContract::new(
            format!("test:mock_contract:{}", Id::new()),
            daemon.clone(),
        );

        contract.upload().unwrap();

        contract
            .instantiate(&InstantiateMsg {}, Some(&sender), None)
            .unwrap();

        let contract_address = contract.address().unwrap();

        let contract_info = rt.block_on(cosm_wasm.contract_info(contract_address));

        asserting!("contract info is ok")
            .that(&contract_info)
            .is_ok();
    }

    fn parse_cw_coins(coins: &[cosmwasm_std::Coin]) -> Result<Vec<cosmrs::Coin>, DaemonError> {
        coins
            .iter()
            .map(|cosmwasm_std::Coin { amount, denom }| {
                Ok(cosmrs::Coin {
                    amount: amount.u128(),
                    denom: Denom::from_str(denom)?,
                })
            })
            .collect::<Result<Vec<_>, DaemonError>>()
    }
}
