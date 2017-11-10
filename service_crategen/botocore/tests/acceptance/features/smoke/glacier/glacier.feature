# language: en
@glacier
Feature: Amazon Glacier

  Scenario: Making a request
    When I call the "ListVaults" API
    Then the response should contain a "VaultList"

  Scenario: Handling errors
    When I attempt to call the "ListVaults" API with:
    | accountId | abcmnoxyz |
    Then I expect the response error code to be "UnrecognizedClientException"
    And I expect the response error to contain a message
