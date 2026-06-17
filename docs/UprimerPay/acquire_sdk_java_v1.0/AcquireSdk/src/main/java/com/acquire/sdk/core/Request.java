package com.acquire.sdk.core;

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.File;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.Map;
import org.apache.http.impl.client.CloseableHttpClient;
import org.apache.commons.codec.digest.DigestUtils;
import org.apache.http.HttpEntity;
import org.apache.http.client.methods.CloseableHttpResponse;
import org.apache.http.client.methods.HttpGet;
import org.apache.http.client.methods.HttpPost;
import org.apache.http.client.methods.HttpRequestBase;
import org.apache.http.entity.ContentType;
import org.apache.http.entity.StringEntity;
import org.apache.http.impl.client.HttpClients;
import com.acquire.sdk.enums.UriEnum;
import com.acquire.sdk.util.LoggerUtil;
import com.alibaba.fastjson.JSON;
import com.alibaba.fastjson.JSONException;
import com.alibaba.fastjson.JSONObject;
import java.io.FileWriter;
import java.lang.Object;
import java.net.URLDecoder;

public class Request {

    public static boolean DEBUG = true;
    public static String HOST = "https://acquire.uprimer.com";  
    protected static Map<String,String> config;
    private static String token = null;
    private static int expireIn = 300000; 
    private static long expireTime = 0;
    private static String tokenFile = "token.json";
    private static String configFile = "config.json";

    public Request()
    {       
        if(DEBUG) {
            HOST = "https://uatacquire.cloudpnr.com";
        }

        if(Request.config == null) {
            getConfig();
        }

        long currentTime = System.currentTimeMillis();
        if(Request.token == null || Request.expireTime < currentTime) {
            getToken();
        }
      
    }

    private void getConfig()
    { 
        Map<String,String> config = new HashMap<>();
        try {
            File file = new File(Request.class.getResource("").getPath()); 
            String currentDir = file.getAbsolutePath();
            String filepath =  currentDir + "/../config/" + configFile;
            filepath = URLDecoder.decode(filepath, "UTF-8");
            String content = new String(Files.readAllBytes(Paths.get(filepath)));           
            JSONObject jsonObject = JSONObject.parseObject(content);              
            config.put("accessCode", jsonObject.getString("accessCode"));
            config.put("secretKey", jsonObject.getString("secretKey"));            
        } catch (Exception e) {
            e.printStackTrace();
        }
        Request.config = config; 
    }
    
    public String getToken(){   
        File file = new File(Request.class.getResource("").getPath()); 
        String currentDir = file.getAbsolutePath();       
        String filepath =  currentDir + "/../config/" + tokenFile;  
        long currentTime = System.currentTimeMillis();
        try {       
            filepath = URLDecoder.decode(filepath, "UTF-8");       
            String content = new String(Files.readAllBytes(new File(filepath).toPath()), StandardCharsets.UTF_8);
            JSONObject jsonObject = JSONObject.parseObject(content);
            String token = jsonObject.getString("token"); 
            long expireTime = jsonObject.getLong("expireTime");            
            if (expireTime > currentTime) {
                Request.token = token;
                return Request.token;
            }     
        } catch (IOException | JSONException e) {
            e.printStackTrace();                       
        }        
        getTokenByRequest(filepath);     
        return Request.token;      
    }

    private void getTokenByRequest(String filepath)
    {
        try {
            String url = HOST + UriEnum.URI_TOKEN_AUTH.getUri();               
            String response = httpGet(url);
            JSONObject jsonObject = JSONObject.parseObject(response);      
            String code = jsonObject.getString("code");            
            if(code.equals("00000000")) {                   
                Request.token = jsonObject.getJSONObject("data").getString("token");  
                Request.expireIn = jsonObject.getJSONObject("data").getIntValue("expireIn");
                JSONObject jsonObject2 = new JSONObject(); 
                jsonObject2.put("token", Request.token);
                long currentTime = System.currentTimeMillis();
                Request.expireTime = Request.expireIn + currentTime;
                jsonObject2.put("expireTime", Request.expireTime);                
                try(BufferedWriter writer = new BufferedWriter(new FileWriter(filepath, false))) {
                    writer.write(jsonObject2.toJSONString());
                    writer.close();
                } catch (IOException ex) {               
                    ex.printStackTrace();
                   
                }    
            }
        } catch (Exception e) {
            e.printStackTrace();
        }      

    }

    public String httpPost(String url, Map<String,Object> data) 
    {       
        try { 
            LoggerUtil.info("httpPost url: " + url);
            LoggerUtil.info("httpPost data: " + data.toString());     
            HttpPost httpPost = new HttpPost(url);              
            httpPost.addHeader("Content-Type", "application/json");            
            httpPost.addHeader("Authorization", "Bearer " + token);
            httpPost.addHeader("X-AccessCode", Request.config.get("accessCode"));
            httpPost.addHeader("X-Signature", sign(data));          
            String json = JSON.toJSONString(data);
            StringEntity entity = new StringEntity(json, ContentType.APPLICATION_JSON);
            httpPost.setEntity(entity);
            return httpRequest(httpPost);
        } catch (Exception e) {  
            LoggerUtil.severe("httpPost exception: " + e.toString());       
            e.printStackTrace();    
            return e.getMessage();
        }
      
    }


    public String httpGet(String url)
    {
        try {
            LoggerUtil.info("httpGet url: " + url);           
            HttpGet httpGet = new HttpGet(url);   
            httpGet.addHeader("Content-Type", "application/json");            
            httpGet.addHeader("Authorization", "Bearer " + token);
            httpGet.addHeader("X-AccessCode", Request.config.get("accessCode"));
            httpGet.addHeader("X-SecretKey", Request.config.get("secretKey"));
            return httpRequest(httpGet);
        } catch (Exception e) {  
            LoggerUtil.severe("httpGet exception: " + e.toString());         
            e.printStackTrace();
            return e.getMessage();
        }
    
    }

    public static String httpRequest(HttpRequestBase httpRequest) 
    {
        try {
            CloseableHttpClient httpclient = HttpClients.createDefault();          
            CloseableHttpResponse response = httpclient.execute(httpRequest);
            HttpEntity entity = response.getEntity();
            BufferedReader in = new BufferedReader(new InputStreamReader(entity.getContent()));
            String inputLine;
            StringBuffer responseBuffer = new StringBuffer();
            while ((inputLine = in.readLine()) != null) {
                responseBuffer.append(inputLine);
            }
            in.close();  
            LoggerUtil.info("httpPost response: " + responseBuffer.toString());            
            return responseBuffer.toString();
        } catch (Exception e) {
            LoggerUtil.severe("httpRequest exception: " + e.toString());          
            return e.getMessage();
        }      
    }

    private String sign(Map<String,Object> data) 
    {        
        String json = JSON.toJSONString(data);
        String sign = DigestUtils.md5Hex(json + Request.config.get("secretKey"));
        return sign;
    }


}
