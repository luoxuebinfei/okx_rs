//! 财务产品相关接口（Staking/DeFi/Savings/Simple Earn）。
//!
//! 覆盖 `/api/v5/finance/staking-defi/*`、`/api/v5/finance/savings/*` 及 simple earn 定期接口。

use okx_core::Result;
use serde_json::Value;

use crate::OkxRestClient;

/// 端点常量。
pub mod endpoints {
    // Staking / DeFi
    pub const DEFI_OFFERS: &str = "/api/v5/finance/staking-defi/offers";
    pub const DEFI_PURCHASE: &str = "/api/v5/finance/staking-defi/purchase";
    pub const DEFI_REDEEM: &str = "/api/v5/finance/staking-defi/redeem";
    pub const DEFI_CANCEL: &str = "/api/v5/finance/staking-defi/cancel";
    pub const DEFI_ORDERS_ACTIVE: &str = "/api/v5/finance/staking-defi/orders-active";
    pub const DEFI_ORDERS_HISTORY: &str = "/api/v5/finance/staking-defi/orders-history";

    // Savings
    pub const SAVING_BALANCE: &str = "/api/v5/finance/savings/balance";
    pub const SAVING_PURCHASE_REDEMPTION: &str = "/api/v5/finance/savings/purchase-redempt";
    pub const SAVING_SET_LENDING_RATE: &str = "/api/v5/finance/savings/set-lending-rate";
    pub const SAVING_LENDING_HISTORY: &str = "/api/v5/finance/savings/lending-history";
    pub const SAVING_PUBLIC_LENDING_RATE: &str = "/api/v5/finance/savings/lending-rate-summary";
    pub const SAVING_LENDING_RATE_HISTORY: &str = "/api/v5/finance/savings/lending-rate-history";

    // Flexible Loan
    pub const FLEXIBLE_LOAN_BORROW_CURRENCIES: &str =
        "/api/v5/finance/flexible-loan/borrow-currencies";
    pub const FLEXIBLE_LOAN_COLLATERAL_ASSETS: &str =
        "/api/v5/finance/flexible-loan/collateral-assets";
    pub const FLEXIBLE_LOAN_MAX_LOAN: &str = "/api/v5/finance/flexible-loan/max-loan";
    pub const FLEXIBLE_LOAN_LOAN_INFO: &str = "/api/v5/finance/flexible-loan/loan-info";
    pub const FLEXIBLE_LOAN_LOAN_HISTORY: &str = "/api/v5/finance/flexible-loan/loan-history";
    pub const FLEXIBLE_LOAN_INTEREST_ACCRUED: &str =
        "/api/v5/finance/flexible-loan/interest-accrued";
    pub const FLEXIBLE_LOAN_MAX_REDEEM_AMOUNT: &str =
        "/api/v5/finance/flexible-loan/max-collateral-redeem-amount";
    pub const FLEXIBLE_LOAN_ADJUST_COLLATERAL: &str =
        "/api/v5/finance/flexible-loan/adjust-collateral";

    // Staking-Defi (ETH)
    pub const STAKING_DEFI_ETH_PRODUCT_INFO: &str = "/api/v5/finance/staking-defi/eth/product-info";
    pub const STAKING_DEFI_ETH_BALANCE: &str = "/api/v5/finance/staking-defi/eth/balance";
    pub const STAKING_DEFI_ETH_APY_HISTORY: &str = "/api/v5/finance/staking-defi/eth/apy-history";
    pub const STAKING_DEFI_ETH_PURCHASE: &str = "/api/v5/finance/staking-defi/eth/purchase";
    pub const STAKING_DEFI_ETH_REDEEM: &str = "/api/v5/finance/staking-defi/eth/redeem";
    pub const STAKING_DEFI_ETH_PURCHASE_REDEEM_HISTORY: &str =
        "/api/v5/finance/staking-defi/eth/purchase-redeem-history";

    // Staking-Defi (SOL)
    pub const STAKING_DEFI_SOL_PRODUCT_INFO: &str = "/api/v5/finance/staking-defi/sol/product-info";
    pub const STAKING_DEFI_SOL_BALANCE: &str = "/api/v5/finance/staking-defi/sol/balance";
    pub const STAKING_DEFI_SOL_APY_HISTORY: &str = "/api/v5/finance/staking-defi/sol/apy-history";
    pub const STAKING_DEFI_SOL_PURCHASE: &str = "/api/v5/finance/staking-defi/sol/purchase";
    pub const STAKING_DEFI_SOL_REDEEM: &str = "/api/v5/finance/staking-defi/sol/redeem";
    pub const STAKING_DEFI_SOL_PURCHASE_REDEEM_HISTORY: &str =
        "/api/v5/finance/staking-defi/sol/purchase-redeem-history";

