use anoma::types::address::Address;
use anoma::types::ethereum_events::{EthereumEvent, MultiSignedEthEvent};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct EthMsg {
    body: EthereumEvent,
    seen_by: Vec<Address>,
    voting_power: (u8, u8),
    seen: bool,
}

pub(crate) fn calculate_eth_msgs_state(
    _events: Vec<MultiSignedEthEvent>,
) -> Vec<EthMsg> {
    vec![]
}

#[cfg(test)]
mod test {
    use super::calculate_eth_msgs_state;

    #[test]
    fn test_calculate_eth_msgs_state() {
        assert_eq!(calculate_eth_msgs_state(vec![]), vec![]);
    }
}
