<?php
namespace AcquireSdk\core;

class Log
{
   
    public static function writeLog($message, $level = "INFO")
    {               

        $logPath = LOG_PATH."/acquire_".date("Ymd").".log";
        $serverAddr = "127.0.0.1";
        if (isset($_SERVER["REMOTE_ADDR"])){
            $serverAddr = $_SERVER["REMOTE_ADDR"];
        }
        $messageFormat = "[". $level ."] [".gmdate("Y-m-d\TH:i:s\Z")."] ". $serverAddr." ". $message. "\n";
        $fp = fopen($logPath, "a+");
        fwrite($fp, $messageFormat);
        fclose($fp);
       
    }
}