package com.acquire.sdk.request;

import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Date;
import java.util.Random;

import com.acquire.sdk.core.Request;
import com.acquire.sdk.enums.UriEnum;
import com.alibaba.fastjson.JSONObject;

/**
 * 外卡收单收银台
 * 文档地址：https://hwonline.cleattle.com/doc/acquire.html#id4
 */
class CheckoutRequest extends Request
{
    public static void main(String[] args) 
    {
        try {
   
            CheckoutRequest request = new CheckoutRequest();

            Map<String, Object> data = new HashMap<>();
            String timestamp = System.currentTimeMillis() + "";
            SimpleDateFormat dateFormat2 = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ssZ");
            Date date = new Date();
            String orderTime = dateFormat2.format(date);
        
            data.put("amount", 19900);
            data.put("currency", "HKD");
            data.put("appId", "10000");
            data.put("merchantOrderId", timestamp);
            data.put("requestId", timestamp);
            data.put("validityPeriod", 500);
            data.put("orderTime", orderTime);
            Map<String, Object> paymentMethod = new HashMap<>();
            paymentMethod.put("methodType", "CARD");
            data.put("paymentMethod", paymentMethod);
            Map<String, Object> shipping = new HashMap<>();
            shipping.put("city", "Toronto");
            shipping.put("countryCode", "CA");
            shipping.put("firstName", "XINGGUO");
            shipping.put("lastName", "XU");
            shipping.put("phoneNumber", "+8618672362337");
            shipping.put("postCode", "M5G 1V7");
            shipping.put("state", "ON");
            shipping.put("street", "160-500 University");
            shipping.put("street2", "");
            data.put("shipping", shipping);
            List<Map<String, Object>> products = new ArrayList<>();
            Map<String, Object> product = new HashMap<>();
            product.put("code", "4098755");
            product.put("name", "High-Collar Boxy Camel Hair Blend Sweater,MUMUXI LED Fairy Lights Battery Operated String Lights [12 Pack] 7.2ft 20 Battery Powered LED Lights | Mini Lights, Centerpiece Table Decorations, Wedding Party Bedroom Mason Jar Christmas, Warm White");
            product.put("quantity", 1);
            product.put("sku", "4098755 black");
            product.put("unitPrice", 100);
            products.add(product);
            data.put("products", products);
            data.put("cancelUrl", "https://hfgj.testpnr.com/crossDemo/webHook.do");
            data.put("notificationUrl", "https://hfgj.testpnr.com/crossDemo/webHook.do");
            data.put("successUrl", "https://hfgj.testpnr.com/crossDemo/webHook.do");
            data.put("failureUrl", "https://hfgj.testpnr.com/crossDemo/webHook.do");

            String url = HOST + UriEnum.URI_CHECKOUT_CREATE.getUri();
            String response = request.httpPost(url, data);
            System.out.println();  
            System.out.println("CheckoutRequest: "); 
            System.out.println();    
            System.out.println("response: " + response);
            System.out.println();
            JSONObject jsonObject = JSONObject.parseObject(response); 
            String code = jsonObject.getString("code");
            System.out.println("response-code: " + code);
            System.out.println();
            String nextActionUrl = jsonObject.getJSONObject("data").getJSONObject("nextAction").getString("url");
            System.out.println("请在浏览器访问该地址: " + nextActionUrl);
            System.out.println();
            
        } catch (Exception e) {
            e.printStackTrace();
        }
        
    }
  
}

