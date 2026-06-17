package com.acquire.sdk.request;

import com.acquire.sdk.core.Request;
import com.acquire.sdk.enums.UriEnum;

/**
 * 交易查询,可以查询交易和退款
 * 文档地址：https://hwonline.cleattle.com/doc/acquire.html#id7
 */
class QueryRequest extends Request
{
    public static void main(String[] args) 
    {
        try {

            QueryRequest request = new QueryRequest();
            String originalId = "1020240726000028";
            String url = HOST + UriEnum.URI_TRANSACTION_QUEURY.getUri();    
            url = url.replace("{originalId}", originalId); 
            String response = request.httpGet(url);
            System.out.println();  
            System.out.println("QueryRequest: ");   
            System.out.println();   
            System.out.println("response: " + response);
            System.out.println();
        } catch (Exception e) {
            e.printStackTrace();
        }
       
    }
   
}

