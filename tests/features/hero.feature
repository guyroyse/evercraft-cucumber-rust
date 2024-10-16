Feature: Hero

  Background:
    Given a brand new hero

  Scenario: A hero has default values
    Then the hero has no name
    Then the hero's alignment is Neutral
    Then the hero's armor class is 10
    Then the hero's hit points are 5

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
    Then the hero's hit points are 5
    And the hero's current hit points are <hp>
    And the hero is <status>

    Examples:
      | pts | hp | status |
      |  1  |  4 |  alive |
      |  3  |  2 |  alive |
      |  5  |  0 |   dead |
      |  6  | -1 |   dead |
