package com.acquire.sdk.request;

import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;
import java.util.Random;

import com.acquire.sdk.core.Request;
import com.acquire.sdk.enums.UriEnum;


/**
 * 交易退款
 * 文档地址：https://hwonline.cleattle.com/doc/acquire.html#id6
 */
class RefundRequest extends Request
{


    public static void main(String[] args) 
    {
        try {

            RefundRequest request = new RefundRequest();
            Map<String, Object> data = new HashMap<>();           
            String timestamp = System.currentTimeMillis() + "";
            SimpleDateFormat dateFormat2 = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ssZ");
            Date date = new Date();
            String orderTime = dateFormat2.format(date);

            data.put("amount", 89900);
            data.put("appId", "10000");
            data.put("currency", "CNY");
            data.put("descriptor", "付款");
            data.put("merchantOrderId", timestamp);
            data.put("refundReason", "不想要了");
            data.put("refundTime", orderTime);
            data.put("requestId", timestamp);
            data.put("notificationUrl", "https://hfgj.testpnr.com/crossDemo/webHook.do");

            String originalId = "920240418001029";
            String url = HOST + UriEnum.URI_PAYMENT_REFUND.getUri();  
            url = url.replace("{originalId}", originalId);   
            String response = request.httpPost(url,data);                 
            System.out.println();  
            System.out.println("RefundRequest: ");   
            System.out.println();   
            System.out.println("response: " + response);
            System.out.println();
        } catch (Exception e) {
            e.printStackTrace();
        }       
    }
  
}

