/**

Anthony Wilson - Advent of Code 2022 - Day 11

2022-12-11

**/

var input = `Monkey 0:
  Starting items: 91, 58, 52, 69, 95, 54
  Operation: new = old * 13
  Test: divisible by 7
    If true: throw to monkey 1
    If false: throw to monkey 5

Monkey 1:
  Starting items: 80, 80, 97, 84
  Operation: new = old * old
  Test: divisible by 3
    If true: throw to monkey 3
    If false: throw to monkey 5

Monkey 2:
  Starting items: 86, 92, 71
  Operation: new = old + 7
  Test: divisible by 2
    If true: throw to monkey 0
    If false: throw to monkey 4

Monkey 3:
  Starting items: 96, 90, 99, 76, 79, 85, 98, 61
  Operation: new = old + 4
  Test: divisible by 11
    If true: throw to monkey 7
    If false: throw to monkey 6

Monkey 4:
  Starting items: 60, 83, 68, 64, 73
  Operation: new = old * 19
  Test: divisible by 17
    If true: throw to monkey 1
    If false: throw to monkey 0

Monkey 5:
  Starting items: 96, 52, 52, 94, 76, 51, 57
  Operation: new = old + 3
  Test: divisible by 5
    If true: throw to monkey 7
    If false: throw to monkey 3

Monkey 6:
  Starting items: 75
  Operation: new = old + 5
  Test: divisible by 13
    If true: throw to monkey 4
    If false: throw to monkey 2

Monkey 7:
  Starting items: 83, 75
  Operation: new = old + 1
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 6`;



var splitInput = input.split("\n\n");

//Store a reference to all monkeys. Used by the Monkey class.
var monkeys = [];

//Store a reference to all items.
var items = [];

//Define a monkey and its method(s)
class Monkey {
  
  constructor(data){
    
    this.id = parseInt(data[0].split(" ")[1].replace(":",""));
    
    //Create items
    var heldItems = data[1].split(": ")[1].split(", ");
    for(var i = 0;i < heldItems.length;i ++){
      items.push( new Item(this, BigInt(heldItems[i])) );
    }
    
    //Break up the operation used when inspecting an item
    this.op = data[2].split(" = ")[1].split(" ");
    
    //Which number to check against the item's worry level to decide which monkey to throw the item to
    this.test = BigInt(data[3].split(" by ")[1]);
    
    //Possible monkeys to throw the item to
    this.testResults = [
      parseInt(data[4].split(" monkey ")[1]),
      parseInt(data[5].split(" monkey ")[1])
    ];
    
    //Store the total number of inspections by this monkey
    this.inspections = 0;
    
  }
  
  //Inspect the items, update the worry level and return
  simulate(reduce){
    
    //Check all items in order
    while(this.items.length > 0){
      
      //Inspect
      var val2 = (this.op[2] == "old" ? this.items[0] : BigInt(this.op[2]));
      if(this.op[1] == "+"){
        this.items[0] = this.items[0] + val2;
      }else if(this.op[1] == "*"){
        this.items[0] = this.items[0] * val2;
      }
      
      //Reduce worry level
      if(reduce){
        this.items[0] = this.items[0]/3n;
      }
      
      //Test & throw
      if(this.items[0] % this.test === 0n){
        monkeys[ this.testResults[0] ].items.push(this.items[0]);
      }else{
        monkeys[ this.testResults[1] ].items.push(this.items[0]);
      }
      this.items.splice(0,1);
      
      this.inspections ++;
      
    }
    
  }
  
}

class Item {
  
  constructor(monkey, worry){
    
    this.monkey = monkey;
    
    this.worry = worry;
    
  }
  
  //Inspect the items, update the worry level and return
  simulate(reduce){
    
    //console.log(this.monkey.id, this.worry);
    
    //Inspect
    var val2 = (this.monkey.op[2] == "old" ? this.worry : BigInt(this.monkey.op[2]));
    if(this.monkey.op[1] == "+"){
      this.worry = this.worry + val2;
    }else if(this.monkey.op[1] == "*"){
      this.worry = this.worry * val2;
    }
    
    //console.log(this.monkey.id, this.worry%this.monkey.test);
    
    //Reduce worry level
    if(reduce){
      this.worry = this.worry/3n;
    }
    
    this.monkey.inspections ++;
    
    var previousMonkeyID = this.monkey.id;
    
    //Test & throw
    if(this.worry % this.monkey.test === 0n){
      this.monkey = monkeys[ this.monkey.testResults[0] ];
    }else{
      this.monkey = monkeys[ this.monkey.testResults[1] ];
    }
    
    if(this.monkey.id > previousMonkeyID){
      this.simulate(reduce);
    }
    
  }
  
}

//Initialise monkeys & items
for(var m = 0;m < splitInput.length;m ++){
  
  var lines = splitInput[m].split("\n");
  
  monkeys.push(new Monkey(lines));
  
}

//console.log(monkeys);
//console.log(items);

for(var i = 0;i < items.length;i ++){
  
  //Simulate 20 rounds
  for(var r = 0;r < 20;r ++){
    
    items[i].simulate(true);
    
  }
  
}

//console.log(monkeys);
//console.log(items);

//Find the two most active monkeys for the result
var mostActive = [0,0];
var result = 1;

for(var a = 0;a < 2;a ++){
  
  for(var m = 1;m < monkeys.length;m ++){
    
    if(monkeys[m].inspections > monkeys[mostActive[a]].inspections){
      mostActive[a] = m;
    }
    
  }
  
  result *= monkeys[mostActive[a]].inspections;
  monkeys.splice(mostActive[a],1);
}

console.log("Result: ",result);



///Part 2 still requires more resources than we have access to
/*
//Reinitialise monkeys
monkeys = [];
for(var m = 0;m < splitInput.length;m ++){
  
  var lines = splitInput[m].split("\n");
  
  monkeys.push(new Monkey(lines));
  
}

//console.log(monkeys);
//console.log(items);

//for(var i = 0;i < items.length;i ++){
for(var i = 0;i < 1;i ++){
  
  console.log("Item",i);
  
  //Simulate 20 rounds
  for(var r = 0;r < 40;r ++){
    
    items[i].simulate(false);
    
  }
  
  console.log(items[i].monkey.id, items[i].worry);
  
}

console.log(monkeys);
//console.log(items);

//Find the two most active monkeys for the result
var mostActive2 = [0,0];
var result2 = 1;

for(var a = 0;a < 2;a ++){
  
  for(var m = 1;m < monkeys.length;m ++){
    
    if(monkeys[m].inspections > monkeys[mostActive2[a]].inspections){
      mostActive2[a] = m;
    }
    
  }
  
  result2 *= monkeys[mostActive2[a]].inspections;
  monkeys.splice(mostActive2[a],1);
}

console.log("Result: ",result2);
*/







