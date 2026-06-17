import sys
sys.path.append('../')
import requests
import json
import time
import hashlib
import os
import traceback
from core import log
import logging

class Request:
    token = ""
    expireTime = 0
    config = {}
    DEBUG = True
    HOST = ""
    
    def __init__(self):   
        configFile = '../config/config.json' 
        if not os.path.exists(configFile):
            print("ERROR: config file doesn't exist!")
            sys.exit()
        
        if not Request.config:
            with open('../config/config.json', 'r') as f:            
                content = f.read()
                contentDict = json.loads(content)
                if not contentDict.get('accessCode') or not contentDict.get('secretKey'):
                    print("ERROR: config file doesn't contain required keys or values!")  
                    sys.exit()             
                Request.config = json.loads(content)    
     
            
        if Request.DEBUG:
            Request.HOST = "https://uatacquire.cloudpnr.com"
        else:    
            Request.HOST = "https://acquire.uprimer.com"

        self.getToken()

    def getToken(self):
        timestamp = int(time.time()) + 10
        if Request.token and Request.expireTime > timestamp:          
            return Request.token

        else:  
            tokenFile = '../config/token.json'
            if os.path.exists(tokenFile):                   
                with open(tokenFile, 'r') as f:            
                    content = f.read()                
                    contentDict = json.loads(content)   
                    if contentDict.get('token') and contentDict.get('expireTime') > timestamp:
                        Request.token = contentDict.get('token')
                        Request.expireTime = contentDict.get('expireTime')
                        return Request.token


            url = Request.HOST + "/authorize"           
            response = self.httpGet(url=url)
            responseDict = json.loads(response.text)
           
            if responseDict.get('code') == '00000000':
                Request.token = responseDict['data']['token']
                Request.expireTime = timestamp + responseDict['data']['expireIn']            
                tokenJson = json.dumps({
                    "token": Request.token,
                    "expireTime": Request.expireTime
                })
                with open(tokenFile, "w") as file:
                    file.write(tokenJson)
                
                return Request.token
       
    
    def httpPost(self, **kwargs):
        try:
            url = kwargs['url']   
            data = json.dumps(kwargs['data'])
            headers = {
                "Content-Type": "application/json", 
                "Authorization": "Bearer " + Request.token,         
                "X-AccessCode": self.config['accessCode'],
                "X-SecretKey": self.config['secretKey'],
                "X-Signature":self.sign(data)
            }     
            logging.info(f"httpPost params url={url}")         
            logging.info(f"httpPost params data={data}")
            logging.info(f"httpPost params headers={headers}")           
            response = requests.post(url,data=data,headers=headers)
            logging.info(f"httpPost response={response.text}") 
            logging.info("--------------------")
            return response
        except Exception as e:
            print()
            print("httpPost请求异常： ")
            print()
            traceback.print_exc()
            logging.exception("httpPost exception occurred: %s", e)
            logging.info("--------------------")


    def httpGet(self, **kwargs):
        try:
            url = kwargs['url']   
            headers = {
                "Content-Type": "application/json", 
                "Authorization": "Bearer " + Request.token,         
                "X-AccessCode": self.config['accessCode'],
                "X-SecretKey": self.config['secretKey'],
            }     
            logging.info(f"httpGet params url={url}")
            logging.info(f"httpGet params headers={headers}")           
            response = requests.get(url,headers=headers)
            logging.info(f"httpGet response={response.text}") 
            logging.info("--------------------")
            return response
        except Exception as e:
            print()
            print("httpGet请求异常： ")
            print()
            traceback.print_exc()
            logging.exception("httpGet exception occurred: %s", e)
            logging.info("--------------------")

    def sign(self,data):      
        string = data + self.config['secretKey']       
        sign = hashlib.md5(string.encode('utf-8')).hexdigest()    
        return sign     


