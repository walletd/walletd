// Real Hedera testnet accounts with private keys
pub struct TestnetAccount {
    pub account_id: &'static str,
    pub private_key: &'static str,
    pub description: &'static str,
}

pub const TESTNET_ACCOUNTS: [TestnetAccount; 3] = [
    TestnetAccount {
        account_id: "0.0.4559331",
        private_key: "302e020100300506032b657004220420853f15aecd22706b105da1d709b4ac05b4906170c2b9c7495dff9af49e1391da",
        description: "Test Account 1 (pre-funded)",
    },
    TestnetAccount {
        account_id: "0.0.4524353", 
        private_key: "3030020100300706052b8104000a04220420c14f86e52e12bec051cf1b71966c465fcc752741cd8bd4e7fc23e698b9e97e9e",
        description: "Test Account 2 (pre-funded)",
    },
    TestnetAccount {
        account_id: "0.0.4481147",
        private_key: "3030020100300706052b8104000a04220420842c5ba9ceaedbbab436d807e0223ac4c9f636be72df92f6f5a630e2dcc89de6",
        description: "Test Account 3 (pre-funded)",
    },
];
