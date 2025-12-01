"""Type stubs for okx_py native module."""

from typing import Any, AsyncIterator, Optional

__version__: str

class Credentials:
    """API credentials for OKX."""

    def __init__(
        self, api_key: str, secret_key: str, passphrase: str
    ) -> None:
        """Create new credentials.

        Args:
            api_key: The API key from OKX
            secret_key: The secret key for signing
            passphrase: The passphrase set when creating the API key
        """
        ...

class Config:
    """Client configuration."""

    simulated: bool
    rest_url: str

    def __init__(
        self,
        credentials: Credentials,
        simulated: bool = False,
        timeout_secs: int = 30,
    ) -> None:
        """Create new configuration.

        Args:
            credentials: API credentials
            simulated: Whether to use simulated (demo) trading
            timeout_secs: Request timeout in seconds
        """
        ...

class Balance:
    """Account balance."""

    total_eq: str
    iso_eq: str
    mgn_ratio: str
    details: list[BalanceDetail]

class BalanceDetail:
    """Balance detail for a currency."""

    ccy: str
    eq: str
    avail_bal: str
    frozen_bal: str

class Position:
    """Trading position."""

    inst_id: str
    pos_side: str
    pos: str
    avg_px: str
    upl: str
    lever: str

class Order:
    """Order information."""

    ord_id: str
    cl_ord_id: str
    inst_id: str
    side: str
    ord_type: str
    state: str
    px: str
    sz: str
    acc_fill_sz: str
    avg_px: str

class Ticker:
    """Market ticker."""

    inst_id: str
    last: str
    ask_px: str
    bid_px: str
    high_24h: str
    low_24h: str
    vol_24h: str

class OkxClient:
    """Synchronous OKX REST client."""

    def __init__(self, config: Config) -> None:
        """Create a new OKX client.

        Args:
            config: Client configuration
        """
        ...

    def get_balance(self, ccy: Optional[str] = None) -> list[Balance]:
        """Get account balance.

        Args:
            ccy: Optional currency filter (e.g., "BTC" or "BTC,ETH")

        Returns:
            List of Balance objects
        """
        ...

    def get_positions(
        self,
        inst_type: Optional[str] = None,
        inst_id: Optional[str] = None,
    ) -> list[Position]:
        """Get positions.

        Args:
            inst_type: Optional instrument type filter
            inst_id: Optional instrument ID filter

        Returns:
            List of Position objects
        """
        ...

    def place_order(
        self,
        inst_id: str,
        td_mode: str,
        side: str,
        ord_type: str,
        sz: str,
        px: Optional[str] = None,
        cl_ord_id: Optional[str] = None,
    ) -> str:
        """Place an order.

        Args:
            inst_id: Instrument ID (e.g., "BTC-USDT")
            td_mode: Trade mode (cash, cross, isolated)
            side: Order side (buy, sell)
            ord_type: Order type (market, limit, post_only, fok, ioc)
            sz: Order size
            px: Price (required for limit orders)
            cl_ord_id: Client order ID (optional)

        Returns:
            Order ID
        """
        ...

    def cancel_order(
        self,
        inst_id: str,
        ord_id: Optional[str] = None,
        cl_ord_id: Optional[str] = None,
    ) -> str:
        """Cancel an order.

        Args:
            inst_id: Instrument ID
            ord_id: Order ID (either ord_id or cl_ord_id required)
            cl_ord_id: Client order ID

        Returns:
            Cancelled order ID
        """
        ...

    def get_order(
        self,
        inst_id: str,
        ord_id: Optional[str] = None,
        cl_ord_id: Optional[str] = None,
    ) -> Optional[Order]:
        """Get order details.

        Args:
            inst_id: Instrument ID
            ord_id: Order ID (either ord_id or cl_ord_id required)
            cl_ord_id: Client order ID

        Returns:
            Order object or None
        """
        ...

    def get_orders_pending(
        self,
        inst_type: Optional[str] = None,
        inst_id: Optional[str] = None,
    ) -> list[Order]:
        """Get pending orders.

        Args:
            inst_type: Optional instrument type filter
            inst_id: Optional instrument ID filter

        Returns:
            List of Order objects
        """
        ...

    def get_ticker(self, inst_id: str) -> Optional[Ticker]:
        """Get ticker for an instrument.

        Args:
            inst_id: Instrument ID (e.g., "BTC-USDT")

        Returns:
            Ticker object or None
        """
        ...

    def get_tickers(self, inst_type: str) -> list[Ticker]:
        """Get tickers for all instruments of a type.

        Args:
            inst_type: Instrument type (SPOT, SWAP, FUTURES, OPTION)

        Returns:
            List of Ticker objects
        """
        ...

    def get_instruments(
        self, inst_type: str, inst_id: Optional[str] = None
    ) -> list[dict[str, Any]]:
        """Get instruments.

        Args:
            inst_type: Instrument type (SPOT, MARGIN, SWAP, FUTURES, OPTION)
            inst_id: Optional instrument ID filter

        Returns:
            List of instrument info as dicts
        """
        ...

    def get_system_time(self) -> str:
        """Get system time.

        Returns:
            Server timestamp in milliseconds
        """
        ...

