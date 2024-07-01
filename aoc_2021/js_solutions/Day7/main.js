/**

Anthony Wilson - Advent of Code 2021 - Day 7

2021-12-7

**/

var input = `16,1,2,0,4,2,7,1,2,14`;

var nums = input.split(",");

var largest = 0;

for(var n = 0;n < nums.length;n ++){
  
  nums[n] = parseInt(nums[n]);
  
  if(nums[n] > largest){
    largest = nums[n];
  }
  
}

console.log(largest);

var smallestSum = largest*nums.length;

for(var a = 0;a < largest;a ++){
  
  var sumOfFuel = 0;
  
  for(var n = 0;n < nums.length;n ++){
    
    sumOfFuel += Math.abs(nums[n] - a);
    
  }
  
  if(sumOfFuel < smallestSum){
    smallestSum = sumOfFuel;
  }
  
}

console.log("Part 1:",smallestSum);


///////////////////////////////////////////////////////////////


var smallestSum2 = 0;

for(var a = 0;a < largest;a ++){
  
  var sumOfFuel = 0;
  
  for(var n = 0;n < nums.length;n ++){
    
    var dist = Math.abs(nums[n] - a)
    
    sumOfFuel += dist*(dist+1)/2;
    
  }
  
  if(smallestSum2 == 0){
    smallestSum2 = sumOfFuel;
  }else if(sumOfFuel < smallestSum2){
    smallestSum2 = sumOfFuel;
  }
  
}

console.log("Part 2:",smallestSum2);






