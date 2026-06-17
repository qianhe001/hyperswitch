<?php
namespace AcquireSdk\enum;

class UriEnum
{

    /**
     * 获取token
     */
    public static $URI_TOKEN_AUTH = "/authorize";
 
    /**
     * 外卡收单收银台
     */
    public static $URI_CHECKOUT_CREATE = "/api/acquire/checkout/create";

    /**
     * 外卡直连交易
     */
    public static $URI_PAYMENT_CREATE = "/api/acquire/payment/create";

    /**
     * 交易退款
     */
    public static $URI_PAYMENT_REFUND = "/api/acquire/payment/{originalId}/refund";

    /**
     * 交易查询
     */
    public static $URI_TRANSACTION_QUEURY = "/api/acquire/payment/{originalId}/get";
}