use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{from_slice, to_binary, Coin, Decimal, Querier, QuerierResult, QueryRequest, SystemError, Uint128, Addr, Api, StdError, Binary, ContractResult, Response, StdResult, OwnedDeps, SystemResult, WasmQuery};
use std::str::FromStr;
use terra_cosmwasm::{
    ExchangeRateItem, ExchangeRatesResponse, TaxCapResponse, TaxRateResponse, TerraQuery,
    TerraQueryWrapper, TerraRoute,
};

pub const MOCK_HUB_CONTRACT_ADDR: &str = "hub";
pub const MOCK_CW20_CONTRACT_ADDR: &str = "lottery";
//pub const MOCK_REWARD_CONTRACT_ADDR: &str = "reward";
pub const MOCK_TOKEN_CONTRACT_ADDR: &str = "token";

pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let contract_addr = Addr::unchecked(MOCK_CONTRACT_ADDR);
    let custom_querier = MockQuerier::new(&[(&contract_addr.as_str(), contract_balance)]);
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier
    }

}

pub struct WasmMockQuerier {
    base: MockQuerier<TerraQueryWrapper>
}


impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<TerraQueryWrapper> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<TerraQueryWrapper>) -> QuerierResult{
        let contract_result: ContractResult<Binary> =  match request {
            QueryRequest::Custom(TerraQueryWrapper { route, query_data }) => match query_data {
                TerraQuery::TaxRate {} => {
                    let res = TaxRateResponse {
                        rate: Decimal::percent(1),
                    };
                    to_binary(&res).into()
                }
                TerraQuery::TaxCap { denom: _ } => {
                    let cap = Uint128(1u128);
                    let res = TaxCapResponse { cap };
                    to_binary(&res).into()
                }
                _ => panic!("DO NOT ENTER HERE"),
            },
            _ => ContractResult::Err("err".to_string()),
        };
        SystemResult::Ok(contract_result)
    }
}

impl WasmMockQuerier {
    pub fn new(base: MockQuerier<TerraQueryWrapper>) -> Self {
        WasmMockQuerier {
            base
        }
    }
}