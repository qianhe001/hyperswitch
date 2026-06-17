package request

import (
	"AcquireSdk/enum"
	"fmt"
	"log"
	"math/rand"
	"time"
)

/**
 * 外卡收单直连 No 3DS
 * 文档地址：https://uprimer.net/open/docs/api#/api_jk_zl
 */
func (request *Request) CreatePaymentNo3DRequest() {
	date := time.Now().UTC()
	now := date.Format("2006-01-02T15:04:05Z")

	data := map[string]interface{}{
		"amount":          290000,
		"appId":           "10000",
		"currency":        "USD",
		"descriptor":      "付款",
		"merchantOrderId": fmt.Sprintf("%s%d", time.Now().Format("20060102150405"), rand.Intn(100000)),
		"requestId":       fmt.Sprintf("%s%d", time.Now().Format("20060102150405"), rand.Intn(100000)),
		"cancelUrl":       "https://hfgj.testpnr.com/crossDemo/webHook.do",
		"successUrl":      "https://hfgj.testpnr.com/crossDemo/webHook.do",
		"failureUrl":      "https://hfgj.testpnr.com/crossDemo/webHook.do",
		"notificationUrl": "https://hfgj.testpnr.com/crossDemo/webHook.do",
		"orderTime":       now,
		"paymentMethod": map[string]interface{}{
			"methodType": "CARD",
			"card": map[string]interface{}{
				"cvv":         "133",
				"expiryMonth": "10",
				"expiryYear":  "26",
				"firstName":   "xingguo",
				"lastName":    "xu",
				"number":      "5200000000001096",
				"billing": map[string]interface{}{
					"firstName":   "xingguo",
					"lastName":    "xu",
					"dateOfBirth": "",
					"phoneNumber": "11144442121",
					"email":       "xu.xg@qq.com",
					"countryCode": "GB",
					"state":       "UK",
					"city":        "Toronto",
					"street":      "160-500 University",
					"postCode":    "12345-1233",
				},
			},
		},
		"products": []map[string]interface{}{
			{
				"code":        "101110",
				"name":        "iphone",
				"quantity":    1,
				"sku":         "black",
				"unitPrice":   6000,
				"totalAmount": 6000,
			},
		},
		"shipping": map[string]interface{}{
			"company":     "shipping company",
			"firstName":   "lucy",
			"lastName":    "king",
			"phoneNumber": "13388888888",
			"countryCode": "GB",
			"state":       "UK",
			"city":        "Toronto",
			"street":      "160-500 University",
			"street2":     "st2",
			"postCode":    "Box1026",
		},
		"deviceData": map[string]interface{}{
			"acceptHeader":             "text/html",
			"browserJavaEnabled":       "true",
			"browserJavascriptEnabled": "true",
			"browserUserAgent":         "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.183",
			"challengeWindow":          "5",
			"language":                 "zh-CN",
			"screenColorDepth":         "48",
			"screenHeight":             "1200",
			"screenWidth":              "1600",
			"timezone":                 "60",
		},
	}
	url := request.Host + enum.URI_PAYMENT_CREATE
	log.Println("请求地址url= " + url)
	resp, err := request.HttpPost(url, data)
	if err == nil {
		code := resp.(map[string]interface{})["code"].(string)
		if code == "00000000" {
			fmt.Println("请求返回状态码code =" + code)
			fmt.Println("请求返回数据 =")
			fmt.Println(resp)
		}

	}
}
