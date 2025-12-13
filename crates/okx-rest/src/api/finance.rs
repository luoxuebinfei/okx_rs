//! 财务产品相关接口（Staking/DeFi/Savings/Simple Earn）。
//!
//! 覆盖 `/api/v5/finance/staking-defi/*`、`/api/v5/finance/savings/*` 及 simple earn 定期接口。

use okx_core::Result;
use serde_json::Value;

use crate::OkxRestClient;

/// 端点常量。
pub mod endpoints {
    // Staking / DeFi
    /// DeFi 产品列表端点
    pub const DEFI_OFFERS: &str = "/api/v5/finance/staking-defi/offers";
    /// DeFi 申购端点
    pub const DEFI_PURCHASE: &str = "/api/v5/finance/staking-defi/purchase";
    /// DeFi 赎回端点
    pub const DEFI_REDEEM: &str = "/api/v5/finance/staking-defi/redeem";
    /// DeFi 取消端点
    pub const DEFI_CANCEL: &str = "/api/v5/finance/staking-defi/cancel";
    /// DeFi 活跃订单端点
    pub const DEFI_ORDERS_ACTIVE: &str = "/api/v5/finance/staking-defi/orders-active";
    /// DeFi 历史订单端点
    pub const DEFI_ORDERS_HISTORY: &str = "/api/v5/finance/staking-defi/orders-history";

    // Savings
    /// 余币宝余额端点
    pub const SAVING_BALANCE: &str = "/api/v5/finance/savings/balance";
    /// 余币宝申购/赎回端点
    pub const SAVING_PURCHASE_REDEMPTION: &str = "/api/v5/finance/savings/purchase-redempt";
    /// 设置出借利率端点
    pub const SAVING_SET_LENDING_RATE: &str = "/api/v5/finance/savings/set-lending-rate";
    /// 出借历史端点
    pub const SAVING_LENDING_HISTORY: &str = "/api/v5/finance/savings/lending-history";
    /// 公共出借利率汇总端点
    pub const SAVING_PUBLIC_LENDING_RATE: &str = "/api/v5/finance/savings/lending-rate-summary";
    /// 出借利率历史端点
    pub const SAVING_LENDING_RATE_HISTORY: &str = "/api/v5/finance/savings/lending-rate-history";

    // Flexible Loan
    /// 弹性借贷可借币种端点
    pub const FLEXIBLE_LOAN_BORROW_CURRENCIES: &str =
        "/api/v5/finance/flexible-loan/borrow-currencies";
    /// 弹性借贷抵押资产端点
    pub const FLEXIBLE_LOAN_COLLATERAL_ASSETS: &str =
        "/api/v5/finance/flexible-loan/collateral-assets";
    /// 弹性借贷最大可借端点
    pub const FLEXIBLE_LOAN_MAX_LOAN: &str = "/api/v5/finance/flexible-loan/max-loan";
    /// 弹性借贷借贷信息端点
    pub const FLEXIBLE_LOAN_LOAN_INFO: &str = "/api/v5/finance/flexible-loan/loan-info";
    /// 弹性借贷历史端点
    pub const FLEXIBLE_LOAN_LOAN_HISTORY: &str = "/api/v5/finance/flexible-loan/loan-history";
    /// 弹性借贷计息记录端点
    pub const FLEXIBLE_LOAN_INTEREST_ACCRUED: &str =
        "/api/v5/finance/flexible-loan/interest-accrued";
    /// 弹性借贷最大可赎回抵押物端点
    pub const FLEXIBLE_LOAN_MAX_REDEEM_AMOUNT: &str =
        "/api/v5/finance/flexible-loan/max-collateral-redeem-amount";
    /// 弹性借贷调整抵押物端点
    pub const FLEXIBLE_LOAN_ADJUST_COLLATERAL: &str =
        "/api/v5/finance/flexible-loan/adjust-collateral";

    // Staking-Defi (ETH)
    /// ETH 质押产品信息端点
    pub const STAKING_DEFI_ETH_PRODUCT_INFO: &str = "/api/v5/finance/staking-defi/eth/product-info";
    /// ETH 质押余额端点
    pub const STAKING_DEFI_ETH_BALANCE: &str = "/api/v5/finance/staking-defi/eth/balance";
    /// ETH 质押 APY 历史端点
    pub const STAKING_DEFI_ETH_APY_HISTORY: &str = "/api/v5/finance/staking-defi/eth/apy-history";
    /// ETH 质押申购端点
    pub const STAKING_DEFI_ETH_PURCHASE: &str = "/api/v5/finance/staking-defi/eth/purchase";
    /// ETH 质押赎回端点
    pub const STAKING_DEFI_ETH_REDEEM: &str = "/api/v5/finance/staking-defi/eth/redeem";
    /// ETH 质押申购/赎回历史端点
    pub const STAKING_DEFI_ETH_PURCHASE_REDEEM_HISTORY: &str =
        "/api/v5/finance/staking-defi/eth/purchase-redeem-history";

