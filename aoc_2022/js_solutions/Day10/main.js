/**

Anthony Wilson - Advent of Code 2022 - Day 10

2022-12-10

**/

var input = `noop
addx 5
noop
noop
noop
addx 1
addx 2
addx 5
addx 2
addx 5
noop
noop
noop
noop
noop
addx -12
addx 18
addx -1
noop
addx 3
addx 5
addx -5
addx 7
noop
addx -36
addx 18
addx -16
noop
noop
noop
addx 5
addx 2
addx 5
addx 2
addx 13
addx -6
addx -4
addx 5
addx 2
addx 4
addx -3
addx 2
noop
addx 3
addx 2
addx 5
addx -40
addx 25
addx -22
addx 25
addx -21
addx 5
addx 3
noop
addx 2
addx 19
addx -10
addx -4
noop
addx -4
addx 7
noop
addx 3
addx 2
addx 5
addx 2
addx -26
addx 27
addx -36
noop
noop
noop
noop
addx 4
addx 6
noop
addx 12
addx -11
addx 2
noop
noop
noop
addx 5
addx 5
addx 2
noop
noop
addx 1
addx 2
addx 5
addx 2
addx 1
noop
noop
addx -38
noop
addx 9
addx -4
noop
noop
addx 7
addx 10
addx -9
addx 2
noop
addx -9
addx 14
addx 5
addx 2
addx -24
addx 25
addx 2
addx 5
addx 2
addx -30
addx 31
addx -38
addx 7
noop
noop
noop
addx 1
addx 21
addx -16
addx 8
addx -4
addx 2
addx 3
noop
noop
addx 5
addx -2
addx 5
addx 3
addx -1
addx -1
addx 4
addx 5
addx -38
noop`;



var lines = input.split("\n");

var X = 1;
var cycle = 0;

var result = 0;

var output = "";

function print(){
  
  var x = cycle%40;
  
  if(x == 0){
    output += "\n";
  }
  
  if(x-1 <= X && x+1 >= X){
    output += "#";
  }else{
    output += ".";
  }
  
}

//Loop through cycles
for(var l = 0;l < lines.length;l ++){
  
  print();
  cycle ++;
  
  var instruct = lines[l].split(" ");
  
  var checked = false;
  
  //Check
  if((cycle+20)%40 == 0){
    result += cycle*X;
    checked = true;
  }
  
  if(instruct[0] == "addx"){
    print();
    cycle ++;
  }
  
  //Check again
  if((cycle+20)%40 == 0 && !checked){
    result += cycle*X;
  }
  
  if(instruct[0] == "addx"){
    X += parseInt(instruct[1]);
  }
  
}

console.log("Result: ",result);

console.log(output);









