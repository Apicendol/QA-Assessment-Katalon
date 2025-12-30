<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET List Users</name>
   <tag></tag>
   <elementGuidId>6be10ec9-5762-402f-ba52-e40b092f2189</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJlNTUwZGFjMjUwMjE4OTQyZjRmOTEzOTFiYjZkY2Nj NSIsInN1YiI6IjYwYWY2MDAxYzVjMWVmMDA1OWU2MzVlYiIsInNjb3BlcyI6WyJhcGlfcmVh ZCJdLCJ2ZXJzaW9uIjoxfQ.wGzbkGdNmzm7cWjLfyNFVGdpLJKMelfCaoEbUuYs-CA</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>e6301afe-0477-4022-bbe8-9677ac0c0782</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJlNTUwZGFjMjUwMjE4OTQyZjRmOTEzOTFiYjZkY2Nj NSIsInN1YiI6IjYwYWY2MDAxYzVjMWVmMDA1OWU2MzVlYiIsInNjb3BlcyI6WyJhcGlfcmVh ZCJdLCJ2ZXJzaW9uIjoxfQ.wGzbkGdNmzm7cWjLfyNFVGdpLJKMelfCaoEbUuYs-CA</value>
      <webElementGuid>be2857f0-dee0-4c38-acbf-b055f53d5230</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.3</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/api/users?page=1</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
