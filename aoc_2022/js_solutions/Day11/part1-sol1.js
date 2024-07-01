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

//Define a monkey and its method(s)
class Monkey {
  
  constructor(data){
    
    //Store item worry levels
    this.items = data[1].split(": ")[1].split(", ");
    for(var i = 0;i < this.items.length;i ++){
      this.items[i] = BigInt(this.items[i]);
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

//Initialise monkeys
for(var m = 0;m < splitInput.length;m ++){
  
  var lines = splitInput[m].split("\n");
  
  monkeys.push(new Monkey(lines));
  
}

//Simulate 20 rounds
for(var r = 0;r < 20;r ++){
  
  for(var m = 0;m < monkeys.length;m ++){
    monkeys[m].simulate(true);
  }
  
}

//console.log(monkeys);

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



/// This version of the solution would work for part 2 with unlimited memory & time, but on my computer we don't have that luxury
/*
//Reinitialise monkeys
monkeys = [];
for(var m = 0;m < splitInput.length;m ++){
  
  var lines = splitInput[m].split("\n");
  
  monkeys.push(new Monkey(lines));
  
}

//Simulate 10000 rounds
for(var r = 0;r < 10000;r ++){
  
  if(r%50 == 0){
    console.log(r);
  }
  
  for(var m = 0;m < monkeys.length;m ++){
    monkeys[m].simulate(true);
  }
  
}

console.log(monkeys);

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







