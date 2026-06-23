# 支付请求完整流程文档

> 本文档详细描述了 Hyperswitch-React-Demo-App 中，前端 React App、server.js (Express 代理服务器) 以及 Hyperswitch 后端 (Rust) 三者之间的完整交互流程。

---

## 目录

- [整体架构](#整体架构)
- [凭证/Token 类型说明](#凭证token-类型说明)
- [流程一：初始化 — 获取配置和 URL](#流程一初始化--获取配置和-url)
- [流程二：创建 Payment Intent](#流程二创建-payment-intent)
- [流程三：加载 SDK 并初始化](#流程三加载-sdk-并初始化)
- [流程四：用户提交支付 — confirmPayment](#流程四用户提交支付--confirmpayment)
- [流程五：查询支付状态 — retrievePaymentIntent](#流程五查询支付状态--retrievepaymentintent)
- [完整时序图](#完整时序图)
- [安全设计要点](#安全设计要点)
- [关键代码文件索引](#关键代码文件索引)

---

## 整体架构

```
┌──────────────────────────────────────────────────────────────────┐
│  浏览器 (前端 React App)               端口: 9060                  │
│                                                                  │
│  Payment.js                                                        │
│  ├─ fetch("http://localhost:5252/config")       → 获取公钥配置     │
│  ├─ fetch("http://localhost:5252/urls")         → 获取后端URL      │
│  ├─ fetch("http://localhost:5252/create-intent")→ 创建支付意图     │
│  ├─ load "{clientUrl}/HyperLoader.js"          ← 动态加载SDK脚本  │
│  └─ window.Hyper({ publishableKey })           ← 初始化SDK实例    │
│                                                                  │
│  CheckoutForm.js                                                   │
│  ├─ <PaymentElement />                          ← SDK渲染支付表单  │
│  └─ hyper.confirmPayment({ elements })          ← SDK确认支付      │
│     (SDK 直接与 Hyperswitch 后端加密通信)                          │
└──────────────────────┬───────────────────────────────────────────┘
                       │
          ┌────────────┼────────────────────┐
          ▼            ▼                    ▼
┌─────────────────┐  ┌────────────────────────────────────────┐
│  webpack proxy  │  │  server.js (Express)                    │
│  localhost:9060 │  │  localhost:5252                         │
│  → /payments    │  │                                         │
│  转发到 5252    │  │  GET /config    → 返回 publishableKey    │
└─────────────────┘  │  GET /urls      → 返回 serverUrl/clientUrl│
                     │  GET /create-intent                      │
                     │     → 用 Secret Key 调用 Hyperswitch API │
                     │     → 返回 clientSecret 给前端            │
                     │                                         │
                     │  持有: HYPERSWITCH_SECRET_KEY (敏感)     │
                     └──────────────┬──────────────────────────┘
                                    │
                                    ▼
                     ┌────────────────────────────────────────┐
                     │  Hyperswitch 后端 (Rust)                │
                     │  http://192.168.1.69:8080               │
                     │                                         │
                     │  校验 api-key (Secret Key)              │
                     │  创建 Payment Intent                    │
                     │  返回 client_secret                     │
                     │  处理 confirmPayment (含3DS验证)        │
                     │  路由到实际 PSP (Stripe/Adyen等)        │
                     └────────────────────────────────────────┘
```

---

## 凭证/Token 类型说明

| 凭证 | 持有者 | 敏感级别 | 作用 |
|------|--------|----------|------|
| `publishableKey` (公钥) | 前端 + server.js | 🔓 公开 | 标识商户身份，用于初始化SDK |
| `secretKey` / `api-key` (密钥) | server.js 仅服务端持有 | 🔴 最高 | 调用 Hyperswitch 后端的鉴权凭证 |
| `clientSecret` (客户端密钥) | 前端暂时持有 | 🟡 一次性 | 授权前端SDK操作本次支付的凭证 |
| `profileId` (配置文件ID) | 前端 + server.js | 🔓 公开 | 标识商户的支付配置 |
| 卡号/CVV (敏感支付信息) | SDK 内部加密传输 | 🔴 最高 | 前端业务代码无法接触明文 |

---

## 流程一：初始化 — 获取配置和 URL

### 触发时机

`Payment.js` 组件挂载时，`useEffect` 执行。

### 前端代码

**文件**: [src/Payment.js:26-58](src/Payment.js#L26-L58)

```javascript
const baseUrl = SELF_SERVER_URL || ENDPOINT;  // "http://localhost:5252"

// 并行请求两个接口
const { configData, urlsData } = await fetchConfigAndUrls(baseUrl);
```

**工具函数**: [src/utils.js:13-30](src/utils.js#L13-L30)

```javascript
export const fetchConfigAndUrls = async (baseUrl) => {
  const [configRes, urlsRes] = await Promise.all([
    fetch(`${baseUrl}/config`),
    fetch(`${baseUrl}/urls`),
  ]);

  const configData = await configRes.json();
  const urlsData = await urlsRes.json();

  return { configData, urlsData };
};
```

### server.js 处理

**文件**: [server.js:38-47](server.js#L38-L47)

```javascript
// GET /config — 从 .env 读取配置
app.get("/config", (req, res) => {
  res.send({
    publishableKey: process.env.HYPERSWITCH_PUBLISHABLE_KEY,
    profileId: process.env.PROFILE_ID,
  });
});

// GET /urls — 返回 SDK 和后端的地址
app.get("/urls", (req, res) => {
  res.send({
    serverUrl: SERVER_URL,   // "http://192.168.1.69:8080" (Hyperswitch 后端)
    clientUrl: CLIENT_URL,   // "http://192.168.1.69:9050" (SDK 托管地址)
  });
});
```

### 数据流

```
前端                                  server.js
  │                                       │
  │  GET /config                          │
  │  ─────────────────────────────────►   │
  │  ◄─── { publishableKey, profileId }   │
  │                                       │
  │  GET /urls                            │
  │  ─────────────────────────────────►   │
  │  ◄─── { serverUrl, clientUrl }        │
  │                                       │
```

### 前端拿到数据

| 数据 | 值示例 | 用途 |
|------|--------|------|
| `publishableKey` | `pk_dev_3a9b858b...` | 初始化 SDK |
| `profileId` | `pro_H5ttzUuRM5NWu...` | SDK 配置标识 |
| `serverUrl` | `http://192.168.1.69:8080` | SDK 直接与后端通信的地址 |
| `clientUrl` | `http://192.168.1.69:9050` | 加载 SDK 脚本的来源 |

---

## 流程二：创建 Payment Intent

### 触发时机

获取配置后立即调用。

### 前端代码

**文件**: [src/utils.js:3-21](src/utils.js#L3-L21)

```javascript
export const getPaymentIntentData = async ({
  baseUrl, isCypressTestMode, clientSecretQueryParam, setError,
}) => {
  if (isCypressTestMode) {
    return { clientSecret: clientSecretQueryParam };
  }

  const res = await fetch(`${baseUrl}/create-intent`);
  const data = await res.json();
  // 返回: { clientSecret: "cs_xxx", paymentId: "pid_xxx" }
  return data;
};
```

### server.js 处理

**文件**: [server.js:113-162](server.js#L113-L162)

```javascript
app.get("/create-intent", async (req, res) => {
  const paymentRequest = SDK_VERSION === "v1"
    ? createPaymentRequest()      // 传统格式
    : createPaymentRequestV2();   // v2 格式

  // 调用 Hyperswitch 后端 API
  const paymentIntent = await createPaymentIntent(paymentRequest);

  const response = {
    clientSecret: paymentIntent.client_secret,  // ← 关键凭证
  };
  if (SDK_VERSION === "v2") {
    response.paymentId = paymentIntent.id;
  }
  res.send(response);
});

async function createPaymentIntent(request) {
  const baseUrl = process.env.HYPERSWITCH_SERVER_URL;  // Hyperswitch 后端

  // V1: POST {serverUrl}/payments
  // 请求头携带 api-key (即 Secret Key)
  const apiResponse = await fetch(`${baseUrl}/payments`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "api-key": process.env.HYPERSWITCH_SECRET_KEY,  // ← Secret Key 只在服务端！
    },
    body: JSON.stringify(request),
  });

  const paymentIntent = await apiResponse.json();
  return paymentIntent;
}
```

### 发送给 Hyperswitch 后端的请求

**文件**: [server.js:54-109](server.js#L54-L109)

```json
POST http://192.168.1.69:8080/payments
Headers:
  api-key: dev_QxCtInewhfCI1QKvDzVW5dRs52HbOiua8aLsOzKElCjnLSv5Xsu8Dvhz4R0fOMD7
  Content-Type: application/json

Body:
{
  "currency": "USD",
  "amount": 2999,
  "confirm": false,
  "capture_method": "automatic",
  "authentication_type": "three_ds",
  "customer_id": "hyperswitch_sdk_demo_id",
  "email": "hyperswitch_sdk_demo_id@gmail.com",
  "shipping": {
    "address": {
      "line1": "1467",
      "line2": "Harrison Street",
      "city": "San Fransico",
      "state": "California",
      "zip": "94122",
      "country": "US",
      "first_name": "joseph",
      "last_name": "Doe"
    },
    "phone": {
      "number": "8056594427",
      "country_code": "+91"
    }
  },
  "billing": { /* 同上 */ },
  "metadata": {
    "udf1": "value1",
    "new_customer": "true",
    "login_date": "2019-09-10T10:11:12Z"
  }
}
```

### 数据流

```
前端                              server.js                      Hyperswitch 后端
  │                                  │                                │
  │  GET /create-intent               │                                │
  │  ───────────────────────────►     │                                │
  │                                  │  POST /payments                 │
  │                                  │  Header: api-key=SecretKey      │
  │                                  │  Body: {amount, confirm:false}  │
  │                                  │  ───────────────────────────►   │
  │                                  │                                │
  │                                  │  ← 校验 api-key 是否有效        │
  │                                  │  ← 创建 Payment Intent 记录     │
  │                                  │  ← 生成 client_secret           │
  │                                  │                                │
  │                                  │  ◄─── { client_secret, id }    │
  │  ◄─── { clientSecret }           │                                │
  │                                  │                                │
```

**关键点**: `confirm: false` 表示此时只是创建支付意图，并未实际扣款。实际扣款在用户提交支付表单后由 SDK 完成。

---

## 流程三：加载 SDK 并初始化

### 触发时机

拿到 `clientSecret` 后。

### 前端代码

**文件**: [src/utils.js:32-55](src/utils.js#L32-L55)

```javascript
export const loadHyperScript = ({
  clientUrl,       // SDK 托管地址
  publishableKey,  // 公钥
  customBackendUrl,// Hyperswitch 后端地址
  profileId,
  isScriptLoaded,
  setIsScriptLoaded,
}) => {
  return new Promise((resolve, reject) => {
    if (isScriptLoaded) return resolve(window.Hyper);

    // 1. 动态创建 <script> 标签加载 SDK
    const script = document.createElement("script");
    script.src = `${clientUrl}/HyperLoader.js`;
    script.async = true;

    script.onload = () => {
      setIsScriptLoaded(true);

      // 2. SDK 加载完成后初始化 Hyper 实例
      //    publishableKey 用于鉴权商户身份
      //    customBackendUrl 告诉 SDK 后端 API 地址
      resolve(
        window.Hyper(
          { publishableKey, profileId },    // 认证信息
          { customBackendUrl }              // 后端地址
        )
      );
    };

    script.onerror = () => {
      reject("Failed to load HyperLoader.js");
    };

    document.head.appendChild(script);
  });
};
```

### 渲染 Provider

**文件**: [src/Payment.js:91-96](src/Payment.js#L91-L96)

```javascript
// 将 clientSecret 传入 options
let selectedOptions;
if (SDK_VERSION === "v1") {
  selectedOptions = hyperOptionsV1(clientSecret, localeQueryParam, themeQueryParam);
} else {
  selectedOptions = hyperOptionsV2(clientSecret, paymentId, localeQueryParam, themeQueryParam);
}

// Provider 将 Hyper 实例注入整个组件树
<HyperElements hyper={hyperPromise} options={selectedOptions}>
  <CheckoutForm />
</HyperElements>
```

**文件**: [src/utils.js:117-136](src/utils.js#L117-L136)

```javascript
export const hyperOptionsV1 = (clientSecret, locale, theme) => ({
  clientSecret,
  ...(locale && { locale }),
  appearance: {
    labels: "floating",
    ...(theme && { theme }),
  },
});

export const hyperOptionsV2 = (clientSecret, paymentId, locale, theme) => ({
  clientSecret,
  paymentId,
  ...(locale && { locale }),
  appearance: {
    labels: "floating",
    ...(theme && { theme }),
  },
});
```

### 数据流

```
前端                                                         SDK 服务器 (hyperswitch-web)
  │                                                                   │
  │  动态创建 <script> 标签                                              │
  │  script.src = "{clientUrl}/HyperLoader.js"                          │
  │  ─────────────────────────────────────────────────────────────►    │
  │                                                                   │
  │  ◄─── 返回 SDK JavaScript 脚本                                     │
  │                                                                   │
  │  执行 window.Hyper({                                               │
  │    publishableKey: "pk_dev_xxx",       // 商户公钥                  │
  │    profileId: "pro_H5ttzUuRM5..."      // 配置文件 ID              │
  │  }, {                                                              │
  │    customBackendUrl: "http://..."       // Hyperswitch 后端地址     │
  │  })                                                                │
  │                                                                   │
  │  SDK 内部:                                                         │
  │  - 用 publishableKey 创建加密通道                                   │
  │  - 用 clientSecret (从 options 传入) 绑定本次支付意图               │
  │  - 准备好渲染 PaymentElement                                       │
  │                                                                   │
```

---

## 流程四：用户提交支付 — confirmPayment

### 触发时机

用户在 `<PaymentElement>` 中填写卡号等信息，点击 "Pay now" 按钮。

### 前端代码

**文件**: [src/CheckoutForm.js:34-56](src/CheckoutForm.js#L34-L56)

```javascript
const handleSubmit = async (e) => {
  e.preventDefault();
  if (!hyper || !elements || isProcessing) return;

  setIsProcessing(true);

  // 调用 SDK 的 confirmPayment
  const { error, status } = await hyper.confirmPayment({
    elements,  // PaymentElement 实例（含用户填写的卡号等）
    confirmParams: {
      return_url: window.location.origin,  // 3DS 跳转回来后的地址
    },
  });

  if (error) {
    setMessage(error.message);
  }
  if (status) {
    handlePaymentStatus(status, setMessage, setIsSuccess);
  }
};
```

### 状态处理

**文件**: [src/utils.js:66-85](src/utils.js#L66-L85)

```javascript
export const handlePaymentStatus = (status, setMessage, setIsSuccess) => {
  const statusMessages = {
    succeeded: "Payment successful.",
    processing: "Your payment is processing.",
    requires_payment_method: "Your payment was not successful. Please try again.",
    requires_capture: "Payment is authorized and requires manual capture.",
    requires_customer_action: "Customer needs to take further action.",
    failed: "Payment failed. Please check your payment method.",
  };

  const messageToSet = statusMessages[status] || `Unexpected payment status: ${status}`;
  setMessage(messageToSet);
  setIsSuccess(status === "succeeded");
};
```

### SDK 内部流程

```
hyper.confirmPayment() 内部流程:

┌──────────────────────────────────────────────────────────────────┐
│                                                                    │
│  ① 从 PaymentElement 收集用户输入的支付信息                          │
│     - 卡号 (Card Number)                                            │
│     - 有效期 (Expiry Date)                                          │
│     - CVV (安全码)                                                  │
│     - 持卡人姓名                                                    │
│                                                                    │
│  ② SDK 将卡号等敏感信息加密                                          │
│     (前端 JavaScript 代码无法截获明文)                               │
│                                                                    │
│  ③ SDK 调用 Hyperswitch 后端:                                       │
│     POST {customBackendUrl}/payments/{payment_id}/confirm           │
│     Headers:                                                        │
│       Authorization: Bearer {clientSecret}                          │
│       X-Profile-Id: {profileId}                                     │
│     Body:                                                           │
│       {                                                             │
│         "payment_method": "card",                                   │
│         "payment_method_data": {                                    │
│           "card": { encrypted_card_data }        ← SDK 加密         │
│         },                                                          │
│         "browser_info": { ... }                  ← 浏览器指纹      │
│       }                                                             │
│                                                                    │
│  ④ Hyperswitch 后端:                                               │
│     - 用 client_secret 验证此次请求的权限                            │
│     - 解密卡信息                                                     │
│     - 根据路由规则选择实际 PSP (Stripe/Adyen 等)                     │
│     - 判断是否需要 3DS 验证                                          │
│                                                                    │
│  ⑤ 需要 3DS 验证:                                                  │
│     ┌────────────────────────────────────────────┐                  │
│     │  Hyperswitch 返回 3DS 认证 URL               │                  │
│     │  SDK 自动弹出 3DS 安全页面                   │                  │
│     │  用户在银行页面输入验证码                    │                  │
│     │  验证完成后跳转回 return_url                │                  │
│     └────────────────────────────────────────────┘                  │
│                                                                    │
│  ⑥ 不需要 3DS 验证:                                                │
│     ┌────────────────────────────────────────────┐                  │
│     │  Hyperswitch 直接转发到 PSP 扣款             │                  │
│     │  PSP 返回扣款结果                             │                  │
│     └────────────────────────────────────────────┘                  │
│                                                                    │
│  ⑦ 返回 { error, status } 给前端                                    │
│                                                                    │
└──────────────────────────────────────────────────────────────────┘
```

### 数据流

```
前端 (SDK)                                   Hyperswitch 后端
  │                                              │
  │  POST /payments/{id}/confirm                 │
  │  Authorization: Bearer {clientSecret}        │
  │  X-Profile-Id: pro_xxx                       │
  │  Body: {                                     │
  │    payment_method: "card",                   │
  │    payment_method_data: {                    │
  │      card: { encrypted_data }                │
  │    },                                        │
  │    browser_info: { ... }                     │
  │  }                                           │
  │  ───────────────────────────────────────►    │
  │                                              │
  │  ← 校验 client_secret ✓                      │
  │  ← 解密卡信息 ✓                              │
  │  ← 路由到 Stripe / Adyen                     │
  │  ← 执行扣款                                   │
  │                                              │
  │  ◄─── { status: "succeeded" }                │
  │  ◄─── 或 { error: { message } }              │
  │                                              │
```

---

## 流程五：查询支付状态 — retrievePaymentIntent

### 触发时机

1. 页面加载时（从 3DS 跳转回来后恢复状态）
2. 需要刷新当前支付状态时

### 前端代码

**文件**: [src/CheckoutForm.js:86-101](src/CheckoutForm.js#L86-L101)

```javascript
useEffect(() => {
  if (!hyper || !clientSecret) return;

  const fetchPaymentIntent = async () => {
    try {
      // SDK 查询 Payment Intent 状态
      const { paymentIntent } = await hyper.retrievePaymentIntent(clientSecret);

      // 根据状态更新 UI
      if (paymentIntent?.status) {
        handlePaymentStatus(paymentIntent.status, setMessage, setIsSuccess);
      }
    } catch (err) {
      console.error("Error retrieving payment intent:", err);
      setMessage("Unable to retrieve payment details.");
    }
  };

  fetchPaymentIntent();
}, [hyper, clientSecret]);
```

### 数据流

```
前端 (SDK)                                   Hyperswitch 后端
  │                                              │
  │  GET /payments/{payment_id}                  │
  │  Authorization: Bearer {clientSecret}        │
  │  ───────────────────────────────────────►    │
  │                                              │
  │  ◄─── {                                      │
  │    id: "pid_xxx",                            │
  │    status: "succeeded",                      │
  │    client_secret: "cs_xxx",                  │
  │    amount: 2999,                             │
  │    currency: "USD",                          │
  │    ...                                       │
  │  }                                           │
  │                                              │
  触发 handlePaymentStatus("succeeded")
  → 显示成功页面 (Completion.js)
```

---

## 完整时序图

```
前端 (9060)               server.js (5252)          Hyperswitch 后端 (8080)        SDK CDN (9050)
    │                          │                          │                          │
    │ ① GET /config            │                          │                          │
    │ ─────────────────────►   │                          │                          │
    │ ◄── {publishableKey} ◄───                          │                          │
    │                          │                          │                          │
    │ ② GET /urls              │                          │                          │
    │ ─────────────────────►   │                          │                          │
    │ ◄── {serverUrl,clientUrl}                          │                          │
    │                          │                          │                          │
    │ ③ GET /create-intent     │                          │                          │
    │ ─────────────────────►   │                          │                          │
    │                          │ ④ POST /payments         │                          │
    │                          │    api-key=SecretKey     │                          │
    │                          │    {amount,confirm:false}│                          │
    │                          │ ──────────────────────►  │                          │
    │                          │                          │ ← 创建 Payment Intent    │
    │                          │ ◄── {client_secret} ◄────│                          │
    │ ◄── {clientSecret} ◄─────                          │                          │
    │                          │                          │                          │
    │ ⑤ 加载 HyperLoader.js    │                          │                          │
    │ ──────────────────────────────────────────────────────────────────────────►   │
    │ ◄── SDK 脚本 ◄─────────────────────────────────────────────────────────────── │
    │                          │                          │                          │
    │ ⑥ 初始化 Hyper 实例      │                          │                          │
    │    window.Hyper({pk}, {customBackendUrl})            │                          │
    │                          │                          │                          │
    │ ⑦ 渲染 <PaymentElement>  │                          │                          │
    │    用户填写卡号、CVV      │                          │                          │
    │    点击 "Pay now"        │                          │                          │
    │                          │                          │                          │
    │ ⑧ hyper.confirmPayment() │                          │                          │
    │    POST /payments/{id}/confirm                      │                          │
    │    Authorization: Bearer {clientSecret}              │                          │
    │    Body: {加密卡信息, browser_info}                   │                          │
    │ ──────────────────────────────────────────────────►  │                          │
    │                          │                          │ ← 校验 client_secret     │
    │                          │                          │ ← 解密卡信息             │
    │                          │                          │ ← 路由到 PSP 扣款        │
    │                          │                          │                          │
    │ ◄── {status:"succeeded"} ◄───────────────────────────                          │
    │                          │                          │                          │
    │ ⑨ 显示成功页面           │                          │                          │
    │    <Completion />        │                          │                          │
    │                          │                          │                          │
```

---

## 安全设计要点

### 1. 分层密钥体系

```
敏感级别        凭证                   存放位置
────────────────────────────────────────────────────
🔴 最高    Secret Key (api-key)      server.js 环境变量
🟡 中等    clientSecret             前端内存变量（有时效性）
🟢 低      publishableKey           前端公开代码
```

### 2. 关键安全原则

| 原则 | 说明 |
|------|------|
| **Secret Key 不上前端** | `HYPERSWITCH_SECRET_KEY` 仅 server.js 使用，前端无法访问 |
| **clientSecret 有时效** | 一次性凭证，过期后无效，即使泄露也只能操作本次支付 |
| **卡号加密传输** | SDK 内部将卡号加密后发送到 Hyperswitch 后端，前端代码接触不到明文 |
| **PCI 合规** | 敏感支付信息不经过商户服务器，由 SDK 直接发送到 Hyperswitch 后端 |

### 3. 攻击面分析

| 攻击场景 | 是否可能 | 防御措施 |
|----------|----------|----------|
| 前端窃取 Secret Key | ❌ 不可能 | Secret Key 在服务端，前端无法访问 |
| 前端窃取卡号 | ❌ 不可能 | SDK 内部加密传输，业务代码拿不到 |
| 伪造 clientSecret | ❌ 不可能 | 由 Hyperswitch 后端签名，无法伪造 |
| 重放 clientSecret | ⚠️ 有限 | 一次性使用，过期失效 |

---

## 关键代码文件索引

| 文件 | 作用 |
|------|------|
| [src/Payment.js](src/Payment.js) | 支付入口组件，负责初始化、加载SDK、渲染 Provider |
| [src/CheckoutForm.js](src/CheckoutForm.js) | 支付表单组件，处理用户交互和 confirmPayment |
| [src/utils.js](src/utils.js) | 工具函数：加载SDK、配置选项、状态处理 |
| [src/Cart.js](src/Cart.js) | 购物车展示组件 |
| [src/Completion.js](src/Completion.js) | 支付成功页面 |
| [server.js](server.js) | Express 代理服务器，处理 /config、/urls、/create-intent |
| [webpack.common.js](webpack.common.js) | Webpack 编译配置，注入全局常量 |
| [.env](.env) | 环境变量配置 |