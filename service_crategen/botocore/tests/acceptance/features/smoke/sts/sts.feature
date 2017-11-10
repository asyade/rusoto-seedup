# language: en
@sts
Feature: AWS STS

  Scenario: Making a request
    When I call the "GetSessionToken" API
    Then the response should contain a "Credentials"

  Scenario: Handling errors
    When I attempt to call the "GetFederationToken" API with:
    | Name   | temp            |
    | Policy | {\"temp\":true} |
    Then I expect the response error code to be "MalformedPolicyDocument"
    And I expect the response error to contain a message
