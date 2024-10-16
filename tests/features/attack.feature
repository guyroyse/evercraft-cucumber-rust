Feature: Attack

  Scenario Outline: A combatant hits another combatant
    Given an attacker
    And a defender with an armor class of <ac>
    When the attacker attacks with a roll of <roll>
    Then the attack is a "<result>"

    Examples:
      | ac | roll | result |
      |  8 |   5  |  miss  |
      | 10 |  10  |  hit   |
      | 12 |  15  |  hit   |
      | 15 |  20  |  hit   |
      | 25 |  20  |  hit   |
