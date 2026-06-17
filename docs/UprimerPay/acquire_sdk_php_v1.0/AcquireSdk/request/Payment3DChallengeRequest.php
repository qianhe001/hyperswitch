<?php
namespace AcquireSdk\request;

require_once dirname(__FILE__)."/../loader.php";

use AcquireSdk\core\Request;
use AcquireSdk\enum\UriEnum;
use DateTime as DateTimeImmutable;

/**
 * 外卡收单直连 3DS 挑战
 * 文档地址：https://uprimer.net/open/docs/api#/api_jk_zl
 */
class Payment3DChallengeRequest extends Request
{
    public function create()
    {   
        $date = new DateTimeImmutable();
        $time = $date->format('Y-m-d\TH:i:sO'); 
        $data = array(
                "amount" => 290000,
                "appId" => "10000",
                "currency" => "CNY",
                "descriptor" => "付款",
                "merchantOrderId" => date("YmdHis").mt_rand(),
                "requestId" => date("YmdHis").mt_rand(),
                "cancelUrl" => "https://hfgj.testpnr.com/crossDemo/webHook.do",
                "successUrl" => "https://hfgj.testpnr.com/crossDemo/webHook.do",                
                "failureUrl" => "https://hfgj.testpnr.com/crossDemo/webHook.do",              
                "notificationUrl" => "https://hfgj.testpnr.com/crossDemo/webHook.do",
                "orderTime" => $time,
                "paymentMethod" => array(
                    "methodType" => "CARD",
                    "card" => array(
                        "cvv" => "133",
                        "expiryMonth" => "10",
                        "expiryYear" => "26",
                        "firstName" => "xingguo",
                        // "firstName" => "",
                        "lastName" => "xu",
                        // "lastName" => "REFUSEDRC82MAC01",
                        "number" => "5200000000001096",
                        "billing" => array(
                            "firstName" => "xingguo",
                            "lastName" => "xu",
                            "dateOfBirth" => "",
                            "phoneNumber" => "11144442121",
                            "email" => "xu.xg@qq.com",
                            "countryCode" => "GB",
                            "state" => "UK",
                            "city" => "Toronto",
                            "street" => "160-500 University",
                            "postCode"=> "12345-1233"
                        )
                    )
                ),
                "products" => array(
                    array(
                        "code" => "101110",
                        "name" => "iphone",
                        "quantity" => 1,
                        "sku" => "black",
                        "unitPrice" => 6000,
                        "totalAmount" => 6000
                    )
                ),
                "shipping" => array(
                    "company" => "shipping company",
                    "firstName" => "lucy",
                    "lastName" => "king",
                    "phoneNumber" => "13388888888",
                    "countryCode" => "GB",
                    "state" => "UK",
                    "city" => "Toronto",
                    "street" => "160-500 University",
                    "street2" => "st2",
                    "postCode" => "Box1026"
                ),
                "deviceData" => array(
                    "acceptHeader" => "text/html",
                    "browserJavaEnabled" => "true",
                    "browserJavascriptEnabled" => "true",
                    "browserUserAgent" => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.183",
                    "challengeWindow" => "5",
                    "language" => "zh-CN",
                    "screenColorDepth" => "48",
                    "screenHeight" => "1200",
                    "screenWidth" => "1600",
                    "timezone" => "60"
                )
            );
    
        $url = HOST . UriEnum::$URI_PAYMENT_CREATE;      
        $response = $this->httpPost($url,$data);        
        return $response;
    }
}

$request = new Payment3DChallengeRequest();
$response = $request->create();
$redirectUrl = json_decode($response,true)['data']['nextAction']['url'];
var_dump($response);
var_dump("请在浏览器访问该地址：".$redirectUrl);
