Feature: Attack

  Background:
    Given an attacker with an attack damage of 2 and a crit damage of 4

  Scenario Outline: An attacker attacks a defender
    Given a defender with an armor class of <ac>
    When the attacker attacks with a roll of <roll>
    Then the attack is a <result>

    Examples:
      | ac | roll | result |
      |  8 |   5  |  miss  |
      | 10 |  10  |  hit   |
      | 12 |  15  |  hit   |
      | 15 |  20  |  crit  |
      | 25 |  20  |  crit  |

  Scenario: An attacker hits a defender
    Given a defender with an armor class of 10
    When the attacker attacks with a roll of 15
    Then the attack is a hit
    And the defender is damaged for 2 point

  Scenario: An attacker crits a defender
    Given a defender with an armor class of 10
    When the attacker attacks with a roll of 20
    Then the attack is a crit
    And the defender is damaged for 4 points
