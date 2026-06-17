<?php
namespace AcquireSdk;

ini_set('date.timezone', 'Asia/Shanghai');
ini_set('display_errors', '1');
ini_set('display_startup_errors', '1');
error_reporting(E_ALL);

if(!defined("DEBUG")) {
    define("DEBUG",true);
}

if(!defined("HOST")) {
    DEBUG ? define("HOST","https://uatacquire.cloudpnr.com") : define("HOST","https://acquire.uprimer.com");
}

if(!defined("BASE_PATH")) {
    define("BASE_PATH",dirname(__FILE__));
}

if(!defined("CONFIG_PATH")) {
    define("CONFIG_PATH",dirname(__FILE__) . "/config");
}

if(!defined("LOG_PATH")) {
    define("LOG_PATH",dirname(__FILE__) . "/log");
}


require_once BASE_PATH . "/core/Log.php";

require_once BASE_PATH . "/core/Request.php";

require_once BASE_PATH . "/enum/UriEnum.php";


