package request

import (
	"AcquireSdk/enum"
	"bytes"
	"crypto/md5"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"time"
)

type Request struct {
	Host          string
	Config        Config
	Token         Token
	ExpireIn      int    `default:"300000"`
	TokenFile     string `default:"token.json"`
	TokenFilePath string `default:""`
}

type Config struct {
	AccessCode string `json:"accessCode"`
	SecretKey  string `json:"secretKey"`
}

type Token struct {
	Token      string `json:"token"`
	ExpireTime int    `json:"expireTime"`
}

func NewRequest(debug bool) (*Request, error) {
	//默认是生产环境
	host := "https://acquire.uprimer.com"
	if debug {
		//测试环境
		host = "https://uatacquire.cloudpnr.com"

	}
	//读取config文件
	wd, _ := os.Getwd()
	configFilePath := wd + "/config/config.json"

	file, err := os.Open(configFilePath)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	data, err := io.ReadAll(file)
	if err != nil {
		return nil, err
	}

	var config Config
	json.Unmarshal(data, &config)

	request := &Request{Host: host, Config: config}
	tokenMap, err := request.GetToken()

	var token Token
	token.Token = tokenMap["token"].(string)
	token.ExpireTime = tokenMap["expireTime"].(int)
	request.Token = token
	return request, err
}

func (request *Request) GetToken() (map[string]interface{}, error) {

	wd, _ := os.Getwd()

	tokenFilePath := wd + "/config/token.json"
	_, err := os.Stat(tokenFilePath)
	if os.IsNotExist(err) || err != nil {
		fmt.Println("文件不存在或其他错误")
		return request.GetAndWriteTokenByPost()

	} else {
		fmt.Println("文件存在")
		file, _ := os.OpenFile(tokenFilePath, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0644)
		defer file.Close()

		data, err := io.ReadAll(file)
		if err != nil {
			return request.GetAndWriteTokenByPost()
		}

		var jsonData interface{}
		json.Unmarshal(data, &jsonData)

		dataMap, ok := jsonData.(map[string]interface{})
		if !ok {
			return request.GetAndWriteTokenByPost()
		}

		token, ok1 := dataMap["token"].(string)
		expireTime, ok2 := dataMap["expireTime"].(float64)

		if !ok1 || !ok2 {
			return request.GetAndWriteTokenByPost()
		}

		if GetCurrentTimestamp() < int(expireTime) {
			tokenMap := map[string]interface{}{"token": token, "expireTime": expireTime}
			return tokenMap, nil
		} else {
			return request.GetAndWriteTokenByPost()
		}
	}

}

func (request *Request) HttpGet(url string) ([]byte, error) {
	log.Println("HttpGet-url = " + url)
	req, _ := http.NewRequest("GET", url, nil)

	wd, _ := os.Getwd()

	configFilePath := wd + "/config/config.json"

	file, err := os.Open(configFilePath)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	data, err := io.ReadAll(file)
	if err != nil {
		return nil, err
	}

	var config Config
	json.Unmarshal(data, &config)

	req.Header.Add("Content-Type", "application/json")
	req.Header.Add("Authorization", "Bearer "+request.Token.Token)
	req.Header.Add("X-AccessCode", config.AccessCode)
	req.Header.Add("X-SecretKey", config.SecretKey)

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return nil, err
	} else {
		defer resp.Body.Close()

		body, err := io.ReadAll(resp.Body)
		if err != nil {
			return nil, err
		}

		return body, nil
	}

}

func (request *Request) GetAndWriteTokenByPost() (map[string]interface{}, error) {
	wd, _ := os.Getwd()
	tokenFilePath := wd + "/config/token.json"
	file, _ := os.OpenFile(tokenFilePath, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0644)
	defer file.Close()

	url := request.Host + enum.URI_TOKEN_AUTH

	resp, err := request.HttpGet(url)
	if err != nil {
		return nil, err
	}
	fmt.Println("Response body:", string(resp))

	var data interface{}
	json.Unmarshal(resp, &data)

	// code := data.(map[string]interface{})["code"].(string)
	// msg := data.(map[string]interface{})["msg"].(string)
	token := data.(map[string]interface{})["data"].(map[string]interface{})["token"].(string)
	expireIn := data.(map[string]interface{})["data"].(map[string]interface{})["expireIn"].(float64)

	expireTime := GetCurrentTimestamp() + int(expireIn)
	tokenMap := map[string]interface{}{"token": token, "expireTime": expireTime}
	jsonData, _ := json.Marshal(tokenMap)

	file.Write(jsonData)

	return tokenMap, nil
}

func GetCurrentTimestamp() int {
	now := time.Now()
	loc, _ := time.LoadLocation("Asia/Shanghai")
	currentTimestamp := int(now.In(loc).Unix())
	return currentTimestamp
}

func (request *Request) HttpPost(url string, params map[string]interface{}) (interface{}, error) {
	log.Println("HttpPost-url = " + url)
	log.Println("HttpPost-params = 下一行内容")
	log.Println(params)

	data, err := json.Marshal(params)
	if err != nil {
		log.Fatal(err)
		return nil, err
	}

	req, err := http.NewRequest("POST", url, bytes.NewBuffer(data))
	if err != nil {
		log.Fatal(err)
		return nil, err
	}
	token := request.Token.Token
	fmt.Println("HttpPost-token = " + token)
	sign := request.Sign(params)
	fmt.Println("HttpPost-sign = " + sign)
	fmt.Println("HttpPost-AccessCode = " + request.Config.AccessCode)

	req.Header.Add("Content-Type", "application/json")
	req.Header.Add("Authorization", "Bearer "+token)
	req.Header.Add("X-AccessCode", request.Config.AccessCode)
	req.Header.Add("X-Signature", request.Sign(params))

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		fmt.Println("HttpPost-错误 = 下一行")
		fmt.Println(err)
		log.Fatal(err)
		return nil, err
	} else {
		defer resp.Body.Close()

		body, err := io.ReadAll(resp.Body)
		if err != nil {
			return nil, err
		}

		var response interface{}
		json.Unmarshal(body, &response)
		fmt.Println("HttpPost-response = 下一行内容")
		fmt.Println(response)
		log.Println("HttpPost-解析后的返回内容 = 下一行内容")
		log.Println(response)

		return response, nil
	}
}

func (request *Request) Sign(params map[string]interface{}) string {
	secretKey := request.Config.SecretKey

	jsonData, err := json.Marshal(params)
	if err != nil {
		log.Fatal("Error encoding JSON:", err)
	}

	sign := md5.Sum(append(jsonData, []byte(secretKey)...))
	return hex.EncodeToString(sign[:])

}

func LogSettings() {
	wd, _ := os.Getwd()
	currentDate := time.Now().Format("20060102")
	logFilePath := wd + "/log/acquire_" + currentDate + ".log"

	logFile, err := os.OpenFile(logFilePath, os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0666)
	if err != nil {
		log.Fatal("Failed to open log file:", err)
	}

	log.SetOutput(logFile)

	// log.SetPrefix("LOG: ")
	log.SetFlags(log.Ldate | log.Ltime | log.Lshortfile)
	// log.Println("This is a log message.")

}
