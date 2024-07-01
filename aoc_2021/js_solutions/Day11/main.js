/**

Anthony Wilson - Advent of Code 2021 - Day 11

2021-12-11

**/

var input = `7232374314
8531113786
3411787828
5482241344
5856827742
7614532764
5311321758
1255116187
5821277714
2623834788`;

var lines = input.split("\n");

var energyLevels = [];

//Define the initial energy levels
for(var l = 0;l < lines.length;l ++){
  energyLevels[l] = lines[l].split("");
}
for(var y = 0;y < energyLevels.length;y ++){
  for(var x = 0;x < energyLevels[y].length;x ++){
    energyLevels[y][x] = parseInt(energyLevels[y][x]);
  }
}

//Simulate a single substep
//Returns how many octopi flashed
function simulateStep(){
  
  var flashCount = 0;
  
  for(var y = 0;y < energyLevels.length;y ++){
    for(var x = 0;x < energyLevels[y].length;x ++){
      
      //This octopus flashes
      if(energyLevels[y][x] > 9){
        
        flashCount ++;
        
        energyLevels[y][x] = -10;
        
        if(y > 0){
          //Directly above
          energyLevels[y-1][x] ++;
          
          //Above/left
          if(x > 0){
            energyLevels[y-1][x-1] ++;
          }
          
          //Above/right
          if(x+1 < energyLevels[y].length){
            energyLevels[y-1][x+1] ++;
          }
        }
        
        if(y+1 < energyLevels.length){
          //Directly below
          energyLevels[y+1][x] ++;
          
          //Below/left
          if(x > 0){
            energyLevels[y+1][x-1] ++;
          }
          
          //Below/right
          if(x+1 < energyLevels[y].length){
            energyLevels[y+1][x+1] ++;
          }
        }
        
        if(x > 0){
          energyLevels[y][x-1] ++;
        }
        
        if(x+1 < energyLevels[y].length){
          energyLevels[y][x+1] ++;
        }
        
      }
      
    }
  }
  
  return flashCount;
  
}

var totalFlashes = 0;

//Simulate 100 steps
for(var i = 0;i < 100;i ++){
  
  //Start by incrementing the energy level of every octopus
  for(var y = 0;y < energyLevels.length;y ++){
    for(var x = 0;x < energyLevels[y].length;x ++){
      
      energyLevels[y][x] ++;
      
    }
  }
  
  
  var stepResult = simulateStep();
  
  while(stepResult > 0){
    
    stepResult = simulateStep();
    
  }
  
  
  var flashesThisStep = 0;
  
  for(var y = 0;y < energyLevels.length;y ++){
    for(var x = 0;x < energyLevels[y].length;x ++){
      
      if(energyLevels[y][x] <= 0){
        
        energyLevels[y][x] = 0;
        
        flashesThisStep ++;
        
      }
      
    }
  }
  
  totalFlashes += flashesThisStep;
  
}

///console.log("energyLevels:",energyLevels);

console.log("Part 1:",totalFlashes);


///////////////////////////////////////////////////////


//Reset energy levels for part 2
energyLevels = [];
for(var l = 0;l < lines.length;l ++){
  energyLevels[l] = lines[l].split("");
}
for(var y = 0;y < energyLevels.length;y ++){
  for(var x = 0;x < energyLevels[y].length;x ++){
    energyLevels[y][x] = parseInt(energyLevels[y][x]);
  }
}

var allFlashedAtOnce = false;

var maxFlashes = energyLevels.length*energyLevels[0].length;

var step = 0;

while(!allFlashedAtOnce){
  
  //Start by incrementing the energy level of every octopus
  for(var y = 0;y < energyLevels.length;y ++){
    for(var x = 0;x < energyLevels[y].length;x ++){
      
      energyLevels[y][x] ++;
      
    }
  }
  
  
  var stepResult = simulateStep();
  
  while(stepResult > 0){
    
    stepResult = simulateStep();
    
  }
  
  
  var flashesThisStep = 0;
  
  for(var y = 0;y < energyLevels.length;y ++){
    for(var x = 0;x < energyLevels[y].length;x ++){
      
      if(energyLevels[y][x] <= 0){
        
        energyLevels[y][x] = 0;
        
        flashesThisStep ++;
        
      }
      
    }
  }
  
  step ++;
  
  if(flashesThisStep == maxFlashes){
    
    allFlashedAtOnce = true;
    
    console.log("Part 2:",step);
    
  }
  
}





