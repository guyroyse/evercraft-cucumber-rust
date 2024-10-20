Feature: Attack

  Scenario Outline: A combatant attacks another combatant
    Given an attacker
    And a defender with an armor class of <ac>
    When the attacker attacks with a roll of <roll>
    Then the attack is a <result>

    Examples:
      | ac | roll | result |
      |  8 |   5  |  miss  |
      | 10 |  10  |  hit   |
      | 12 |  15  |  hit   |
      | 15 |  20  |  crit  |
      | 25 |  20  |  crit  |

  Scenario: A combatant hits another combatant
    Given an attacker
    And a defender with an armor class of 10
    When the attacker attacks with a roll of 15
    Then the attack is a hit
    And the defender is damaged for 1 point

  Scenario: A combatant crits another combatant
    Given an attacker
    And a defender with an armor class of 10
    When the attacker attacks with a roll of 20
    Then the attack is a crit
    And the defender is damaged for 2 points

