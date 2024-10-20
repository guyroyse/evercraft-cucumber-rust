Feature: Ability

  Background:
    Given a new ability

  Scenario: An ability has default values
    Then the ability score is 10
    And the ability modifier is 0

  Scenario: Changing an ability score
    When the ability score is set to 15
    Then the ability score is 15
    And an Ok result is returned

  Scenario: Changing an ability score to less than 1
    When the ability score is set to 0
    Then an Err result is returned
    And the error message is 'Score was 0. Ability score must be between 1 and 20.'

  Scenario: Changing an ability score to more than 20
    When the ability score is set to 21
    Then an Err result is returned
    And the error message is 'Score was 21. Ability score must be between 1 and 20.'

  Scenario Outline: Changing an ability modifier
    When the ability score is set to <score>
    Then the ability modifier is <modifier>

    Examples:
      | score | modifier |
      |   1   |    -5    |
      |   2   |    -4    |
      |   3   |    -4    |
      |   4   |    -3    |
      |   5   |    -3    |
      |   6   |    -2    |
      |   7   |    -2    |
      |   8   |    -1    |
      |   9   |    -1    |
      |  10   |     0    |
      |  11   |     0    |
      |  12   |    +1    |
      |  13   |    +1    |
      |  14   |    +2    |
      |  15   |    +2    |
      |  16   |    +3    |
      |  17   |    +3    |
      |  18   |    +4    |
      |  19   |    +4    |
      |  20   |    +5    |
