Feature: Hero

  Background:
    Given a brand new hero

  Scenario: A hero has default values
    Then the hero has no name
    And the hero's alignment is Neutral
    And the hero's armor class is 10
    And the hero's hit points are 5
    And the hero's attack modifier is 0
    And the hero's attack damage is 1
    And the hero's crit damage is 2

  Scenario: Changing a hero's name
    When the hero's name is set to Bob
    Then the hero's name is Bob

  Scenario Outline: Changing a hero's alignment
    When the hero's alignment is set to <alignment>
    Then the hero's alignment is <alignment>

    Examples:
      | alignment |
      | Good      |
      | Neutral   |
      | Evil      |

  Scenario Outline: Damaging a hero
    When the hero takes <pts> points of damage
    Then the hero's hit points are <hp>
    And the hero's current hit points are <current_hp>
    And the hero is <status>

    Examples:
      | pts | hp | current_hp | status |
      |  1  |  5 |     4      |  alive |
      |  3  |  5 |     2      |  alive |
      |  5  |  5 |     0      |  dead  |
      |  7  |  5 |     0      |  dead  |
