import sys
sys.path.append('../')
from core.request import Request
import datetime
import random
import json
from datetime import datetime

# 
#  外卡收单收银台
#  文档地址：https://uprimer.net/open/docs/api#/api_jk_syt
#  
class CheckoutRequest(Request):
    def __init__(self):       
        super().__init__()

    def create(self,data):        
        url = Request.HOST + "/api/acquire/checkout/create"           
        response = self.httpPost(url=url,data=data)
        return json.loads(response.text)
       

if __name__ == '__main__':

    datetime_string = datetime.now().strftime("%Y%m%d%H%M%S")
    randNum = str(random.randint(0, 999999)).zfill(6)
    randomId = datetime_string + randNum      

    dt = datetime.now()
    orderTime = dt.strftime('%Y-%m-%dT%H:%M:%S%z')    

    data = {
        "amount": 19900,
        "currency": "HKD",
        "appId": "10000",
        "merchantOrderId": randomId,
        "requestId": randomId,
        "validityPeriod": 500,
        "orderTime": orderTime,
        "paymentMethod": {
            "methodType": "CARD"
        },
        "shipping": {
            "city": "Toronto",
            "countryCode": "CA",
            "firstName": "XINGGUO",
            "lastName": "XU",
            "phoneNumber": "+8618672362337",
            "postCode": "M5G 1V7",
            "state": "ON",
            "street": "160-500 University",
            "street2":  ""
        },
        "products": [
            {
                "code": "4098755",
                "name": "High-Collar Boxy Camel Hair Blend Sweater,MUMUXI LED Fairy Lights Battery Operated String Lights [12 Pack] 7.2ft 20 Battery Powered LED Lights | Mini Lights, Centerpiece Table Decorations, Wedding Party Bedroom Mason Jar Christmas, Warm White",
                "quantity": 1,
                "sku": "4098755 black",
                "unitPrice": 100
            }
        ],
        "cancelUrl": "https://hfgj.testpnr.com/crossDemo/webHook.do",
        "notificationUrl": "https://hfgj.testpnr.com/crossDemo/webHook.do",
        "successUrl": "https://hfgj.testpnr.com/crossDemo/webHook.do",
        "failureUrl": "https://hfgj.testpnr.com/crossDemo/webHook.do"
    }

    request = CheckoutRequest()
    response = request.create(data)

    print()
    print()
    print(f"接口返回={response}")

    if response.get('code') == '00000000':
        url = response.get('data').get('nextAction').get('url')
        print()
        print()
        print(f"请在浏览器访问该地址：{url}")
