<?php
namespace AcquireSdk\core;

use AcquireSdk\enum\UriEnum;

class Request
{
    protected $log;
    protected static $config;
    private static $token = "";
    private static $expireIn = 300000; 
    private static $tokenFile = "token.json";
    private static $tokenFilePath = "";


    public function __construct() {        
        $this->log = new Log();
        $configPath = CONFIG_PATH."/config.json";    
        self::$config = json_decode(file_get_contents($configPath),true);  
        self::$tokenFilePath =  CONFIG_PATH."/". self::$tokenFile;  
        $this->getToken();
    }

    public function getToken()
    {    
        
        if (file_exists(self::$tokenFilePath)) {
            $tokenJson = file_get_contents(self::$tokenFilePath);
            $tokenArray = json_decode($tokenJson, true);
            if (isset($tokenArray['expireTime'])  && $tokenArray['expireTime'] > time() && 
                isset($tokenArray['token']) && trim($tokenArray['token']) != "") {            

                self::$token = $tokenArray['token']; 
                return self::$token;
            }
        }         
        
        $url = HOST . UriEnum::$URI_TOKEN_AUTH;       
        $responseJson = $this->httpGet($url);
        $this->log->writeLog($responseJson);
        $responseArray = json_decode($responseJson,true);
        if($responseArray['code'] == "00000000") {
            self::$token = $responseArray['data']['token'];
            self::$expireIn = $responseArray['data']['expireIn'];
            $tokenArray= array(
                'token' => self::$token,
                'expireTime' => time() + self::$expireIn
            );
            $tokenJson = json_encode($tokenArray);
            file_put_contents(self::$tokenFilePath, $tokenJson);
        }        
        return self::$token;
    }

    public function httpPost($url,$data)
    {  
        $logMessage = "httpPost param-url: ".$url.
                " param-data: ". json_encode($data,JSON_UNESCAPED_UNICODE);
        $this->log->writeLog($logMessage);  
        $curl = curl_init();
        curl_setopt_array($curl, array(
            CURLOPT_URL => $url,
            CURLOPT_RETURNTRANSFER => true,
            CURLOPT_ENCODING => "",
            CURLOPT_MAXREDIRS => 10,
            CURLOPT_TIMEOUT => 30,
            CURLOPT_HTTP_VERSION => CURL_HTTP_VERSION_1_1,
            CURLOPT_CUSTOMREQUEST => "POST",
            CURLOPT_POSTFIELDS => json_encode($data),          
            CURLOPT_HTTPHEADER => $this->setHttpPostHeader($data),
            CURLOPT_SSL_VERIFYHOST => false,
            CURLOPT_SSL_VERIFYPEER => false,
        ));
        $response = curl_exec($curl);
        $err = curl_error($curl);
        curl_close($curl);
        if ($err) {
            $logMessage = "httpPost response-error: ". $err;
            $this->log->writeLog($logMessage, "POST ERROR");   
            return "cURL Error #:" . $err;
        } else {
            $logMessage = "httpPost response-data: ". $response;
            $this->log->writeLog($logMessage);  
            return $response;
        }
    }

    public function httpGet($url){
        $logMessage = "httpGet param-url: ".$url;
        $this->log->writeLog($logMessage); 

        $curl = curl_init();
        curl_setopt_array($curl, array(
            CURLOPT_URL => $url,
            CURLOPT_RETURNTRANSFER => true,
            CURLOPT_ENCODING => "",
            CURLOPT_MAXREDIRS => 10,
            CURLOPT_TIMEOUT => 30,
            CURLOPT_HTTP_VERSION => CURL_HTTP_VERSION_1_1,
            CURLOPT_CUSTOMREQUEST => "GET",
            CURLOPT_HTTPHEADER => $this->setHttpGetHeader(),
            CURLOPT_SSL_VERIFYHOST => false,
            CURLOPT_SSL_VERIFYPEER => false,
        ));
        $response = curl_exec($curl);
        $err = curl_error($curl);
        curl_close($curl);
        if ($err) {
            $logMessage = "httpGet response-error: ". $err;
            $this->log->writeLog($logMessage,"GET ERROR");   
            return "cURL Error #:" . $err;
        } else {
            $logMessage = "httpGet response-data: ". $response;
            $this->log->writeLog($logMessage);  
            return $response;
        }
    }


    private function setHttpPostHeader($data)
    {        
        $header = array(
            "Content-Type: application/json",
            "Authorization: Bearer " . self::$token,
            "X-AccessCode: " .self::$config['accessCode'],   
            "X-Signature: ". $this->sign($data),
        );       
        return $header;
    }


    private function setHttpGetHeader()
    {         
        $header = array(
            "Content-Type: application/json",
            "Authorization: Bearer " . self::$token,
            "X-AccessCode: " .self::$config['accessCode'],
            "X-SecretKey: " .self::$config['secretKey']
        );     
        return $header;
    }

    private function sign($data)
    {
        $secretKey = self::$config['secretKey'];
        $sign = md5(json_encode($data).$secretKey); 
        return $sign;
    }
}