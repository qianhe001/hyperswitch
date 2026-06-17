package request

import (
	"AcquireSdk/enum"
	"fmt"
	"log"
	"math/rand"
	"strings"
	"time"
)

/**
 * 交易退款
 * 文档地址：https://uprimer.net/open/docs/api#/api_jytk
 */
func (request *Request) CreateRefundRequest() {
	date := time.Now().UTC()
	now := date.Format("2006-01-02T15:04:05Z")
	data := map[string]interface{}{
		"amount":          89900,
		"appId":           "10000",
		"currency":        "CNY",
		"descriptor":      "付款",
		"merchantOrderId": fmt.Sprintf("%s%d", date.Format("20060102150405"), rand.Intn(100000)),
		"refundReason":    "不想要了",
		"refundTime":      now,
		"requestId":       fmt.Sprintf("%s%d", date.Format("20060102150405"), rand.Intn(100000)),
		"notificationUrl": "https://hfgj.testpnr.com/crossDemo/webHook.do",
	}
	originalId := "920240418001029"
	url := request.Host + enum.URI_PAYMENT_REFUND
	url = strings.Replace(url, "{originalId}", originalId, 1)

	log.Println("请求地址url= " + url)
	resp, err := request.HttpPost(url, data)

	fmt.Println("请求返回错误 = 看下一行")
	fmt.Println(err)
	fmt.Println("请求返回数据 = 看下一行")
	fmt.Println(resp)

}
