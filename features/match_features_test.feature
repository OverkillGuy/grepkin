Feature: Matching reference feature files to grepkin tests
  As a developer
  I need to check that reference Gherkin features match with grepkin test comments
  So that I know my project has the right features

Scenario: Single reference feature matches up with test code
  Given a reference Gherkin feature in "features/"
  And test code under "tests/"
  When I parse the project via grepkin
  Then both reference feature and grepkin test are found
  And reference feature matches test code
