use mock_contract::{ExecuteMsg, InstantiateMsg, MigrateMsg, MockContract, QueryMsg};

use cosmwasm_std::Event;
use cw_orch::prelude::{ContractInstance, CwOrchExecute, CwOrchMigrate, CwOrchQuery};

use cosmwasm_std::Addr;
use cw_orch::prelude::CwOrchUpload;
use cw_orch::prelude::{CwOrchInstantiate, Mock};

#[test]
fn test_instantiate() {
    let contract = MockContract::new(
        "test:mock_contract",
        Mock::new(&Addr::unchecked("Ghazshag")),
    );
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg {}, None, None)
        .unwrap();
}

#[test]
fn test_execute() {
    let contract = MockContract::new(
        "test:mock_contract",
        Mock::new(&Addr::unchecked("Ghazshag")),
    );
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg {}, None, None)
        .unwrap();

    let response = contract
        .execute(&ExecuteMsg::FirstMessage {}, None)
        .unwrap();
    response.has_event(
        &Event::new("wasm")
            .add_attribute("_contract_addr", "contract0")
            .add_attribute("action", "first message passed"),
    );

    contract
        .execute(&ExecuteMsg::SecondMessage { t: "".to_string() }, None)
        .unwrap_err();
}

#[test]
fn test_query() {
    let contract = MockContract::new(
        "test:mock_contract",
        Mock::new(&Addr::unchecked("Ghazshag")),
    );
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg {}, None, None)
        .unwrap();

    let response: String = contract.query(&QueryMsg::FirstQuery {}).unwrap();
    assert_eq!(response, "first query passed");

    contract
        .query::<String>(&QueryMsg::SecondQuery { t: "".to_string() })
        .unwrap_err();
}

#[test]
fn test_migrate() {
    let admin = Addr::unchecked("Ghazshag");
    let contract = MockContract::new("test:mock_contract", Mock::new(&admin));
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg {}, Some(&admin), None)
        .unwrap();

    contract
        .migrate(
            &MigrateMsg {
                t: "error".to_string(),
            },
            contract.code_id().unwrap(),
        )
        .unwrap_err();
    let response = contract
        .migrate(
            &MigrateMsg {
                t: "success".to_string(),
            },
            contract.code_id().unwrap(),
        )
        .unwrap();
    assert_eq!(response.events.len(), 1);
}
