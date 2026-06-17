import sys
sys.path.append('../')
from core.request import Request
import datetime
import random
import json
from datetime import datetime

# 
#  外卡收单直连 3DS 无摩擦
#  文档地址：https://uprimer.net/open/docs/api#/api_jk_zl
#  
class Payment3DChallengeRequest(Request):
    def __init__(self):
        super().__init__()    

    def create(self,data):              
        url = Request.HOST + "/api/acquire/payment/create"           
        response = self.httpPost(url=url,data=data)
        return json.loads(response.text)
       
if __name__ == '__main__':

    datetime_string = datetime.now().strftime("%Y%m%d%H%M%S")
    randNum = str(random.randint(0, 999999)).zfill(6)
    randomId = datetime_string + randNum      

    dt = datetime.now()
    orderTime = dt.strftime('%Y-%m-%dT%H:%M:%S%z')     

    data = {
            "amount": 89900,
            "appId": "10000",
            "currency": "HKD",
            "descriptor": "付款",
            "merchantOrderId": randomId,
            "requestId": randomId,
            "cancelUrl": "https://hfgj.testpnr.com/crossDemo/webHook.do",
            "successUrl": "https://hfgj.testpnr.com/crossDemo/webHook.do",
            "failureUrl": "https://hfgj.testpnr.com/crossDemo/webHook.do",
            "notificationUrl": "https://hfgj.testpnr.com/crossDemo/webHook.do",
            "orderTime": orderTime,
            "paymentMethod": {
                "methodType": "CARD",
                "card": {
                    "cvv": "133",
                    "expiryMonth": "10",
                    "expiryYear": "26",
                    "firstName": "xingguo",
                    "lastName": "xu",
                    "number": "5200000000001096",
                    "billing": {
                        "firstName": "xingguo",
                        "lastName": "xu",
                        "dateOfBirth": "",
                        "phoneNumber": "11144442121",
                        "email": "xu.xg@qq.com",
                        "countryCode": "GB",
                        "state": "UK",
                        "city": "Toronto",
                        "street": "160-500 University",
                        "postCode": "12345-1233"
                    }
                }
            },
            "products": [
                {
                    "code": "101110",
                    "name": "iphone",
                    "quantity": 1,
                    "sku": "black",
                    "unitPrice": 6000,
                    "totalAmount": 6000
                }
            ],
            "shipping": {
                "company": "shipping company",
                "firstName": "lucy",
                "lastName": "king",
                "phoneNumber": "13388888888",
                "countryCode": "GB",
                "state": "UK",
                "city": "Toronto",
                "street": "160-500 University",
                "street2": "st2",
                "postCode": "Box1026"
            },
            "deviceData": {
                "acceptHeader": "text/html",
                "browserJavaEnabled": "true",
                "browserJavascriptEnabled": "true",
                "browserUserAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.183",
                "challengeWindow": "5",
                "language": "zh-CN",
                "screenColorDepth": "48",
                "screenHeight": "1200",
                "screenWidth": "1600",
                "timezone": "60"
            }
        }
    
    request = Payment3DChallengeRequest()
    response = request.create(data)
    
    print()
    print()
    print(f"接口返回={response}")

    if response.get('code') == '00000000':
        url = response.get('data').get('nextAction').get('url')
        print()
        print()
        print(f"请在浏览器访问该地址：{url}")