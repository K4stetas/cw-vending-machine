# CosmWasm Vending Machine Smart Contract

It is an implementation of a vending machine, which can store 3 types of snacks: chocolate bars, water bottles and chips packets.
It also stores the owner's address of the instantiated contract.

To create it, you must pass the initial numbers of snacks:

```rust
pub struct _InstantiateMsg_ {
    pub chocolate_bars: u64,
    pub water_bottles: u64,
    pub chips_packets: u64,
}
```

## Messages

There are two messages supported by vending machine smart contract:

__GetItem__ { category } - removes 1 snack of the passed category. All users can use this message.
Attributes emitted:
_"left"_ - shows how many items of the passed category are left.

__Refill__ { number } - increases the number of snacks by the number. Only the owner is able to perfom refilling.
Attributes emitted:
_"chocolate_bars"_ - shows how many chocolate bars are left after the refilling
_"water_bottles"_ - shows how many water bottles are left after the refilling
_"chips_packets"_ - shows how many chips packets are left after the refilling

## Queries

__ItemsCount__ - returns how many snacks of each type are left in the machine