class AsyncOkxClient:
    """Asynchronous OKX REST client."""

    def __init__(self, config: Config) -> None:
        """Create a new async OKX client.

        Args:
            config: Client configuration
        """
        ...

    async def get_balance(self, ccy: Optional[str] = None) -> list[Balance]:
        """Get account balance (async)."""
        ...

    async def get_positions(
        self,
        inst_type: Optional[str] = None,
        inst_id: Optional[str] = None,
    ) -> list[Position]:
        """Get positions (async)."""
        ...

    async def place_order(
        self,
        inst_id: str,
        td_mode: str,
        side: str,
        ord_type: str,
        sz: str,
        px: Optional[str] = None,
        cl_ord_id: Optional[str] = None,
    ) -> str:
        """Place an order (async)."""
        ...

    async def cancel_order(
        self,
        inst_id: str,
        ord_id: Optional[str] = None,
        cl_ord_id: Optional[str] = None,
    ) -> str:
        """Cancel an order (async)."""
        ...

    async def get_order(
        self,
        inst_id: str,
        ord_id: Optional[str] = None,
        cl_ord_id: Optional[str] = None,
    ) -> Optional[Order]:
        """Get order details (async)."""
        ...

    async def get_orders_pending(
        self,
        inst_type: Optional[str] = None,
        inst_id: Optional[str] = None,
    ) -> list[Order]:
        """Get pending orders (async)."""
        ...

    async def get_ticker(self, inst_id: str) -> Optional[Ticker]:
        """Get ticker for an instrument (async)."""
        ...

    async def get_tickers(self, inst_type: str) -> list[Ticker]:
        """Get tickers for all instruments of a type (async)."""
        ...

    async def get_instruments(
        self, inst_type: str, inst_id: Optional[str] = None
    ) -> list[dict[str, Any]]:
        """Get instruments (async)."""
        ...

    async def get_system_time(self) -> str:
        """Get system time (async)."""
        ...

class WsClient:
    """WebSocket client for real-time data streaming."""

    @staticmethod
    async def connect_public(
        config: Config, max_reconnect_attempts: Optional[int] = None
    ) -> "WsClient":
        """Connect to the public WebSocket endpoint.

        Args:
            config: Client configuration
            max_reconnect_attempts: Maximum reconnection attempts

        Returns:
            WsClient instance
        """
        ...

    @staticmethod
    async def connect_private(
        config: Config, max_reconnect_attempts: Optional[int] = None
    ) -> "WsClient":
        """Connect to the private WebSocket endpoint.

        Args:
            config: Client configuration
            max_reconnect_attempts: Maximum reconnection attempts

        Returns:
            WsClient instance
        """
        ...

    async def subscribe_tickers(self, inst_id: str) -> None:
        """Subscribe to ticker channel."""
        ...

    async def subscribe_orderbook(self, inst_id: str) -> None:
        """Subscribe to order book channel."""
        ...

    async def subscribe_trades(self, inst_id: str) -> None:
        """Subscribe to trades channel."""
        ...

    async def subscribe_candles(
        self, inst_id: str, interval: str = "1m"
    ) -> None:
        """Subscribe to candlestick channel.

        Args:
            inst_id: Instrument ID
            interval: Candle interval (1m, 5m, 15m, 1H, 4H, 1D)
        """
        ...

    async def subscribe_account(self, ccy: Optional[str] = None) -> None:
        """Subscribe to account channel (private)."""
        ...

    async def subscribe_positions(
        self, inst_type: str, inst_id: Optional[str] = None
    ) -> None:
        """Subscribe to positions channel (private)."""
        ...

    async def subscribe_orders(
        self, inst_type: str, inst_id: Optional[str] = None
    ) -> None:
        """Subscribe to orders channel (private)."""
        ...

    async def recv(self) -> Optional[dict[str, Any]]:
        """Receive the next message from the WebSocket.

        Returns:
            dict with message data, or None if connection closed
        """
        ...

    async def is_connected(self) -> bool:
        """Check if the client is connected."""
        ...

    async def reconnect(self) -> None:
        """Manually trigger reconnection."""
        ...

    async def close(self) -> None:
        """Close the WebSocket connection."""
        ...

    async def subscription_count(self) -> int:
        """Get the number of active subscriptions."""
        ...

    def __aiter__(self) -> AsyncIterator[Optional[dict[str, Any]]]:
        """Async iterator support."""
        ...

    async def __anext__(self) -> Optional[dict[str, Any]]:
        """Async iterator next."""
        ...
