package com.acquire.sdk.enums;

public enum UriEnum {
    
    URI_TOKEN_AUTH("/authorize","获取token"),
    URI_CHECKOUT_CREATE("/api/acquire/checkout/create","外卡收单收银台"),
    URI_PAYMENT_CREATE("/api/acquire/payment/create","外卡直连交易"),
    URI_PAYMENT_REFUND("/api/acquire/payment/{originalId}/refund","交易退款"),
    URI_TRANSACTION_QUEURY("/api/acquire/payment/{originalId}/get","交易查询");

    private final String uri;
    private final String name;

    private UriEnum(String uri, String name) {
        this.uri = uri;
        this.name = name;
    }

    public String getUri() {
        return this.uri;
    }

    public String getName() {
        return this.name;
    } 
    
}