    // Staking-Defi (SOL)
    /// SOL 质押产品信息端点
    pub const STAKING_DEFI_SOL_PRODUCT_INFO: &str = "/api/v5/finance/staking-defi/sol/product-info";
    /// SOL 质押余额端点
    pub const STAKING_DEFI_SOL_BALANCE: &str = "/api/v5/finance/staking-defi/sol/balance";
    /// SOL 质押 APY 历史端点
    pub const STAKING_DEFI_SOL_APY_HISTORY: &str = "/api/v5/finance/staking-defi/sol/apy-history";
    /// SOL 质押申购端点
    pub const STAKING_DEFI_SOL_PURCHASE: &str = "/api/v5/finance/staking-defi/sol/purchase";
    /// SOL 质押赎回端点
    pub const STAKING_DEFI_SOL_REDEEM: &str = "/api/v5/finance/staking-defi/sol/redeem";
    /// SOL 质押申购/赎回历史端点
    pub const STAKING_DEFI_SOL_PURCHASE_REDEEM_HISTORY: &str =
        "/api/v5/finance/staking-defi/sol/purchase-redeem-history";

    // Simple Earn Fixed Loan (official SimpleEarnFixedAPI)
    /// 简单赚币出借产品端点
    pub const SIMPLE_EARN_LENDING_OFFERS: &str = "/api/v5/finance/fixed-loan/lending-offers";
    /// 简单赚币出借 APY 历史端点
    pub const SIMPLE_EARN_LENDING_APY_HISTORY: &str =
        "/api/v5/finance/fixed-loan/lending-apy-history";
    /// 简单赚币待处理出借量端点
    pub const SIMPLE_EARN_PENDING_LENDING_VOLUME: &str =
        "/api/v5/finance/fixed-loan/pending-lending-volume";
    /// 简单赚币下出借订单端点
    pub const SIMPLE_EARN_LENDING_ORDER: &str = "/api/v5/finance/fixed-loan/lending-order";
    /// 简单赚币修改出借订单端点
    pub const SIMPLE_EARN_AMEND_LENDING_ORDER: &str =
        "/api/v5/finance/fixed-loan/amend-lending-order";
    /// 简单赚币出借订单列表端点
    pub const SIMPLE_EARN_LENDING_ORDERS_LIST: &str =
        "/api/v5/finance/fixed-loan/lending-orders-list";
    /// 简单赚币出借子订单端点
    pub const SIMPLE_EARN_LENDING_SUB_ORDERS: &str =
        "/api/v5/finance/fixed-loan/lending-sub-orders";
}

/// 财务产品 API。
pub trait FinanceApi {
    // Staking / DeFi
    /// 获取 DeFi 产品列表。
    fn defi_get_offers(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// DeFi 产品申购。
    fn defi_purchase(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// DeFi 产品赎回。
    fn defi_redeem(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 取消 DeFi 订单。
    fn defi_cancel(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取 DeFi 活跃订单。
    fn defi_orders_active(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取 DeFi 历史订单。
    fn defi_orders_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    // Savings
    /// 获取余币宝余额。
    fn saving_balance(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 余币宝申购/赎回。
    fn saving_purchase_redemption(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 设置出借利率。
    fn saving_set_lending_rate(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取出借历史。
    fn saving_lending_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取公共出借利率汇总。
    fn saving_public_lending_rate(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取出借利率历史。
    fn saving_lending_rate_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    // Flexible Loan
    /// 获取弹性借贷可借币种。
    fn flexible_loan_borrow_currencies(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取弹性借贷抵押资产。
    fn flexible_loan_collateral_assets(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取弹性借贷最大可借额度。
    fn flexible_loan_max_loan(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取弹性借贷最大可赎回抵押物。
    fn flexible_loan_max_collateral_redeem_amount(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 调整弹性借贷抵押物。
    fn flexible_loan_adjust_collateral(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取弹性借贷借贷信息。
    fn flexible_loan_loan_info(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取弹性借贷历史。
    fn flexible_loan_loan_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取弹性借贷计息记录。
    fn flexible_loan_interest_accrued(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    // Staking-Defi ETH
    /// 获取 ETH 质押产品信息。
    fn staking_defi_eth_product_info(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// ETH 质押申购。
    fn staking_defi_eth_purchase(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// ETH 质押赎回。
    fn staking_defi_eth_redeem(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取 ETH 质押余额。
    fn staking_defi_eth_balance(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取 ETH 质押申购/赎回历史。
    fn staking_defi_eth_purchase_redeem_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取 ETH 质押 APY 历史。
    fn staking_defi_eth_apy_history(
        &self,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    // Staking-Defi SOL
    /// 获取 SOL 质押产品信息。
    fn staking_defi_sol_product_info(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// SOL 质押申购。
    fn staking_defi_sol_purchase(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// SOL 质押赎回。
    fn staking_defi_sol_redeem(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取 SOL 质押余额。
    fn staking_defi_sol_balance(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取 SOL 质押申购/赎回历史。
    fn staking_defi_sol_purchase_redeem_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取 SOL 质押 APY 历史。
    fn staking_defi_sol_apy_history(
        &self,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;

    // Simple Earn Fixed Loan
    /// 获取简单赚币出借产品列表。
    fn simple_earn_get_lending_offers(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取简单赚币出借 APY 历史。
    fn simple_earn_get_lending_apy_history(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取简单赚币待处理出借量。
    fn simple_earn_get_pending_lending_volume(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 下简单赚币出借订单。
    fn simple_earn_place_lending_order(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 修改简单赚币出借订单。
    fn simple_earn_amend_lending_order(
        &self,
        request: Value,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取简单赚币出借订单列表。
    fn simple_earn_get_lending_orders_list(
        &self,
        params: Option<Value>,
    ) -> impl std::future::Future<Output = Result<Vec<Value>>> + Send;
    /// 获取简单赚币出借子订单。
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
