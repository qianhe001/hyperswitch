### UMD 方式

```html
<div id="app"></div>
<script src="https://checkout.uprimer.com/icc-pay/v2/index.js""></script>
<script>
  const pay = new UPrimerPay({
    el: "#app",
    locale: "en",
    apiUrl: "https://acquire.uprimer.com",
    theme: {
      primaryColor: "#484dd4",
    },
    onSuccess: (result) => {
      console.log("操作成功：", result);
      alert("");
    },
    onError: (error) => {
      console.error("发生错误：", error);
    },
  });
  // 挂载应用
  pay.mount();

  // 切换浏览器语言
  pay.updateConfig({
    locale: "cn",
  });

  // 获取当前配置
  const currentConfig = pay.getConfig();

  // 卸载应用
  pay.unmount();
</script>
```

#### UPrimerPayConfig

| 参数   | 类型                  | 必填 | 默认值                          | 说明       |
| ------ | --------------------- | ---- | ------------------------------- | ---------- |
| el     | string \| HTMLElement | 是   | -                               | 挂载点     |
| locale | string                | 否   | 'en'                            | 国际化语言 |
| apiUrl | string                | 否   | '<https://acquire.uprimer.com>' | API 地址   |
| theme  | ThemeConfig           | 否   | -                               | 主题配置   |

#### ThemeConfig 主题配置

| 参数          | 类型   | 必填 | 默认值  | 说明               |
| ------------- | ------ | ---- | ------- | ------------------ |
| primaryColor  | string | 否   | #484dd4 | 主题色             |
| successColor  | string | 否   | #52C41A | 成功               |
| warningColor  | string | 否   | #EF8E34 | 警告               |
| dangerColor   | string | 否   | #E34D59 | 失败               |
| textColor     | string | 否   | #000000 | 文字颜色           |
| borderRadius  | number | 否   | 8       | 圆角               |
| fontSize      | number | 否   | 14      | 字号               |
| componentSize | number | 否   | 40      | 输入框 \| 按钮高度 |
