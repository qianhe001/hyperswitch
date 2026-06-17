package com.acquire.sdk.request;

import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.Date;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Random;

import com.acquire.sdk.core.Request;
import com.acquire.sdk.enums.UriEnum;
import com.alibaba.fastjson.JSONObject;

/**
 * 外卡收单直连 3DS 无摩擦
 * 文档地址：https://hwonline.cleattle.com/doc/acquire.html#id5
 */
class Payment3DFrictionlessRequest extends Request
{
    public static void main(String[] args) 
    {      
        try {

            Payment3DFrictionlessRequest request = new Payment3DFrictionlessRequest();

            Map<String, Object> data = new HashMap<>();
            String timestamp = System.currentTimeMillis() + "";
            SimpleDateFormat dateFormat2 = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ssZ");
            Date date = new Date();
            String orderTime = dateFormat2.format(date);

            data.put("amount", 89900);
            data.put("appId", "10000");
            data.put("currency", "HKD");
            data.put("descriptor", "付款");
            data.put("merchantOrderId", timestamp);
            data.put("requestId", timestamp);
            data.put("cancelUrl", "https://hfgj.testpnr.com/crossDemo/webHook.do");
            data.put("successUrl", "https://hfgj.testpnr.com/crossDemo/webHook.do");
            data.put("failureUrl", "https://hfgj.testpnr.com/crossDemo/webHook.do");
            data.put("notificationUrl", "https://hfgj.testpnr.com/crossDemo/webHook.do");
            data.put("orderTime", orderTime);
            Map<String, Object> paymentMethod = new HashMap<>();
            paymentMethod.put("methodType", "CARD");
            Map<String, Object> card = new HashMap<>();
            card.put("cvv", "133");
            card.put("expiryMonth", "10");
            card.put("expiryYear", "26");
            card.put("firstName", "xingguo");
            card.put("lastName", "xu");
            card.put("number", "5200000000002235");
            Map<String, Object> billing = new HashMap<>();
            billing.put("firstName", "xingguo");
            billing.put("lastName", "xu");
            billing.put("dateOfBirth", "");
            billing.put("phoneNumber", "11144442121");
            billing.put("email", "xu.xg@qq.com");
            billing.put("countryCode", "GB");
            billing.put("state", "UK");
            billing.put("city", "Toronto");
            billing.put("street", "160-500 University");
            billing.put("postCode", "12345-1233");
            card.put("billing", billing);
            paymentMethod.put("card", card);
            data.put("paymentMethod", paymentMethod);
            List<Map<String, Object>> products = new ArrayList<>();
            Map<String, Object> product = new HashMap<>();
            product.put("code", "101110");
            product.put("name", "iphone");
            product.put("quantity", 1);
            product.put("sku", "black");
            product.put("unitPrice", 6000);
            product.put("totalAmount", 6000);
            products.add(product);
            data.put("products", products);
            Map<String, Object> shipping = new HashMap<>();
            shipping.put("company", "shipping company");
            shipping.put("firstName", "lucy");
            shipping.put("lastName", "king");
            shipping.put("phoneNumber", "13388888888");
            shipping.put("countryCode", "GB");
            shipping.put("state", "UK");
            shipping.put("city", "Toronto");
            shipping.put("street", "160-500 University");
            shipping.put("street2", "st2");
            shipping.put("postCode", "Box1026");
            data.put("shipping", shipping);
            Map<String, Object> deviceData = new HashMap<>();
            deviceData.put("acceptHeader", "text/html");
            deviceData.put("browserJavaEnabled", "true");
            deviceData.put("browserJavascriptEnabled", "true");
            deviceData.put("browserUserAgent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36 Edg/115.0.1901.183");
            deviceData.put("challengeWindow", "5");
            deviceData.put("language", "zh-CN");
            deviceData.put("screenColorDepth", "48");
            deviceData.put("screenHeight", "1200");
            deviceData.put("screenWidth", "1600");
            deviceData.put("timezone", "60");
            data.put("deviceData", deviceData);

            String url = HOST + UriEnum.URI_PAYMENT_CREATE.getUri();
            String response = request.httpPost(url, data);
            System.out.println();  
            System.out.println("PaymentRequest: "); 
            System.out.println();
            System.out.println("response: " + response);
            System.out.println();
            JSONObject jsonObject = JSONObject.parseObject(response); 
            String code = jsonObject.getString("code");
            System.out.println("response-code: " + code);
            System.out.println();
            if(jsonObject.getJSONObject("data").containsKey("nextAction")) {
            String nextActionUrl = jsonObject.getJSONObject("data").getJSONObject("nextAction").getString("url");
            	System.out.println("请在浏览器访问该地址: " + nextActionUrl);
            }
            System.out.println();
            
        } catch (Exception e) {
            e.printStackTrace();
        }
        
    }
}
