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
      //items.push( new Item(this, BigInt(heldItems[i])) );
      items.push( new Item(this, parseInt(heldItems[i])) );
    }
    
    //Break up the operation used when inspecting an item
    this.op = data[2].split(" = ")[1].split(" ");
    
    //Which number to check against the item's worry level to decide which monkey to throw the item to
    //this.test = BigInt(data[3].split(" by ")[1]);
    this.test = parseInt(data[3].split(" by ")[1]);
    
    //Possible monkeys to throw the item to
    this.testResults = [
      parseInt(data[4].split(" monkey ")[1]),
      parseInt(data[5].split(" monkey ")[1])
    ];
    
    //Store the total number of inspections by this monkey
    this.inspections = 0;
    
  }
  
}

class Item {
  
  constructor(monkey, worry){
    
    //Store a reference to the current monkey holding this item
    this.monkey = monkey;
    
    //Store the initial worry level
    this.worry = worry;
    
    //Keep track of the possible remainders when the worry level is divided by any of the integers used by the monkeys to test
    /* Each entry is an object with two attributes:
    {
      test: [Monkey.test]
      remainder: [worry level % test]
    }
    */
    this.levels = [];
    
  }
  
  //Once all the monkeys have been initialised, the worry levels of this item can be tracked
  initialiseLevels(){
    
    for(var m = 0;m < monkeys.length;m ++){
      
      this.levels.push({
        test: monkeys[m].test,
        remainder: this.worry % monkeys[m].test
      });
      
    }
    
  }
  
  //Inspect the items, update the worry levels and return
  simulate(){
    
    //console.log(this.monkey.id, this.worry);
    //console.log(this.monkey.id, this.levels);
    //console.log(this.monkey.id);
    
    //Inspect
    this.updateLevels();
    
    //console.log(this.monkey.id, this.levels[this.monkey.id]);
    
    //Reduce worry level
    //if(reduce){
    //  this.worry = this.worry/3n;
    //}
    
    this.monkey.inspections ++;
    
    var previousMonkeyID = this.monkey.id;
    
    //Test & throw
    //if(this.worry % this.monkey.test === 0n){
    if(this.levels[this.monkey.id].remainder == 0){
      this.monkey = monkeys[ this.monkey.testResults[0] ];
    }else{
      this.monkey = monkeys[ this.monkey.testResults[1] ];
    }
    
    if(this.monkey.id > previousMonkeyID){
      this.simulate();
    }
    
  }
  
  updateLevels(add, mul){
    
    for(var l = 0;l < this.levels.length;l ++){
      
      var val2 = (this.monkey.op[2] == "old" ? this.levels[l].remainder : parseInt(this.monkey.op[2]));
      if(this.monkey.op[1] == "+"){
        this.levels[l].remainder += val2;
      }else if(this.monkey.op[1] == "*"){
        this.levels[l].remainder *= val2;
      }
      
      this.levels[l].remainder = this.levels[l].remainder % this.levels[l].test;
      
    }
    
  }
  
}

//Initialise monkeys & items
for(var m = 0;m < splitInput.length;m ++){
  
  var lines = splitInput[m].split("\n");
  
  monkeys.push(new Monkey(lines));
  
}

//Initialise item worry level remainders
for(var i = 0;i < items.length;i ++){
  
  items[i].initialiseLevels();
  
}

//console.log(monkeys);
//console.log(items);

for(var i = 0;i < items.length;i ++){
  
  //Simulate 20 rounds
  for(var r = 0;r < 10000;r ++){
    
    items[i].simulate();
    
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

console.log("Result 2: ",result);









