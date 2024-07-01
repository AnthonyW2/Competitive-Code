/**

Anthony Wilson - Advent of Code 2022 - Day 5

2022-12-5

**/

var startingConfig = `    [V] [G]             [H]        
[Z] [H] [Z]         [T] [S]        
[P] [D] [F]         [B] [V] [Q]    
[B] [M] [V] [N]     [F] [D] [N]    
[Q] [Q] [D] [F]     [Z] [Z] [P] [M]
[M] [Z] [R] [D] [Q] [V] [T] [F] [R]
[D] [L] [H] [G] [F] [Q] [M] [G] [W]
[N] [C] [Q] [H] [N] [D] [Q] [M] [B]
 1   2   3   4   5   6   7   8   9 `.split("\n");

var procedure = `move 3 from 2 to 5
move 2 from 9 to 6
move 4 from 7 to 1
move 7 from 3 to 4
move 2 from 9 to 8
move 8 from 8 to 6
move 1 from 7 to 4
move 8 from 6 to 4
move 4 from 5 to 7
move 3 from 4 to 9
move 2 from 6 to 3
move 11 from 4 to 1
move 1 from 3 to 4
move 2 from 3 to 1
move 1 from 7 to 6
move 14 from 1 to 6
move 7 from 4 to 3
move 2 from 5 to 9
move 5 from 6 to 4
move 9 from 6 to 1
move 3 from 4 to 8
move 1 from 7 to 6
move 3 from 4 to 1
move 7 from 3 to 8
move 5 from 9 to 5
move 4 from 1 to 4
move 3 from 7 to 2
move 5 from 6 to 2
move 3 from 4 to 1
move 7 from 8 to 5
move 3 from 6 to 8
move 11 from 2 to 1
move 1 from 4 to 3
move 1 from 3 to 9
move 2 from 2 to 9
move 8 from 5 to 4
move 1 from 1 to 7
move 1 from 9 to 5
move 8 from 4 to 1
move 1 from 6 to 8
move 2 from 9 to 1
move 4 from 5 to 3
move 2 from 7 to 3
move 40 from 1 to 2
move 24 from 2 to 9
move 1 from 5 to 6
move 11 from 2 to 3
move 9 from 3 to 5
move 12 from 9 to 4
move 6 from 5 to 7
move 4 from 7 to 4
move 2 from 5 to 1
move 2 from 1 to 9
move 1 from 6 to 8
move 9 from 4 to 8
move 6 from 4 to 9
move 17 from 9 to 6
move 1 from 4 to 6
move 17 from 6 to 5
move 1 from 1 to 4
move 2 from 7 to 9
move 1 from 6 to 7
move 2 from 2 to 9
move 2 from 7 to 2
move 6 from 3 to 8
move 3 from 5 to 9
move 1 from 4 to 9
move 2 from 3 to 7
move 4 from 5 to 6
move 1 from 7 to 4
move 1 from 4 to 2
move 1 from 7 to 5
move 9 from 8 to 1
move 1 from 1 to 2
move 2 from 9 to 3
move 7 from 2 to 7
move 1 from 9 to 5
move 12 from 8 to 7
move 3 from 1 to 9
move 2 from 6 to 4
move 9 from 9 to 3
move 1 from 6 to 7
move 1 from 9 to 5
move 1 from 6 to 1
move 9 from 7 to 1
move 7 from 1 to 8
move 4 from 3 to 9
move 5 from 7 to 1
move 3 from 9 to 1
move 4 from 7 to 2
move 12 from 1 to 5
move 2 from 9 to 4
move 7 from 8 to 2
move 7 from 5 to 7
move 4 from 3 to 4
move 1 from 8 to 1
move 2 from 2 to 1
move 2 from 3 to 1
move 3 from 2 to 7
move 13 from 5 to 4
move 1 from 8 to 3
move 1 from 3 to 8
move 1 from 3 to 5
move 1 from 8 to 7
move 17 from 4 to 8
move 5 from 2 to 6
move 2 from 1 to 6
move 5 from 6 to 3
move 9 from 7 to 1
move 4 from 4 to 3
move 1 from 6 to 2
move 4 from 7 to 4
move 1 from 6 to 5
move 2 from 3 to 2
move 15 from 1 to 4
move 6 from 5 to 4
move 4 from 3 to 5
move 4 from 5 to 2
move 2 from 2 to 4
move 11 from 8 to 1
move 2 from 8 to 3
move 5 from 3 to 7
move 4 from 2 to 8
move 2 from 2 to 9
move 4 from 7 to 8
move 11 from 4 to 6
move 2 from 5 to 4
move 3 from 6 to 9
move 4 from 1 to 4
move 15 from 4 to 9
move 1 from 7 to 3
move 2 from 1 to 2
move 6 from 4 to 5
move 11 from 8 to 2
move 16 from 9 to 4
move 2 from 9 to 1
move 4 from 2 to 3
move 8 from 4 to 9
move 1 from 8 to 7
move 5 from 4 to 7
move 6 from 7 to 3
move 10 from 9 to 5
move 5 from 3 to 1
move 1 from 1 to 4
move 5 from 1 to 9
move 5 from 1 to 7
move 5 from 4 to 1
move 4 from 1 to 6
move 3 from 1 to 9
move 10 from 5 to 9
move 2 from 7 to 1
move 5 from 3 to 6
move 4 from 5 to 7
move 4 from 2 to 6
move 2 from 5 to 6
move 5 from 2 to 7
move 18 from 6 to 1
move 5 from 9 to 2
move 7 from 9 to 6
move 16 from 1 to 7
move 4 from 6 to 1
move 1 from 2 to 6
move 2 from 2 to 6
move 1 from 2 to 4
move 4 from 9 to 3
move 1 from 2 to 8
move 5 from 7 to 5
move 2 from 9 to 3
move 1 from 5 to 9
move 7 from 3 to 4
move 1 from 9 to 7
move 8 from 1 to 9
move 1 from 8 to 9
move 3 from 6 to 9
move 17 from 7 to 5
move 3 from 4 to 8
move 3 from 4 to 2
move 3 from 8 to 3
move 3 from 3 to 7
move 7 from 9 to 3
move 6 from 5 to 9
move 4 from 9 to 3
move 10 from 7 to 2
move 15 from 5 to 2
move 4 from 6 to 3
move 1 from 3 to 2
move 23 from 2 to 5
move 2 from 4 to 6
move 2 from 6 to 7
move 1 from 7 to 2
move 1 from 6 to 9
move 5 from 9 to 8
move 3 from 8 to 7
move 5 from 2 to 6
move 2 from 2 to 3
move 2 from 6 to 3
move 3 from 6 to 2
move 3 from 6 to 8
move 10 from 5 to 9
move 2 from 7 to 5
move 1 from 5 to 8
move 13 from 9 to 5
move 6 from 5 to 6
move 1 from 6 to 1
move 1 from 7 to 3
move 1 from 7 to 3
move 13 from 5 to 6
move 3 from 3 to 5
move 1 from 2 to 1
move 4 from 8 to 9
move 2 from 2 to 6
move 2 from 5 to 3
move 2 from 3 to 6
move 5 from 6 to 4
move 9 from 5 to 9
move 10 from 6 to 9
move 1 from 1 to 7
move 3 from 3 to 9
move 1 from 8 to 1
move 3 from 6 to 3
move 1 from 7 to 6
move 1 from 8 to 7
move 2 from 6 to 1
move 2 from 6 to 4
move 3 from 4 to 6
move 2 from 1 to 4
move 10 from 9 to 6
move 6 from 4 to 9
move 17 from 9 to 1
move 4 from 9 to 5
move 19 from 1 to 7
move 4 from 5 to 6
move 1 from 9 to 3
move 5 from 3 to 4
move 5 from 4 to 8
move 17 from 6 to 9
move 17 from 9 to 2
move 1 from 6 to 1
move 1 from 1 to 2
move 1 from 8 to 3
move 2 from 3 to 2
move 5 from 7 to 1
move 1 from 7 to 3
move 5 from 2 to 9
move 4 from 8 to 2
move 2 from 7 to 8
move 3 from 9 to 3
move 7 from 3 to 9
move 2 from 8 to 7
move 8 from 2 to 9
move 5 from 9 to 6
move 4 from 3 to 9
move 11 from 2 to 3
move 2 from 6 to 5
move 1 from 9 to 4
move 10 from 7 to 3
move 3 from 1 to 8
move 2 from 6 to 7
move 15 from 3 to 8
move 2 from 3 to 2
move 2 from 1 to 3
move 14 from 9 to 6
move 1 from 4 to 9
move 14 from 6 to 3
move 5 from 7 to 2
move 2 from 9 to 2
move 1 from 5 to 3
move 1 from 5 to 8
move 12 from 3 to 7
move 13 from 7 to 8
move 1 from 6 to 7
move 5 from 2 to 6
move 1 from 6 to 2
move 1 from 7 to 6
move 4 from 6 to 8
move 31 from 8 to 7
move 15 from 7 to 8
move 7 from 7 to 5
move 4 from 2 to 3
move 1 from 6 to 2
move 3 from 5 to 8
move 9 from 7 to 4
move 2 from 2 to 9
move 4 from 5 to 6
move 13 from 3 to 9
move 3 from 3 to 5
move 13 from 9 to 1
move 1 from 3 to 2
move 2 from 6 to 5
move 1 from 3 to 4
move 2 from 6 to 5
move 1 from 9 to 1
move 6 from 8 to 9
move 5 from 5 to 2
move 2 from 9 to 8
move 2 from 1 to 6
move 1 from 9 to 4
move 12 from 8 to 4
move 2 from 6 to 9
move 11 from 4 to 3
move 9 from 4 to 2
move 4 from 9 to 7
move 2 from 5 to 6
move 8 from 3 to 4
move 2 from 3 to 9
move 2 from 8 to 9
move 4 from 4 to 9
move 2 from 6 to 7
move 1 from 3 to 7
move 2 from 9 to 1
move 5 from 4 to 2
move 9 from 1 to 8
move 1 from 4 to 9
move 4 from 9 to 3
move 1 from 3 to 6
move 4 from 8 to 7
move 1 from 3 to 6
move 4 from 1 to 7
move 1 from 3 to 8
move 1 from 1 to 8
move 2 from 6 to 7
move 2 from 9 to 1
move 1 from 4 to 5
move 1 from 1 to 5
move 11 from 8 to 4
move 12 from 2 to 8
move 1 from 9 to 8
move 2 from 4 to 5
move 1 from 1 to 8
move 5 from 2 to 1
move 1 from 3 to 2
move 9 from 7 to 3
move 6 from 7 to 5
move 1 from 3 to 4
move 1 from 5 to 1
move 4 from 2 to 5
move 4 from 4 to 1
move 2 from 7 to 3
move 3 from 4 to 1
move 6 from 3 to 7
move 9 from 8 to 7
move 3 from 8 to 7
move 11 from 5 to 9
move 2 from 4 to 8
move 5 from 8 to 7
move 1 from 9 to 8
move 12 from 9 to 5
move 1 from 4 to 5
move 5 from 1 to 8
move 6 from 8 to 3
move 1 from 3 to 8
move 3 from 7 to 9
move 4 from 7 to 6
move 3 from 1 to 3
move 3 from 1 to 6
move 1 from 8 to 1
move 7 from 6 to 2
move 3 from 1 to 8
move 7 from 3 to 4
move 3 from 4 to 1
move 1 from 4 to 2
move 3 from 1 to 2
move 1 from 7 to 6
move 1 from 8 to 5
move 9 from 5 to 3
move 1 from 6 to 9
move 11 from 3 to 6
move 1 from 4 to 1
move 1 from 3 to 4
move 8 from 6 to 9
move 1 from 3 to 1
move 1 from 9 to 1
move 2 from 6 to 2
move 5 from 5 to 7
move 5 from 9 to 3
move 2 from 8 to 5
move 1 from 1 to 2
move 1 from 9 to 1
move 15 from 7 to 4
move 1 from 1 to 6
move 1 from 6 to 9
move 3 from 9 to 3
move 1 from 3 to 5
move 5 from 5 to 3
move 9 from 2 to 9
move 5 from 4 to 1
move 1 from 6 to 7
move 7 from 9 to 3
move 1 from 4 to 7
move 1 from 9 to 6
move 1 from 6 to 5
move 2 from 1 to 4
move 3 from 9 to 3
move 1 from 5 to 6
move 7 from 4 to 3
move 1 from 9 to 3
move 16 from 3 to 1
move 9 from 1 to 3
move 5 from 4 to 2
move 1 from 6 to 9
move 12 from 1 to 9
move 3 from 2 to 9
move 5 from 7 to 3
move 2 from 4 to 8
move 2 from 7 to 2
move 12 from 3 to 5
move 6 from 2 to 9
move 12 from 3 to 1
move 2 from 8 to 6
move 1 from 6 to 1
move 6 from 5 to 8
move 5 from 3 to 2
move 2 from 5 to 8
move 8 from 1 to 8
move 13 from 9 to 7
move 4 from 7 to 5
move 4 from 1 to 4
move 8 from 5 to 6
move 1 from 1 to 6
move 4 from 7 to 3
move 1 from 3 to 1
move 1 from 1 to 9
move 4 from 9 to 5
move 3 from 3 to 7
move 12 from 8 to 7
move 2 from 4 to 3
move 2 from 6 to 9
move 4 from 8 to 2
move 2 from 3 to 9
move 2 from 4 to 7
move 3 from 5 to 7
move 2 from 9 to 7
move 3 from 6 to 1
move 4 from 6 to 7
move 1 from 5 to 4
move 1 from 9 to 3
move 12 from 2 to 5
move 4 from 9 to 7
move 11 from 5 to 1
move 1 from 6 to 5
move 1 from 1 to 4
move 10 from 1 to 2
move 2 from 5 to 1
move 1 from 3 to 5
move 7 from 2 to 5
move 8 from 7 to 8
move 2 from 2 to 8
move 3 from 9 to 4
move 5 from 4 to 3
move 1 from 5 to 7
move 3 from 7 to 1
move 3 from 5 to 8
move 1 from 2 to 5
move 12 from 7 to 6
move 4 from 1 to 3
move 2 from 5 to 6
move 7 from 3 to 7
move 14 from 6 to 4
move 1 from 5 to 6
move 3 from 1 to 3
move 4 from 3 to 2
move 2 from 5 to 8
move 11 from 7 to 4
move 7 from 4 to 5
move 1 from 3 to 4
move 1 from 5 to 6
move 14 from 8 to 7
move 11 from 7 to 3
move 2 from 2 to 6
move 1 from 2 to 3
move 5 from 5 to 4
move 4 from 6 to 4
move 8 from 7 to 8
move 3 from 7 to 3
move 1 from 2 to 1
move 5 from 8 to 2
move 4 from 4 to 3
move 1 from 2 to 9
move 1 from 1 to 9
move 3 from 2 to 1
move 1 from 5 to 4
move 3 from 8 to 1
move 1 from 7 to 4
move 4 from 3 to 9
move 1 from 8 to 7
move 2 from 9 to 1
move 6 from 3 to 4
move 28 from 4 to 7
move 15 from 7 to 8
move 3 from 3 to 8
move 1 from 2 to 9
move 2 from 3 to 2
move 7 from 1 to 4
move 10 from 4 to 5
move 10 from 5 to 6
move 3 from 8 to 2
move 1 from 1 to 7
move 1 from 4 to 7
move 1 from 9 to 6
move 9 from 6 to 7
move 1 from 2 to 4
move 1 from 9 to 5`;



