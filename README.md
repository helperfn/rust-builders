 learning how builder pattern works in rust

patterns -> different from algorithms
generally applied across OOP languages (but OOPS is not lang-specific, it is a way of programming)

there are 3 types:
creational patterns : factory method, builder

structural patterns : ?? dont know the ones listed

behavourial patterns : iterator

OOPS in rust

rust doesnt support classes or inheritance directly. you can use structs, enums to implement them.

pros of builders in rust:
- implement oops concept
- make it readable and also not require to list every single parameter each type youre.. constructing ? something

cons:
- while implemetning, i realised that i needed a lot of defintions
- it is only necessary for complex tasks

implementation:
based on https://refactoring.guru/design-patterns/builder using house (and car) as example, i decided to make a playerbuilder. this is to show different constraints and how i can apply complex ones using this

game concept:


## Game Concept

A player can be a warrior, mage, rogue, or healer.  

Qualities like health, strength, agility, and intelligence determine abilities.  

Weapon and armor determine physical capabilities and are optional.

---

## Base Stats

| Class   | Base Strength | Base Agility | Base Intelligence | Base Health |

|--------|---------------|--------------|-------------------|------------|

| Warrior | 15 | 8  | 3  | 120 |

| Mage    | 3  | 8  | 15 | 80  |

| Rogue   | 10 | 14 | 6  | 100 |

| Healer  | 5  | 10 | 14 | 90  |

---

## Weapon Rules

| Class   | Allowed Weapons | Notes |

|--------|-----------------|-------|

| Warrior | Sword, Axe      | Should have a weapon |

| Mage    | Staff           | Optional |

| Rogue   | Bow, Dagger     | Optional |

| Healer  | None            | Cannot equip weapon |

---

## Armor Rules

| Class   | Allowed Armor | Notes |

|--------|---------------|-------|

| Warrior | Light, Heavy  | Should have armor |

| Mage    | None          | Cannot equip armor |

| Rogue   | Light, Medium | Optional |

| Healer  | Light, Medium | Should have armor |

---

## Attribute Rules

| Attribute | Rule |

|----------|------|

| Strength | Default from class, can override |

| Agility | Default from class, can override |

| Intelligence | Default from class, can override |

| Health | Always from class (no override) |

| Name | Required |

| Class | Required |

| Weapon | Optional (validated if present) |

| Armor | Optional (validated if present) |

| Stats | Optional overrides |

| Health (final) | Derived only |
