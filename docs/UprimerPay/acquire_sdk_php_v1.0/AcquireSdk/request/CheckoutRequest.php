<?php
namespace AcquireSdk\request;

require_once dirname(__FILE__)."/../loader.php";

use AcquireSdk\core\Request;
use AcquireSdk\enum\UriEnum;
use DateTime as DateTimeImmutable;

/**
 * 外卡收单收银台
 * 文档地址：https://uprimer.net/open/docs/api#/api_jk_syt
 */
class CheckoutRequest extends Request
{
    public function create()
    {   
        $date = new DateTimeImmutable();
        $time = $date->format('Y-m-d\TH:i:sO'); 
        $data = array(
            "eci" => "00",
            "amount" => 50,
            "currency" => "HKD",
            "appId" => "10000",
            "merchantOrderId" => date("YmdHis").mt_rand(),
            "requestId" => date("YmdHis").mt_rand(),
            "validityPeriod" => 500,
            "orderTime" => $time,
            "paymentMethod" => array(
                "methodType" => "CARD",
                "card" => array(
                    "cvv" => "133",
                    "expiryMonth" => "10",
                    "expiryYear" => "26",
                    "firstName" => "xingguo",
                    "lastName" => "xu",
                    // "firstName" => " ",
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
            "shipping" => array(
                "city" => "Toronto",
                "countryCode" => "CA",
                "firstName" => "XINGGUO",
                "lastName" => "XU",
                "phoneNumber" => "+8618672362337",
                "postCode" => "M5G 1V7",
                "state" => "ON",
                "street" => "160-500 University",
                "street2"=>  ""
            ),
            "products" => array(
                array(
                    "code" => "4098755",
                    "name" => "High-Collar Boxy Camel Hair Blend Sweater,MUMUXI LED Fairy Lights Battery Operated String Lights [12 Pack] 7.2ft 20 Battery Powered LED Lights | Mini Lights, Centerpiece Table Decorations, Wedding Party Bedroom Mason Jar Christmas, Warm White",
                    "quantity"=> 1,
                    "sku" => "4098755 black",
                    "unitPrice" => 100
                )
            ),
            // "cancelUrl" => "https://hfgj.testpnr.com/crossDemo/webHook.do",
            "cancelUrl" => "https://www.baidu.com",
            "notificationUrl" => "https://hfgj.testpnr.com/crossDemo/webHook.do",
            "successUrl" => "https://hfgj.testpnr.com/crossDemo/webHook.do",
            "failureUrl" => "https://hfgj.testpnr.com/crossDemo/webHook.do"
      
            );
        var_dump($data);
        $url = HOST . UriEnum::$URI_CHECKOUT_CREATE;      
        $response = $this->httpPost($url,$data);        
        return $response;
    }
}

$request = new CheckoutRequest();
$response = $request->create();
$redirectUrl = json_decode($response,true)['data']['nextAction']['url'];
var_dump($response);
var_dump("URL：".$redirectUrl);
