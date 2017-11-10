# language: en
@acm
Feature: AWS Certificate Manager

  Scenario: Making a request
    When I call the "ListCertificates" API
    Then the value at "CertificateStatuses" should be a list

  Scenario: Handling errors
    When I attempt to call the "GetCertificate" API with:
        | CertificateArn   | arn:aws:acm:region:123456789012:certificate/12345678-1234-1234-1234-123456789012 |
    Then I expect the response error to contain a message
