package request

import (
	"AcquireSdk/enum"
	"fmt"
	"log"
	"math/rand"
	"time"
)

/**
 * 外卡收单收银台
 * 文档地址：https://uprimer.net/open/docs/api#/api_jk_syt
 */
func (request *Request) CreateCheckOut() {
	date := time.Now().UTC()
	now := date.Format("2006-01-02T15:04:05Z")
	data := map[string]interface{}{
		"amount":          19900,
		"currency":        "HKD",
		"appId":           "10000",
		"merchantOrderId": fmt.Sprintf("%s%d", date.Format("20060102150405"), rand.Intn(100000)),
		"requestId":       fmt.Sprintf("%s%d", date.Format("20060102150405"), rand.Intn(100000)),
		"validityPeriod":  500,
		"orderTime":       now,
		"paymentMethod": map[string]string{
			"methodType": "CARD",
		},
		"shipping": map[string]string{
			"city":        "Toronto",
			"countryCode": "CA",
			"firstName":   "XINGGUO",
			"lastName":    "XU",
			"phoneNumber": "+8618672362337",
			"postCode":    "M5G 1V7",
			"state":       "ON",
			"street":      "160-500 University",
			"street2":     "",
		},
		"products": []interface{}{
			map[string]interface{}{
				"code":      "4098755",
				"name":      "High-Collar Boxy Camel Hair Blend Sweater,MUMUXI LED Fairy Lights Battery Operated String Lights [12 Pack] 7.2ft 20 Battery Powered LED Lights | Mini Lights, Centerpiece Table Decorations, Wedding Party Bedroom Mason Jar Christmas, Warm White",
				"quantity":  1,
				"sku":       "4098755 black",
				"unitPrice": 100,
			},
		},
		"cancelUrl":       "https://hfgj.testpnr.com/crossDemo/webHook.do",
		"notificationUrl": "https://hfgj.testpnr.com/crossDemo/webHook.do",
		"successUrl":      "https://hfgj.testpnr.com/crossDemo/webHook.do",
		"failureUrl":      "https://hfgj.testpnr.com/crossDemo/webHook.do",
	}

	url := request.Host + enum.URI_CHECKOUT_CREATE
	log.Println("请求地址url= " + url)
	resp, err := request.HttpPost(url, data)
	if err == nil {
		code := resp.(map[string]interface{})["code"].(string)
		if code == "00000000" {
			url := resp.(map[string]interface{})["data"].(map[string]interface{})["nextAction"].(map[string]interface{})["url"].(string)
			fmt.Println("请在浏览器访问该地址：" + url)
		}

	}
}
