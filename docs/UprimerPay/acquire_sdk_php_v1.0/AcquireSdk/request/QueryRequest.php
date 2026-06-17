<?php
namespace AcquireSdk\request;

require_once dirname(__FILE__)."/../loader.php";

use AcquireSdk\core\Request;
use AcquireSdk\enum\UriEnum;

/**
 * 交易查询,可以查询交易和退款
 * 文档地址：https://uprimer.net/open/docs/api#/api_jycx
 */
class QueryRequest extends Request
{
    public function create()
    {          
        $originalId = "1020241220000123";
        $url = HOST . UriEnum::$URI_TRANSACTION_QUEURY;    
        $url = str_replace("{originalId}", $originalId, $url);    
        $response = $this->httpGet($url);        
        return $response;
    }
}

$request = new QueryRequest();
$response = $request->create();
var_dump($response);

