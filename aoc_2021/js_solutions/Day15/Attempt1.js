/**

Anthony Wilson - Advent of Code 2021 - Day 15

2021-12-15

**/

var input = `1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581`;

var lines = input.split("\n");

var riskLevels = [];

for(var l = 0;l < lines.length;l ++){
  
  riskLevels.push(lines[l].split(""));
  
}

for(var y = 0;y < riskLevels.length;y ++){
  for(var x = 0;x < riskLevels[y].length;x ++){
    riskLevels[y][x] = parseInt(riskLevels[y][x]);
  }
}

///console.log("riskLevels:",riskLevels);

//The variables are modified from within the recursivePathfinder() function
var safestPath = [];
var safestPathRiskLevel = 9*riskLevels.length*riskLevels[0].length;
///safestPathRiskLevel = 1077;

//Identify the safest path to travel using a recursive method
function recursivePathfinder(riskLevels, cellStates, currentRisk, x, y){
  
  ///console.log("safestPathRiskLevel:",safestPathRiskLevel);
  
  ///if(cellStates[y][x] == 1){
  ///  console.log("Error");
  ///  return;
  ///}
  
  if(currentRisk+riskLevels[y][x] > safestPathRiskLevel){
    ///console.log("terminated:",currentRisk+riskLevels[y][x],x,y);
    return;
  }
  
  //Mark this cell as visited
  cellStates[y][x] = 1;
  
  currentRisk += riskLevels[y][x];
  
  if(x == riskLevels[0].length-1 && y == riskLevels.length-1){
    
    //Update global "tracking" variables
    safestPathRiskLevel = currentRisk;
    
    console.log("Found new shortest:",safestPathRiskLevel);
    
    return;
    
  }
  
  var neighbours = [];
  
  if(x+1 < riskLevels[0].length){
    neighbours.push([x+1,y]);
  }
  if(y+1 < riskLevels.length){
    neighbours.push([x,y+1]);
  }
  if(x > 0){
    neighbours.push([x-1,y]);
  }
  if(y > 0){
    neighbours.push([x,y-1]);
  }
  
  ///console.log("neighbours:",neighbours);
  
  for(var n = 0;n < neighbours.length;n ++){
    
    if(cellStates[ neighbours[n][1] ][ neighbours[n][0] ] == 0 && currentRisk+riskLevels[ neighbours[n][1] ][ neighbours[n][0] ] < safestPathRiskLevel){
      
      recursivePathfinder(
        riskLevels,
        JSON.parse(JSON.stringify(cellStates)),
        currentRisk,
        neighbours[n][0],
        neighbours[n][1]
      );
      
    }
    
  }
  
  return;
  
}

var cellStates = [];
for(var y = 0;y < riskLevels.length;y ++){
  cellStates[y] = [];
  for(var x = 0;x < riskLevels[y].length;x ++){
    
    cellStates[y][x] = 0;
    
  }
}

recursivePathfinder(riskLevels, cellStates, 0, 0, 0);



console.log("Part 1:",safestPathRiskLevel-riskLevels[0][0]);





