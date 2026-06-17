package enum

const (
	/**
	 * debug模式/生产模式
	 */
	DEBUG = true
	/**
	 * 获取token
	 */
	URI_TOKEN_AUTH = "/authorize"

	/**
	 * 外卡收单收银台
	 */
	URI_CHECKOUT_CREATE = "/api/acquire/checkout/create"

	/**
	 * 外卡直连交易
	 */
	URI_PAYMENT_CREATE = "/api/acquire/payment/create"

	/**
	 * 交易退款
	 */
	URI_PAYMENT_REFUND = "/api/acquire/payment/{originalId}/refund"

	/**
	 * 交易查询
	 */
	URI_TRANSACTION_QUEURY = "/api/acquire/payment/{originalId}/get"
)
