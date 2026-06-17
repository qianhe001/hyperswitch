import sys
sys.path.append('../') 
from core.request import Request

# 
#   获取token
#   文档地址：https://uprimer.net/open/docs/specification#/jsgf_tksq * 
#  
class TokenRequest(Request):
    def __init__(self):
        super().__init__()

if __name__ == '__main__': 
    request = TokenRequest()
    token = request.getToken()
    print()
    print("token:  ")
    print()
    print(token)