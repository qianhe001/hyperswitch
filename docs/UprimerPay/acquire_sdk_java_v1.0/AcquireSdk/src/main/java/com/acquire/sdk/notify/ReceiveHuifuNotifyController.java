  
/**
* Copyright (c) 2020-2024 恒新  All rights reserved.
* https://www.cleattle.com 
*  
*/  

package com.acquire.sdk.notify;

import javax.servlet.http.HttpServletRequest;

import org.apache.commons.codec.digest.DigestUtils;
import org.apache.commons.lang3.StringUtils;
import org.springframework.stereotype.Component;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.ResponseBody;

import com.acquire.sdk.core.Request;

/**  
 * 接收汇付交易结果异步回调通知示例
*/
@Component
@RequestMapping("/notify")
public class ReceiveHuifuNotifyController extends Request{
	
	// private Request request = new Request();
	
    @PostMapping("/receive")
    @ResponseBody
	public String test(HttpServletRequest httpRequest) throws Exception{
    	// 获取目标签名串
		String targetSign = httpRequest.getHeader("X-Signature");
		
		// 获取请求体的输入流
        byte[] buffer = new byte[httpRequest.getContentLength()];
        httpRequest.getInputStream().read(buffer);
        String requestBody = new String(buffer, "UTF-8");
		System.out.println(requestBody);
		
		// 生成MD5
		String currentSign = DigestUtils.md5Hex(requestBody + config.get("secretKey"));
		
		// 比较签名串
		if(StringUtils.equals(targetSign, currentSign)){
			return "签名验证通过";
		}else {
			return "签名验证失败";
		}
	}
}
