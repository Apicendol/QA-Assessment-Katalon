<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT Update</name>
   <tag></tag>
   <elementGuidId>1ab6d632-7bdd-45c3-80c7-850e268d20e9</elementGuidId>
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
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;first_name\&quot;: \&quot;John\&quot;, \n  \&quot;last_name\&quot;: \&quot;Doe\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJlNTUwZGFjMjUwMjE4OTQyZjRmOTEzOTFiYjZkY2Nj NSIsInN1YiI6IjYwYWY2MDAxYzVjMWVmMDA1OWU2MzVlYiIsInNjb3BlcyI6WyJhcGlfcmVh ZCJdLCJ2ZXJzaW9uIjoxfQ.wGzbkGdNmzm7cWjLfyNFVGdpLJKMelfCaoEbUuYs-CA</value>
      <webElementGuid>13ee3cc6-7979-427b-b414-4fb3c6947506</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c825894f-ec8b-4ce5-a9c5-494d1087cc6a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.3</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/api/users/2</restUrl>
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
