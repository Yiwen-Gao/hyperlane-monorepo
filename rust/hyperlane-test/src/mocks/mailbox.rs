#![allow(non_snake_case)]

use async_trait::async_trait;
use mockall::*;

use hyperlane_core::*;

mock! {
    pub MailboxContract {
        // Mailbox
        pub fn _address(&self) -> H256 {}

        pub fn _domain(&self) -> &HyperlaneDomain {}

        pub fn _domain_hash(&self) -> H256 {}

        pub fn _raw_message_by_id(
            &self,
            leaf: H256,
        ) -> ChainResult<Option<RawHyperlaneMessage>> {}

        pub fn _id_by_nonce(
            &self,
            nonce: usize,
        ) -> ChainResult<Option<H256>> {}

        pub fn _count(&self) -> ChainResult<u32> {}

        pub fn _latest_checkpoint(&self, maybe_lag: Option<u64>) -> ChainResult<Checkpoint> {}

        pub fn _default_ism(&self) -> ChainResult<H256> {}

        pub fn _delivered(&self, id: H256) -> ChainResult<bool> {}

        pub fn process(
            &self,
            message: &HyperlaneMessage,
            metadata: &[u8],
            tx_gas_limit: Option<U256>,
        ) -> ChainResult<TxOutcome> {}

        pub fn process_estimate_costs(
            &self,
            message: &HyperlaneMessage,
            metadata: &[u8],
        ) -> ChainResult<TxCostEstimate> {}

        pub fn process_calldata(
            &self,
            message: &HyperlaneMessage,
            metadata: &[u8],
        ) -> Vec<u8> {}
    }
}

impl std::fmt::Debug for MockMailboxContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MockMailboxContract")
    }
}

#[async_trait]
impl Mailbox for MockMailboxContract {
    async fn count(&self) -> ChainResult<u32> {
        self._count()
    }

    async fn latest_checkpoint(&self, maybe_lag: Option<u64>) -> ChainResult<Checkpoint> {
        self._latest_checkpoint(maybe_lag)
    }

    async fn default_ism(&self) -> ChainResult<H256> {
        self._default_ism()
    }

    async fn delivered(&self, id: H256) -> ChainResult<bool> {
        self._delivered(id)
    }

    async fn process(
        &self,
        message: &HyperlaneMessage,
        metadata: &[u8],
        tx_gas_limit: Option<U256>,
    ) -> ChainResult<TxOutcome> {
        self.process(message, metadata, tx_gas_limit)
    }

    async fn process_estimate_costs(
        &self,
        message: &HyperlaneMessage,
        metadata: &[u8],
    ) -> ChainResult<TxCostEstimate> {
        self.process_estimate_costs(message, metadata)
    }

    fn process_calldata(&self, message: &HyperlaneMessage, metadata: &[u8]) -> Vec<u8> {
        self.process_calldata(message, metadata)
    }
}

impl HyperlaneChain for MockMailboxContract {
    fn domain(&self) -> &HyperlaneDomain {
        self._domain()
    }
}

impl HyperlaneContract for MockMailboxContract {
    fn address(&self) -> H256 {
        self._address()
    }
}
