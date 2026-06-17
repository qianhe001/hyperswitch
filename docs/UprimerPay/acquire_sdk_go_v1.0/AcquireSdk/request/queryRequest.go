package request

import (
	"AcquireSdk/enum"
	"fmt"
	"log"
	"strings"
)

/**
 * 交易查询,可以查询交易和退款
 * 文档地址：https://uprimer.net/open/docs/api#/api_jycx
 */
func (request *Request) CreateQueryRequest() {
	originalId := "1020240726000028"
	url := request.Host + enum.URI_TRANSACTION_QUEURY
	url = strings.Replace(url, "{originalId}", originalId, 1)

	log.Println("请求地址url= " + url)
	resp, err := request.HttpGet(url)

	fmt.Println("请求返回错误 = 看下一行")
	fmt.Println(err)
	fmt.Println("请求返回数据 =" + string(resp))

}
