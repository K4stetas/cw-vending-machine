# CosmWasm Vending Machine Smart Contract

This is a implementation of a vending machine, which can store 3 types of snacks: chocolates, water bottle and chips.
It also stores the owner of the instantiated vending machine.

To create it, you must pass the initial numbers of snacks:

```rust
pub struct _InstantiateMsg_ {
    pub chocolates: u64,
    pub water_bottles: u64,
    pub chips: u64,
}
```

## Messages
___

There are two messages supported by vending machine contract:

__GetItem__ { category } - removes 1 snack of the inserted category. All users can use this message.
Attributes emitted:
_"left"_ - shows how many items of the insered category are left.

__Refill__ { number } - increases the number of snacks by the amount. Only owner are able to perfom refill.
Attributes emitted:
_"chocolates"_ - shows how many chocolates are left after the refilling
_"water_bottle"_ - shows how many water bottles are left after the refilling
_"chips"_ - shows how many chips are left after the refilling

## Queries
___

__ItemsCount__ - returns how many snacks of each type are left in the machine
