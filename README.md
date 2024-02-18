# CosmWasmContract
Vending machine

This is a implementation of a vending machine, which can store 3 types of snacks: chocolates, water bottle and chips.
It also stores the owner of the instantiated vending machine.

To create it, you must pass the initial numbers of snacks:

pub struct InstantiateMsg {
    pub chocolates: u64,
    pub water_bottles: u64,
    pub chips: u64,
}

Messages

There are two messages supported by vending machine contract:

GetItem { category } - removes 1 snack of the inserted category. All users can use this message.

Attributes emitted:
left - shows how many items of the insered category are left.


Refill { number } - increases the number of snacks by the amount. Only owner are able to perfom refill.

Attributes emitted:
"chocolates" - shows how many chocolates are left after the refilling
"water_bottle" - shows how many water bottles are left after the refilling
"chips" - shows how many chips are left after the refilling

Queries

ItemsCount - returns how many snacks of each type are left in the machine
