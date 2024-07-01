/**

Anthony Wilson - Advent of Code 2021 - Day 6

2021-12-6

**/

var input = `1,1,3,5,1,3,2,1,5,3,1,4,4,4,1,1,1,3,1,4,3,1,2,2,2,4,1,1,5,5,4,3,1,1,1,1,1,1,3,4,1,2,2,5,1,3,5,1,3,2,5,2,2,4,1,1,1,4,3,3,3,1,1,1,1,3,1,3,3,4,4,1,1,5,4,2,2,5,4,5,2,5,1,4,2,1,5,5,5,4,3,1,1,4,1,1,3,1,3,4,1,1,2,4,2,1,1,2,3,1,1,1,4,1,3,5,5,5,5,1,2,2,1,3,1,2,5,1,4,4,5,5,4,1,1,3,3,1,5,1,1,4,1,3,3,2,4,2,4,1,5,5,1,2,5,1,5,4,3,1,1,1,5,4,1,1,4,1,2,3,1,3,5,1,1,1,2,4,5,5,5,4,1,4,1,4,1,1,1,1,1,5,2,1,1,1,1,2,3,1,4,5,5,2,4,1,5,1,3,1,4,1,1,1,4,2,3,2,3,1,5,2,1,1,4,2,1,1,5,1,4,1,1,5,5,4,3,5,1,4,3,4,4,5,1,1,1,2,1,1,2,1,1,3,2,4,5,3,5,1,2,2,2,5,1,2,5,3,5,1,1,4,5,2,1,4,1,5,2,1,1,2,5,4,1,3,5,3,1,1,3,1,4,4,2,2,4,3,1,1`;

var fish = input.split(",");

for(var f = 0;f < fish.length;f ++){
  fish[f] = parseInt(fish[f]);
}

for(var a = 0;a < 80;a ++){
  
  for(var f = 0;f < fish.length;f ++){
    
    if(fish[f] == 0){
      fish[f] = 6;
      fish.push(9);
    }else{
      fish[f] --;
    }
    
  }

}

console.log("Part 1:",fish.length);


////////////////////////////////////////////////////////////////////////////


var startingFish = input.split(",");

//            0,1,2,3,4,5,6,7,8
var timers = [0,0,0,0,0,0,0,0,0];

//Count the initial timers
for(var f = 0;f < startingFish.length;f ++){
  timers[parseInt(startingFish[f])] ++;
}

//console.log(timers);

for(var a = 0;a < 256;a ++){
  
  var reprodNum = timers[0];
  
  //Shift everything in the timers array to the left by 1
  for(var t = 0;t < 8;t ++){
    
    timers[t] = timers[t+1];
    
  }
  timers[8] = 0;
  
  timers[6] += reprodNum;
  timers[8] += reprodNum;
  
}

var total = 0;
for(var t = 0;t <= 8;t ++){
  total += timers[t];
}

console.log("Part 2:",total);





