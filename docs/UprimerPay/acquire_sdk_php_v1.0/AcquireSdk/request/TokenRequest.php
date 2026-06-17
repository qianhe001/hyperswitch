<?php
namespace AcquireRequest;

require_once dirname(__FILE__)."/../loader.php";

use AcquireSdk\core\Request;

/**
 * 获取token
 * 文档地址：https://uprimer.net/open/docs/specification#/jsgf_tksq * 
 */
$request = new Request();

$token = $request->getToken();
var_dump($token);