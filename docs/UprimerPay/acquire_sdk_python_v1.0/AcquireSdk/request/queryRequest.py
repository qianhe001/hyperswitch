import sys
sys.path.append('../') 
from core.request import Request
import json

# 
#   交易查询,可以查询交易和退款
#   文档地址：https://uprimer.net/open/docs/api#/api_jycx
#  
class QueryRequest(Request):
    def __init__(self):
        super().__init__()

    def create(self,originalId):              
        url = Request.HOST + f"/api/acquire/payment/{originalId}/get"           
        response = self.httpGet(url=url)
        return json.loads(response.text) 

if __name__ == '__main__': 

    originalId = "1020240726000028" 
    request = QueryRequest()
    response = request.create(originalId)
    print()
    print()
    print(f"接口返回={response}")