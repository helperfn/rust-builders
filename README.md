# Learning the Builder Pattern in Rust

## Overview

- **Design patterns** are reusable solutions to common design problems  
- They are different from algorithms  
- Commonly used across OOP-style programming (not language-specific)

---

## Types of Design Patterns

### 1. Creational Patterns
- Factory Method  
- Builder  

### 2. Structural Patterns
- (Not explored in this context)

### 3. Behavioral Patterns
- Iterator  

---

## OOP in Rust

Rust does not support traditional **classes** or **inheritance**.

Instead, similar concepts are achieved using:
- `struct`
- `enum`
- Traits (for shared behavior)

---

## Builder Pattern in Rust

### Pros

- Helps model OOP-style construction patterns
- Improves readability when creating complex objects
- Avoids passing many parameters in constructors
- Allows step-by-step object construction

### Cons

- Requires additional boilerplate (structs, methods, validation logic)
- Not necessary for simple data structures
- Best suited for complex object creation with constraints

---

## Implementation Approach

Based on:  
https://refactoring.guru/design-patterns/builder

Instead of typical examples (house/car), this implementation uses a **PlayerBuilder** to demonstrate:
- Validation rules
- Conditional constraints
- Derived attributes

---

## Game Concept

A player can belong to one of the following classes:
- Warrior  
- Mage  
- Rogue  
- Healer  

Each class determines base stats:
- Health  
- Strength  
- Agility  
- Intelligence  

Additional attributes:
- Weapon (optional, class-dependent)
- Armor (optional, class-dependent)

---

## Base Stats by Class

| Class   | Strength | Agility | Intelligence | Health |
|--------|----------|---------|--------------|--------|
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

- **Name** → Required  
- **Class** → Required  

- **Weapon**
  - Optional
  - Must follow class constraints  

- **Armor**
  - Optional
  - Must follow class constraints  

- **Stats (Strength, Agility, Intelligence)**
  - Default from class
  - Can be overridden  

- **Health**
  - Always derived from class
  - Cannot be overridden  

---

## Key Idea

The **Builder Pattern** is useful here because:

- Object creation involves multiple optional and required fields
- Several constraints must be validated
- Some values are derived (e.g., health)
- It keeps construction logic clean and readable
