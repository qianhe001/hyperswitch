<?php
namespace AcquireSdk\request;

require_once dirname(__FILE__)."/../loader.php";

use AcquireSdk\core\Request;
use AcquireSdk\enum\UriEnum;
use DateTime as DateTimeImmutable;

/**
 * 交易退款
 * 文档地址：https://uprimer.net/open/docs/api#/api_jytk
 */
class RefundRequest extends Request
{
    public function create()
    {   
        $date = new DateTimeImmutable();
        $time = $date->format('Y-m-d\TH:i:sO'); 
        $data = array(
                "amount" => 89900,
                "appId" => "10000",
                "currency" => "CNY",
                "descriptor" => "付款",
                "merchantOrderId" => date("YmdHis").mt_rand(),
                "refundReason" => "不想要了",
                "refundTime" => $time,              
                "requestId" => date("YmdHis").mt_rand(),                
                "notificationUrl" => "https://hfgj.testpnr.com/crossDemo/webHook.do",                      
            );
        $originalId = "920240418001029";
        $url = HOST . UriEnum::$URI_PAYMENT_REFUND;  
        $url = str_replace("{originalId}", $originalId, $url);   
        $response = $this->httpPost($url,$data);        
        return $response;
    }
}

$request = new RefundRequest();
$response = $request->create();
var_dump($response);

