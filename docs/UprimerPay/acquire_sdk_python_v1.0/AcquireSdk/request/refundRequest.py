import sys
sys.path.append('../')
from core.request import Request
import datetime
import random
import json
from datetime import datetime

# 
#   交易退款
#   文档地址：https://uprimer.net/open/docs/api#/api_jytk
#  
class RefundRequest(Request):
    def __init__(self):
        super().__init__()

    def create(self,originalId,data):              
        url = Request.HOST + f"/api/acquire/payment/{originalId}/refund"           
        response = self.httpPost(url=url,data=data)
        return json.loads(response.text) 

if __name__ == '__main__': 
    
    originalId = "1020240726000028" 
    datetime_string = datetime.now().strftime("%Y%m%d%H%M%S")
    randNum = str(random.randint(0, 999999)).zfill(6)
    randomId = datetime_string + randNum      

    # dt = datetime.now()
    # refundTime = dt.strftime('%Y-%m-%dT%H:%M:%S%z')  
    # print(refundTime)
    # 注意：退款接口中的refundTime需要传带有“+0800”结尾的日期，否则报错“refundTime 退款时间不正确”，后续可能不是必须带”+0800“
    refundTime ="2024-02-29T09:28:21+0800"  

    data = {
        "amount": 89900,
        "appId": "10000",
        "currency": "CNY",
        "descriptor": "付款",
        "merchantOrderId": randomId,
        "refundReason": "不想要了",
        "refundTime": refundTime,
        "requestId": randomId,
        "notificationUrl": "https://hfgj.testpnr.com/crossDemo/webHook.do"
    }

    request = RefundRequest()
    response = request.create(originalId,data)
    print()
    print()
    print(f"接口返回={response}")