    // Simple Earn Fixed Loan (official SimpleEarnFixedAPI)
    pub const SIMPLE_EARN_LENDING_OFFERS: &str = "/api/v5/finance/fixed-loan/lending-offers";
    pub const SIMPLE_EARN_LENDING_APY_HISTORY: &str =
        "/api/v5/finance/fixed-loan/lending-apy-history";
    pub const SIMPLE_EARN_PENDING_LENDING_VOLUME: &str =
        "/api/v5/finance/fixed-loan/pending-lending-volume";
    pub const SIMPLE_EARN_LENDING_ORDER: &str = "/api/v5/finance/fixed-loan/lending-order";
    pub const SIMPLE_EARN_AMEND_LENDING_ORDER: &str =
        "/api/v5/finance/fixed-loan/amend-lending-order";
    pub const SIMPLE_EARN_LENDING_ORDERS_LIST: &str =
        "/api/v5/finance/fixed-loan/lending-orders-list";
    pub const SIMPLE_EARN_LENDING_SUB_ORDERS: &str =
        "/api/v5/finance/fixed-loan/lending-sub-orders";
}

/// 财务产品 API。
pub trait FinanceApi {
    // Staking / DeFi
    fn defi_get_offers(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn defi_purchase(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn defi_redeem(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn defi_cancel(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn defi_orders_active(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn defi_orders_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    // Savings
    fn saving_balance(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn saving_purchase_redemption(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn saving_set_lending_rate(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn saving_lending_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn saving_public_lending_rate(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn saving_lending_rate_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    // Flexible Loan
    fn flexible_loan_borrow_currencies(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn flexible_loan_collateral_assets(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn flexible_loan_max_loan(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn flexible_loan_max_collateral_redeem_amount(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn flexible_loan_adjust_collateral(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn flexible_loan_loan_info(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn flexible_loan_loan_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn flexible_loan_interest_accrued(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    // Staking-Defi ETH
    fn staking_defi_eth_product_info(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn staking_defi_eth_purchase(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn staking_defi_eth_redeem(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn staking_defi_eth_balance(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn staking_defi_eth_purchase_redeem_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn staking_defi_eth_apy_history(
        &self,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    // Staking-Defi SOL
    fn staking_defi_sol_product_info(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn staking_defi_sol_purchase(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn staking_defi_sol_redeem(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn staking_defi_sol_balance(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn staking_defi_sol_purchase_redeem_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn staking_defi_sol_apy_history(
        &self,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    // Simple Earn Fixed Loan
    fn simple_earn_get_lending_offers(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn simple_earn_get_lending_apy_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn simple_earn_get_pending_lending_volume(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn simple_earn_place_lending_order(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn simple_earn_amend_lending_order(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn simple_earn_get_lending_orders_list(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    fn simple_earn_get_lending_sub_orders(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
}

impl FinanceApi for OkxRestClient {
    // Staking / DeFi
    async fn defi_get_offers(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::DEFI_OFFERS, params.as_ref()).await
    }
    async fn defi_purchase(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::DEFI_PURCHASE, &request).await
    }
    async fn defi_redeem(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::DEFI_REDEEM, &request).await
    }
    async fn defi_cancel(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::DEFI_CANCEL, &request).await
    }
    async fn defi_orders_active(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::DEFI_ORDERS_ACTIVE, params.as_ref())
            .await
    }
    async fn defi_orders_history(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::DEFI_ORDERS_HISTORY, params.as_ref())
            .await
    }

    // Savings
    async fn saving_balance(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::SAVING_BALANCE, params.as_ref()).await
    }
    async fn saving_purchase_redemption(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::SAVING_PURCHASE_REDEMPTION, &request)
            .await
    }
    async fn saving_set_lending_rate(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::SAVING_SET_LENDING_RATE, &request)
            .await
    }
    async fn saving_lending_history(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::SAVING_LENDING_HISTORY, params.as_ref())
            .await
    }
    async fn saving_public_lending_rate(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get_public(endpoints::SAVING_PUBLIC_LENDING_RATE, params.as_ref())
            .await
    }

    async fn saving_lending_rate_history(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get_public(endpoints::SAVING_LENDING_RATE_HISTORY, params.as_ref())
            .await
    }

    // Flexible Loan
    async fn flexible_loan_borrow_currencies(&self) -> Result<Vec<Value>> {
        self.get(endpoints::FLEXIBLE_LOAN_BORROW_CURRENCIES, None::<&()>)
            .await
    }

    async fn flexible_loan_collateral_assets(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::FLEXIBLE_LOAN_COLLATERAL_ASSETS, params.as_ref())
            .await
    }

    async fn flexible_loan_max_loan(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::FLEXIBLE_LOAN_MAX_LOAN, &request).await
    }

    async fn flexible_loan_max_collateral_redeem_amount(
        &self,
        params: Option<Value>,
    ) -> Result<Vec<Value>> {
        self.get(endpoints::FLEXIBLE_LOAN_MAX_REDEEM_AMOUNT, params.as_ref())
            .await
    }

    async fn flexible_loan_adjust_collateral(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::FLEXIBLE_LOAN_ADJUST_COLLATERAL, &request)
            .await
    }

    async fn flexible_loan_loan_info(&self) -> Result<Vec<Value>> {
        self.get(endpoints::FLEXIBLE_LOAN_LOAN_INFO, None::<&()>).await
    }

    async fn flexible_loan_loan_history(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::FLEXIBLE_LOAN_LOAN_HISTORY, params.as_ref())
            .await
    }

    async fn flexible_loan_interest_accrued(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::FLEXIBLE_LOAN_INTEREST_ACCRUED, params.as_ref())
            .await
    }

    // Staking-Defi ETH
    async fn staking_defi_eth_product_info(&self) -> Result<Vec<Value>> {
        self.get(endpoints::STAKING_DEFI_ETH_PRODUCT_INFO, None::<&()>)
            .await
    }

    async fn staking_defi_eth_purchase(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::STAKING_DEFI_ETH_PURCHASE, &request)
            .await
    }

    async fn staking_defi_eth_redeem(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::STAKING_DEFI_ETH_REDEEM, &request).await
    }

    async fn staking_defi_eth_balance(&self) -> Result<Vec<Value>> {
        self.get(endpoints::STAKING_DEFI_ETH_BALANCE, None::<&()>)
            .await
    }

    async fn staking_defi_eth_purchase_redeem_history(
        &self,
        params: Option<Value>,
    ) -> Result<Vec<Value>> {
        self.get(
            endpoints::STAKING_DEFI_ETH_PURCHASE_REDEEM_HISTORY,
            params.as_ref(),
        )
        .await
    }

    async fn staking_defi_eth_apy_history(&self, params: Value) -> Result<Vec<Value>> {
        self.get(endpoints::STAKING_DEFI_ETH_APY_HISTORY, Some(&params))
            .await
    }

    // Staking-Defi SOL
    async fn staking_defi_sol_product_info(&self) -> Result<Vec<Value>> {
        self.get(endpoints::STAKING_DEFI_SOL_PRODUCT_INFO, None::<&()>)
            .await
    }

    async fn staking_defi_sol_purchase(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::STAKING_DEFI_SOL_PURCHASE, &request)
            .await
    }

    async fn staking_defi_sol_redeem(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::STAKING_DEFI_SOL_REDEEM, &request).await
    }

    async fn staking_defi_sol_balance(&self) -> Result<Vec<Value>> {
        self.get(endpoints::STAKING_DEFI_SOL_BALANCE, None::<&()>)
            .await
    }

    async fn staking_defi_sol_purchase_redeem_history(
        &self,
        params: Option<Value>,
    ) -> Result<Vec<Value>> {
        self.get(
            endpoints::STAKING_DEFI_SOL_PURCHASE_REDEEM_HISTORY,
            params.as_ref(),
        )
        .await
    }

    async fn staking_defi_sol_apy_history(&self, params: Value) -> Result<Vec<Value>> {
        self.get(endpoints::STAKING_DEFI_SOL_APY_HISTORY, Some(&params))
            .await
    }

    // Simple Earn Fixed Loan
    async fn simple_earn_get_lending_offers(&self, params: Option<Value>) -> Result<Vec<Value>> {
        self.get(endpoints::SIMPLE_EARN_LENDING_OFFERS, params.as_ref())
            .await
    }
    async fn simple_earn_get_lending_apy_history(
        &self,
        params: Option<Value>,
    ) -> Result<Vec<Value>> {
        self.get(endpoints::SIMPLE_EARN_LENDING_APY_HISTORY, params.as_ref())
            .await
    }
    async fn simple_earn_get_pending_lending_volume(
        &self,
        params: Option<Value>,
    ) -> Result<Vec<Value>> {
        self.get(
            endpoints::SIMPLE_EARN_PENDING_LENDING_VOLUME,
            params.as_ref(),
        )
        .await
    }
    async fn simple_earn_place_lending_order(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::SIMPLE_EARN_LENDING_ORDER, &request)
            .await
    }
    async fn simple_earn_amend_lending_order(&self, request: Value) -> Result<Vec<Value>> {
        self.post(endpoints::SIMPLE_EARN_AMEND_LENDING_ORDER, &request)
            .await
    }
    async fn simple_earn_get_lending_orders_list(
        &self,
        params: Option<Value>,
    ) -> Result<Vec<Value>> {
        self.get(endpoints::SIMPLE_EARN_LENDING_ORDERS_LIST, params.as_ref())
            .await
    }
    async fn simple_earn_get_lending_sub_orders(
        &self,
        params: Option<Value>,
    ) -> Result<Vec<Value>> {
        self.get(endpoints::SIMPLE_EARN_LENDING_SUB_ORDERS, params.as_ref())
            .await
    }
}
