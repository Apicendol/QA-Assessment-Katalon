<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Register</name>
   <tag></tag>
   <elementGuidId>f2b4756f-0c96-4b6a-96f7-f90fe3663049</elementGuidId>
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
  &quot;text&quot;: &quot;{\n  \&quot;email\&quot;: \&quot;${userEmail}\&quot;,\n  \&quot;password\&quot;: \&quot;pistol\&quot;\n}\n&quot;,
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
      <webElementGuid>4656479f-8745-4998-af55-89ba3537ac06</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>203606be-d84f-4664-9663-6711a750af7f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.3</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/api/register</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5ca5bc5e-af2d-422d-ac26-41d6fd3dd1e8</id>
      <masked>false</masked>
      <name>userEmail</name>
   </variables>
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