var steps = procedure.split("\n");

var stacks = [];

//Initialise the stacks array
for(var x = 1;x < startingConfig[startingConfig.length-1].length;x += 4){
  
  stacks.push([]);
  
  for(var y = startingConfig.length-2;y >= 0;y --){
    
    if(startingConfig[y].charAt(x) == " "){
      y = -1;
    }else{
      stacks[stacks.length-1].push(startingConfig[y].charAt(x));
    }
    
  }
  
}

//console.log(stacks);



//Move a crate from one stack to another
function move(s1,s2){
  
  var item = stacks[s1].splice(stacks[s1].length-1, 1)[0];
  
  stacks[s2].push(item);
  
}

//Loop through the steps
for(var a = 0;a < steps.length;a ++){
  
  var step = steps[a].replace("move ","").replace(" from ",",").replace(" to ",",").split(",");
  
  //Store the number of crates to be moved
  var number = parseInt(step[0]);
  
  //Move 1 crate at a time
  for(var i = 0;i < number;i ++){
    move(parseInt(step[1])-1, parseInt(step[2])-1);
  }
  
}

//console.log(stacks);

var result = "";

for(var s = 0;s < stacks.length;s ++){
  result += stacks[s][stacks[s].length-1];
}

console.log("Result: ",result);




//Part 2

stacks = [];

//Initialise the stacks array
for(var x = 1;x < startingConfig[startingConfig.length-1].length;x += 4){
  
  stacks.push([]);
  
  for(var y = startingConfig.length-2;y >= 0;y --){
    
    if(startingConfig[y].charAt(x) == " "){
      y = -1;
    }else{
      stacks[stacks.length-1].push(startingConfig[y].charAt(x));
    }
    
  }
  
}

//Loop through the steps
for(var a = 0;a < steps.length;a ++){
  
  var step = steps[a].replace("move ","").replace(" from ",",").replace(" to ",",").split(",");
  
  var number = parseInt(step[0]);
  var from = parseInt(step[1])-1;
  var to = parseInt(step[2])-1;
  
  var items = stacks[from].splice(stacks[from].length-number, number);
  
  stacks[to] = stacks[to].concat(items);
  
}

//console.log(stacks);

var result2 = "";

for(var s = 0;s < stacks.length;s ++){
  result2 += stacks[s][stacks[s].length-1];
}

console.log("Result 2: ",result2);






