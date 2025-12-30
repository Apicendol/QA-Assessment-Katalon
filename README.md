# QA-Assessment-Katalon
Automation tests using Katalon Studio for QA Assessment

## Tech Stack
- Katalon Studio 10.x
- Groovy
- Reqres API

## Test Scenarios
1. GET List Users: Retrieves a list of users
2. GET Single User: Retrieves a single user and saves the email to a Global Variable
3. PUT Update User: Updates user data
4. POST Register Successful: Register using the email obtained from the previous scenario

## How to Run
1. Open the project in Katalon Studio
2. Make sure the Global Variable `baseUrl` in the default profile is filled with `https://reqres.in/`.
3. Select Test Suite `TS_apiTest`
4. Click Run
