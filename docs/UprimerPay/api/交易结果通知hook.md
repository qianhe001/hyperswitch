## 交易结果异步通知

*最近更新时间：2026.05.07*

### 接口说明

*   交易结果 WEBHOOK 事件通知
    
*   平台在交易状态更新后，向商户下单时传入的 `notificationUrl` 发起异步通知
    
*   本次新增字段：`paymentMethod`、`paymentAcceptance`
    

### 请求头信息

| 参数  | 中文名 | 定义  | 长度  | 必填  | 说明  |
| --- | --- | --- | --- | --- | --- |
| X-Signature | 签名  | String | 32  | O   | 签名值，生成规则：`MD5(请求体原始JSON字符串 + 商户secretKey)` |

### 请求参数

| 参数  | 中文名 | 定义  | 长度  | 必填  | 说明  |
| --- | --- | --- | --- | --- | --- |
| requestId | 请求流水号 | String | 64  | M   | 商户请求流水号 |
| appId | 商户应用ID | String | 32  | M   | 商户应用ID |
| merchantOrderId | 商户订单号 | String | 64  | M   | 商户订单号 |
| amount | 交易金额 | Long | <p></p> | M   | 交易金额 |
| currency | 交易币种 | String | 3   | M   | 交易币种 |
| capturedAmount | 捕获金额 | Long | <p></p> | O   | 已捕获金额 |
| id  | 汇付交易号 | String | 32  | M   | 平台交易ID |
| transactionType | 交易类型 | String | 32  | M   | 如：`SALE`\-消费、`REFUND`\-退款、`AUTHORIZE`\-预授权、`CAPTURE`\-预授权完成 |
| status | 交易状态 | String | 32  | M   | `SUCCEED`\-成功、`FAILED`\-失败、`PENDING`\-处理中、`RETRY_PENDING`\-等待重试 |
| paymentMethod | 支付方式 | String | 32  | M   | 本次新增。如：`CARD`\-卡支付、`WECHATPAY`\-微信支付、`ALIPAYCN`\-支付宝中国钱包等 |
| paymentAcceptance | 支付受理方式 | String | 32  | O   | 本次新增。仅 `paymentMethod=CARD` 时返回，取值为卡组编码，如：`001`\-Visa、`002`\-Mastercard |
| merchantMemo | 商户备注 | String | 128 | O   | 商户下单时传入的备注 |
| errorCode | 错误码 | String | 32  | O   | 交易失败时返回 |
| errorMsg | 错误描述 | String | 256 | O   | 交易失败时返回；交易成功时为空 |
| createTime | 创建时间 | String | 32  | O   | 格式：`yyyy-MM-dd'T'HH:mm:ssZ` |
| updateTime | 更新时间 | String | 32  | O   | 格式：`yyyy-MM-dd'T'HH:mm:ssZ` |
| rawCode | 原始返回码 | String | 32  | O   | 卡支付且非处理中状态时可能返回 |
| rawRefusalDescription | 原始拒绝原因描述 | String | 256 | O   | 卡支付失败时可能返回 |
| authCode | 授权码 | String | 32  | O   | 卡支付授权码 |
| shortCardNo | 短卡号 | String | 32  | O   | 收银台卡支付场景可能返回 |
| countryCode | 国家/地区码 | String | 2   | O   | 卡账单国家/地区码 |
| eci | ECI | String | 32  | O   | 3DS ECI |
| threeDS | 3DS状态 | String | 32  | O   | 3DS 状态 |
| subscriptionRequestId | 订阅请求ID | String | 64  | O   | 订阅支付场景返回 |
| subscriptionId | 订阅ID | String | 32  | O   | 订阅支付场景返回 |
| periodNum | 订阅期数 | Integer | <p></p> | O   | 订阅支付场景返回 |
| cancellationReason | 取消原因 | String | 256 | O   | 取消/撤销交易时可能返回 |
| originalId | 原交易号 | String | 32  | O   | 退款/撤销等关联原交易时返回 |
| refundReason | 退款原因 | String | 256 | O   | 退款交易时可能返回 |
| retryCode | 重试建议码 | String | 32  | O   | `RETRY_PENDING` 场景可能返回 |
| retryMsg | 重试建议描述 | String | 256 | O   | `RETRY_PENDING` 场景可能返回 |

### 事件通知示例

```json
{
    "requestId": "20220313000001",
    "appId": "600001",
    "merchantOrderId": "202203130000abc",
    "amount": 100000,
    "currency": "USD",
    "capturedAmount": 100000,
    "id": "1020260507001001",
    "transactionType": "SALE",
    "status": "SUCCEED",
    "paymentMethod": "CARD",
    "paymentAcceptance": "001",
    "merchantMemo": "112233",
    "errorCode": null,
    "errorMsg": null,
    "createTime": "2026-05-07T10:30:00+0800",
    "updateTime": "2026-05-07T10:30:05+0800",
    "rawCode": "00",
    "rawRefusalDescription": "Approval and completed successfully Accepted and processed",
    "authCode": "123456",
    "shortCardNo": "400000******1000",
    "countryCode": "US",
    "eci": "05",
    "threeDS": "NONEED"
}
```