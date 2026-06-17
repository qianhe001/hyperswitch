<?php
namespace AcquireSdk\notify;

require_once dirname(__FILE__)."/../loader.php";

use AcquireSdk\core\Request;

/**
 * 接收汇付交易结果异步回调通知示例
 */
class ReceiveHuifuNotifyController extends Request
{
	public function callback() 
	{
		$responseBody = file_get_contents("php://input");
		$sign = $_SERVER['HTTP_X_Signature'];	
		$this->log->writeLog("callback responseBody: ". $responseBody);  
		$this->log->writeLog("callback sign: ". $sign);  

		//回调的sign 和 回调数据生成的sign 对比	

        if ($sign == md5(json_encode($responseBody).self::$config['secretKey'])) {
			$this->log->writeLog("callback check sign result: ". "验签成功");  
		} else {
			$this->log->writeLog("callback check sign result: ". "验签失败");  
		}

	}
}