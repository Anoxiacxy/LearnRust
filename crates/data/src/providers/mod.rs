pub mod binance;
pub mod crypto;
pub mod local;
pub mod yahoo;

pub use binance::BinanceDataProvider;
pub use crypto::CryptoDataProvider;
pub use local::LocalDataProvider;
