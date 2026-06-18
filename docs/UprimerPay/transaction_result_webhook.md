# UprimerPay 交易结果异步通知

最近更新时间：2026-05-07

## 接口说明

UprimerPay 在交易状态更新后，会向商户下单或退款请求中传入的 `notificationUrl` 发起异步通知。Hyperswitch 接收到该通知后，会根据通知中的交易类型和状态更新对应的支付或退款状态。

本次通知新增字段：

- `paymentMethod`
- `paymentAcceptance`

## 请求头

| 参数 | 中文名 | 类型 | 长度 | 必填 | 说明 |
| --- | --- | --- | --- | --- | --- |
| `X-Signature` | 签名 | String | 32 | 是 | 签名值，生成规则：`MD5(请求体原始 JSON 字符串 + 商户 secretKey)` |

## 签名验证

收到 webhook 后，需要先读取原始请求体，不要重新格式化或重新序列化 JSON，然后用商户 `secretKey` 计算 MD5：

```text
currentSign = MD5(raw_request_body + secretKey)
```

再与请求头中的 `X-Signature` 比较。Hyperswitch 的 UprimerPay connector 已按 SDK 示例 `ReceiveHuifuNotifyController` 的方式处理：

1. 从请求头读取 `X-Signature`
2. 使用原始请求体拼接 connector 配置中的 `Secret Key`
3. 计算 MD5
4. 与 `X-Signature` 比较
5. 验签通过后才继续处理业务状态

## 请求参数

| 参数 | 中文名 | 类型 | 长度 | 必填 | 说明 |
| --- | --- | --- | --- | --- | --- |
| `requestId` | 请求流水号 | String | 64 | 是 | 商户请求流水号 |
| `appId` | 商户应用 ID | String | 32 | 是 | 商户应用 ID |
| `merchantOrderId` | 商户订单号 | String | 64 | 是 | 商户订单号 |
| `amount` | 交易金额 | Long | - | 是 | 交易金额，最小货币单位 |
| `currency` | 交易币种 | String | 3 | 是 | ISO 货币代码，如 `USD` |
| `capturedAmount` | 捕获金额 | Long | - | 否 | 已捕获金额 |
| `id` | 汇付交易号 | String | 32 | 是 | 平台交易 ID |
| `transactionType` | 交易类型 | String | 32 | 是 | `SALE`、`REFUND`、`AUTHORIZE`、`CAPTURE` |
| `status` | 交易状态 | String | 32 | 是 | `SUCCEED`、`FAILED`、`PENDING`、`RETRY_PENDING` |
| `paymentMethod` | 支付方式 | String | 32 | 是 | 如 `CARD`、`WECHATPAY`、`ALIPAYCN` |
| `paymentAcceptance` | 支付受理方式 | String | 32 | 否 | 仅 `paymentMethod=CARD` 时返回，取值为卡组编码，如 `001` Visa、`002` Mastercard |
| `merchantMemo` | 商户备注 | String | 128 | 否 | 商户下单时传入的备注 |
| `errorCode` | 错误码 | String | 32 | 否 | 交易失败时返回 |
| `errorMsg` | 错误描述 | String | 256 | 否 | 交易失败时返回；交易成功时为空 |
| `createTime` | 创建时间 | String | 32 | 否 | 格式：`yyyy-MM-dd'T'HH:mm:ssZ` |
| `updateTime` | 更新时间 | String | 32 | 否 | 格式：`yyyy-MM-dd'T'HH:mm:ssZ` |
| `rawCode` | 原始返回码 | String | 32 | 否 | 卡支付且非处理中状态时可能返回 |
| `rawRefusalDescription` | 原始拒绝原因描述 | String | 256 | 否 | 卡支付失败时可能返回 |
| `authCode` | 授权码 | String | 32 | 否 | 卡支付授权码 |
| `shortCardNo` | 短卡号 | String | 32 | 否 | 收银台卡支付场景可能返回 |
| `countryCode` | 国家/地区码 | String | 2 | 否 | 卡账单国家/地区码 |
| `eci` | ECI | String | 32 | 否 | 3DS ECI |
| `threeDS` | 3DS 状态 | String | 32 | 否 | 3DS 状态 |
| `subscriptionRequestId` | 订阅请求 ID | String | 64 | 否 | 订阅支付场景返回 |
| `subscriptionId` | 订阅 ID | String | 32 | 否 | 订阅支付场景返回 |
| `periodNum` | 订阅期数 | Integer | - | 否 | 订阅支付场景返回 |
| `cancellationReason` | 取消原因 | String | 256 | 否 | 取消/撤销交易时可能返回 |
| `originalId` | 原交易号 | String | 32 | 否 | 退款/撤销等关联原交易时返回 |
| `refundReason` | 退款原因 | String | 256 | 否 | 退款交易时可能返回 |
| `retryCode` | 重试建议码 | String | 32 | 否 | `RETRY_PENDING` 场景可能返回 |
| `retryMsg` | 重试建议描述 | String | 256 | 否 | `RETRY_PENDING` 场景可能返回 |

## Hyperswitch 状态映射

## Hyperswitch 关联规则

- 支付类通知（`SALE`、`AUTHORIZE`、`CAPTURE`）优先使用 `originalId` 作为原交易号关联支付；没有 `originalId` 时使用 `id` 关联支付。
- 退款类通知（`REFUND`）使用 `merchantOrderId` 关联 Hyperswitch refund id。当前 UprimerPay 退款请求中，Hyperswitch 会将 `merchantOrderId` 和 `requestId` 都设置为退款 ID。
- 回调验签必须先通过；签名不匹配时 Hyperswitch 会拒绝处理该通知。

| UprimerPay `transactionType` | UprimerPay `status` | Hyperswitch webhook event |
| --- | --- | --- |
| `SALE` | `SUCCEED` | `PaymentIntentSuccess` |
| `SALE` | `FAILED` | `PaymentIntentFailure` |
| `SALE` | `PENDING` / `RETRY_PENDING` | `PaymentIntentProcessing` |
| `AUTHORIZE` | `SUCCEED` | `PaymentIntentAuthorizationSuccess` |
| `AUTHORIZE` | `FAILED` | `PaymentIntentAuthorizationFailure` |
| `AUTHORIZE` | `PENDING` / `RETRY_PENDING` | `PaymentIntentProcessing` |
| `CAPTURE` | `SUCCEED` | `PaymentIntentCaptureSuccess` |
| `CAPTURE` | `FAILED` | `PaymentIntentCaptureFailure` |
| `CAPTURE` | `PENDING` / `RETRY_PENDING` | `PaymentIntentProcessing` |
| `REFUND` | `SUCCEED` | `RefundSuccess` |
| `REFUND` | `FAILED` | `RefundFailure` |
| `REFUND` | `PENDING` / `RETRY_PENDING` | 暂不更新，返回 `EventNotSupported` |

## 通知示例

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
