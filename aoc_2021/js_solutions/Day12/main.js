/**

Anthony Wilson - Advent of Code 2021 - Day 12

2021-12-12

**/

var input = `end-MY
MY-xc
ho-NF
start-ho
NF-xc
NF-yf
end-yf
xc-TP
MY-qo
yf-TP
dc-NF
dc-xc
start-dc
yf-MY
MY-ho
EM-uh
xc-yf
ho-dc
uh-NF
yf-ho
end-uh
start-NF`;

var lines = input.split("\n");

//Store a list of unique nodes
var nodes = [];

for(var l = 0;l < lines.length;l ++){
  
  var line = lines[l].split("-");
  
  var cave0 = line[0];
  var cave1 = line[1];
  
  var cave0Listed = false;
  var cave1Listed = false;
  
  for(var n = 0;n < nodes.length;n ++){
    
    if(nodes[n] == cave0){
      cave0Listed = true;
    }
    
    if(nodes[n] == cave1){
      cave1Listed = true;
    }
    
  }
  
  if(!cave0Listed){
    nodes.push(cave0);
  }
  if(!cave1Listed){
    nodes.push(cave1);
  }
  
}

//Store the graph sructure as an array of sub-arrays
var graph = [];

//0 = small cave, 1 = large cave
var nodeTypes = [];

for(var n = 0;n < nodes.length;n ++){
  
  graph[n] = [];
  
  for(var l = 0;l < lines.length;l ++){
    
    var line = lines[l].split("-");
    
    var cave0 = line[0];
    var cave1 = line[1];
    
    var cave0Listed = false;
    var cave1Listed = false;
    
    if(cave0 == nodes[n]){
      
      //Identify the "ID" of cave1
      for(var m = 0;m < nodes.length;m ++){
        if(nodes[m] == cave1){
          //Add the ID to the graph
          graph[n].push(m);
          m = nodes.length;
        }
      }
      
    }else if(cave1 == nodes[n]){
      
      //Identify the "ID" of cave0
      for(var m = 0;m < nodes.length;m ++){
        if(nodes[m] == cave0){
          //Add the ID to the graph
          graph[n].push(m);
          m = nodes.length;
        }
      }
      
    }
    
  }
  
  //Detect uppercase/lowercase
  if(nodes[n].toLowerCase() == nodes[n]){
    nodeTypes[n] = 0;
  }else{
    nodeTypes[n] = 1;
  }
  
}

///console.log("nodes:",nodes);
///
///console.log("node types:",nodeTypes);
///
///console.log("graph:",graph);

var paths = [];

//Use an algorithm similar to depth-first search to list all the paths from the start to the end

function RecursiveSearch(graph, nodeTypes, end, nodeStates, node, path){
  
  ///console.log("node:",node);
  ///console.log("nodeStates:",nodeStates);
  ///console.log("path:",path);
  ///console.log("nodeTypes:",nodeTypes);
  
  //Add the node to the current path
  path.push(node);
  
  //If the funcion reaches the end, return it and end this branch
  if(node == end){
    ///console.log("Found end",path);
    return [path];
  }
  
  //Mark the node as "visited", only if it is a small cave
  if(nodeTypes[node] == 0){
    nodeStates[node] = 1;
  }
  
  var neighbours = graph[node];
  
  var output = [];
  
  for(var n = 0;n < neighbours.length;n ++){
    
    var neighbour = neighbours[n];
    
    if(nodeStates[neighbour] == 0){
      
      output = output.concat(RecursiveSearch(
        graph,
        nodeTypes,
        end,
        JSON.parse(JSON.stringify(nodeStates)),
        neighbour,
        JSON.parse(JSON.stringify(path))
      ));
      
    }
    
  }
  
  ///console.log("Returning output",output);
  return output;
  
}

var start;
var end;
for(var n = 0;n < nodes.length;n ++){
  if(nodes[n] == "start"){
    start = n;
  }else if(nodes[n] == "end"){
    end = n;
  }
}

///console.log("start:",start,"end:",end);

var inputNodeStates = new Array(graph.length);
for(var n = 0;n < graph.length;n ++){
  inputNodeStates[n] = 0;
}

var paths = RecursiveSearch(graph, nodeTypes, end, inputNodeStates, start, []);

///console.log("paths:",paths);

console.log("Part 1:",paths.length);


/////////////////////////////////////////////////////////////////////////////////


function RecursiveSearchPart2(graph, nodeTypes, start, end, nodeStates, node, path, smallCaveVisitedTwice){
  
  ///console.log("node:",node);
  ///console.log("nodeStates:",nodeStates);
  ///console.log("path:",path);
  ///console.log("nodeTypes:",nodeTypes);
  
  //Add the node to the current path
  path.push(node);
  
  //If the funcion reaches the end, return it and end this branch
  if(node == end){
    ///console.log("Found end",path);
    return [path];
  }
  
  //Mark the node as "visited", only if it is a small cave
  if(nodeTypes[node] == 0){
    nodeStates[node] ++;
  }
  
  var neighbours = graph[node];
  
  var output = [];
  
  for(var n = 0;n < neighbours.length;n ++){
    
    var neighbour = neighbours[n];
    
    if(!smallCaveVisitedTwice){
      
      if(nodeStates[neighbour] == 1 && neighbour != start){
        
        output = output.concat(RecursiveSearchPart2(
          graph,
          nodeTypes,
          start,
          end,
          JSON.parse(JSON.stringify(nodeStates)),
          neighbour,
          JSON.parse(JSON.stringify(path)),
          true
        ));
        
      }
      
    }
    
    if(nodeStates[neighbour] == 0){
      
      output = output.concat(RecursiveSearchPart2(
        graph,
        nodeTypes,
        start,
        end,
        JSON.parse(JSON.stringify(nodeStates)),
        neighbour,
        JSON.parse(JSON.stringify(path)),
        smallCaveVisitedTwice
      ));
      
    }
    
  }
  
  ///console.log("Returning output",output);
  return output;
  
}

inputNodeStates = new Array(graph.length);
for(var n = 0;n < graph.length;n ++){
  inputNodeStates[n] = 0;
}

var paths2 = RecursiveSearchPart2(graph, nodeTypes, start, end, inputNodeStates, start, [], false);

console.log("Part 2:",paths2.length);











