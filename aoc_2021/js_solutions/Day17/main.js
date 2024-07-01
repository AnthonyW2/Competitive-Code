/**

Anthony Wilson - Advent of Code 2021 - Day 17

2021-12-17

**/

var input = `target area: x=253..280, y=-73..-46`;
///var input = `target area: x=20..30, y=-10..-5`;

input = input.substring(15);

var inputParts = input.split(", ");

var targetBounds = [
  inputParts[0].split(".."), //x-min, x-max
  inputParts[1].split("..")  //y-min, y-max
];
targetBounds[0][0] = parseInt(targetBounds[0][0]);
targetBounds[0][1] = parseInt(targetBounds[0][1]);
targetBounds[1][0] = parseInt(targetBounds[1][0].substring(2));
targetBounds[1][1] = parseInt(targetBounds[1][1]);

console.log("targetBounds:",targetBounds);

function simulateStep(x,y,velX,velY){
  
  x += velX;
  y += velY;
  
  velX += (velX == 0 ? 0 : (velX < 0 ? 1 : -1));
  velY --;
  
  return [x,y,velX,velY];
  
}

function evaluateGivenVelocity(startXVel,startYVel,targetBounds){
  
  var x = 0;
  var y = 0;
  var velX = startXVel;
  var velY = startYVel;
  
  while(true){
    
    ///var stepResult = simulateStep(x,y,velX,velY);
    ///
    ///x = stepResult[0];
    ///y = stepResult[1];
    ///velX = stepResult[2];
    ///velY = stepResult[3];
    
    x += velX;
    y += velY;
    
    velX += (velX == 0 ? 0 : (velX < 0 ? 1 : -1));
    velY --;
    
    ///console.log(x,y,velX,velY);
    
    if(x > targetBounds[0][1]){
      //Overshot horizontal
      return false;
    }
    if(y < targetBounds[1][0]){
      //Overshot vertical
      return false;
    }
    
    if(x >= targetBounds[0][0] && y <= targetBounds[1][1]){
      return true;
    }
    
    if(velX == 0 && x < targetBounds[0][0]){
      //Won't make it far enough
      return false;
    }
    
  }
  
}

function evaluateXVelocity(startXVel,targetBounds){
  
  var x = 0;
  var velX = startXVel;
  
  while(true){
    
    var stepResult = simulateStep(x,0,velX,0);
    
    x = stepResult[0];
    velX = stepResult[2];
    
    if(x > targetBounds[0][1]){
      return 0;
    }
    
    if(x > targetBounds[0][0]){
      
      if(velX == 0){
        //Return a special result if the horizontal velocity ends at 0
        return 2;
      }
      
      return 1;
    }
    
    if(velX == 0){
      return 0;
    }
    
  }
  
}

function evaluateYVelocity(startYVel,targetBounds){
  
  var y = 0;
  var velY = startYVel;
  
  var maxHeight = 0;
  
  while(true){
    
    y += velY;
    velY --;
    
    if(y > maxHeight){
      maxHeight = y;
    }
    
    if(velY == -10000){
      return 0;
    }
    
    if(y < targetBounds[1][0]){
      return 0;
    }
    
    if(y < targetBounds[1][1]){
      return maxHeight;
    }
    
  }
  
}

///console.log("test 1:",evaluateGivenVelocity(9,0,targetBounds));   // true
///console.log("test 2:",evaluateGivenVelocity(6,3,targetBounds));   // true
///console.log("test 3:",evaluateGivenVelocity(17,-4,targetBounds)); // false
///console.log("test 4:",evaluateGivenVelocity(7,2,targetBounds));   // true
///console.log("test 5:",evaluateGivenVelocity(6,9,targetBounds));   // true

var xVelocity = 0;

while(
  evaluateXVelocity(xVelocity,targetBounds) != 2 &&
  xVelocity < targetBounds[0][0]
){
  xVelocity ++;
}

///console.log("xVelocity:",xVelocity);

var maxHeight = 0;
var maxVel = 0; //For part 2

for(var v = 0;v < 10000;v ++){
  
  var height = evaluateYVelocity(v,targetBounds);
  
  if(height > 0){
    ///console.log(v,height);
    
    if(height > maxHeight){
      maxHeight = height;
    }
    maxVel = v;
  }
  
}

console.log("Part 1:",maxHeight);


////////////////////////////////////////////////////////////////////////////////////////////////////


var successfulVelocities = [];

///console.log("maxVel:",maxVel);

for(var xv = 0;xv <= targetBounds[0][1];xv ++){
  
  ///console.log("xv:",xv);
  
  for(var yv = targetBounds[1][0];yv <= maxVel;yv ++){
    
    if(evaluateGivenVelocity(xv,yv,targetBounds)){
      
      successfulVelocities.push([xv,yv]);
      
    }
    
  }
  
}

///console.log(successfulVelocities);

console.log("Part 2:",successfulVelocities.length);





