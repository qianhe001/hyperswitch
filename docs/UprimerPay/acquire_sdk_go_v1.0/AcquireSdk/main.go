package main

import (
	"AcquireSdk/request"
	"log"
)

func main() {

	// 日志设置
	request.LogSettings()

	//初始化获取token，当前为测试环境，生产环境设置为debug=false
	debug := true
	r, err := request.NewRequest(debug)
	if err != nil {
		log.Fatal(err)
	}

	/**
	 * 外卡收单收银台
	 * 文档地址：https://hwonline.cleattle.com/doc/acquire.html#id4
	 */
	r.CreateCheckOut()

	/**
	 * 外卡收单直连 3DS 挑战
	 * 文档地址：https://hwonline.cleattle.com/doc/acquire.html#id5
	 */
	r.CreatePayment3DChallengeRequest()

	/**
	 * 外卡收单直连 3DS 无摩擦
	 * 文档地址：https://hwonline.cleattle.com/doc/acquire.html#id5
	 */
	r.CreatePayment3DFrictionlessRequest()

	/**
	 * 外卡收单直连 No 3DS
	 * 文档地址：https://hwonline.cleattle.com/doc/acquire.html#id5
	 */
	r.CreatePaymentNo3DRequest()

	/**
	 * 交易查询,可以查询交易和退款
	 * 文档地址：https://hwonline.cleattle.com/doc/acquire.html#id7
	 */
	r.CreateQueryRequest()

	/**
	 * 交易退款
	 * 文档地址：https://hwonline.cleattle.com/doc/acquire.html#id6
	 */
	r.CreateRefundRequest()

}
