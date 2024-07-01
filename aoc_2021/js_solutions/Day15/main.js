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

//Process the input data for easy use
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

//Input data processed.

//console.log("riskLevels:",riskLevels);


var posX = 0;
var posY = 0;

var totalRisk = 0;

var navigate = (x, y, risk) => {
  
  
  
}


///Implement A*, BFS, or Dijkstra’s Algorithm



console.log("Part 1:");





