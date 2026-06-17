package com.acquire.sdk.request;

import com.acquire.sdk.core.Request;

class TokenRequest extends Request
{   
 
    public static void main(String[] args) 
    {
        try {

            TokenRequest request = new TokenRequest();
            System.out.println("TokenRequest: ");
            System.out.println();
            System.out.println("token: " + request.getToken());
            System.out.println();
            
        } catch (Exception e) {
            e.printStackTrace();
        }
        
    }
}