/**

Anthony Wilson - Advent of Code 2021 - Day 18

2021-12-18

**/

var input = [
  [[[7,1],2],3]                                            ,
  [[1,7],7]                                                ,
  [[6,8],[[6,[3,6]],[0,5]]]                                ,
  [[[[2,1],8],[[9,4],8]],[[6,5],4]]                        ,
  [[1,[[3,8],[9,1]]],[[9,1],[[1,7],0]]]                    ,
  [[[7,4],[8,[7,6]]],[9,[[6,3],[7,8]]]]                    ,
  [[[[5,0],1],4],[[5,[6,9]],[[4,3],2]]]                    ,
  [[[3,8],8],[[[3,2],8],[9,[0,5]]]]                        ,
  [[[[5,8],[3,9]],[7,[1,4]]],[6,1]]                        ,
  [3,[[[3,3],9],[0,7]]]                                    ,
  [[[6,9],1],[[0,[8,4]],[[2,2],9]]]                        ,
  [[[[6,2],3],[0,4]],3]                                    ,
  [[[[3,8],7],[[7,4],0]],[2,[5,[2,8]]]]                    ,
  [[4,[9,[8,0]]],[[1,5],[[9,3],8]]]                        ,
  [[[8,5],[3,[1,4]]],[[6,[8,0]],[[2,7],[2,6]]]]            ,
  [4,7]                                                    ,
  [[[[2,3],0],[[1,9],[4,1]]],[[1,[4,2]],3]]                ,
  [[[8,[5,3]],[[5,7],7]],[[5,6],[6,4]]]                    ,
  [[[[2,4],1],[8,6]],[[6,5],[0,[9,1]]]]                    ,
  [[[1,[5,7]],8],[[[9,1],9],[[1,2],4]]]                    ,
  [[[[5,5],[4,0]],[4,[9,6]]],[[[2,1],1],7]]                ,
  [[[[1,9],[9,5]],[[5,0],[3,1]]],[[[6,7],[8,8]],[[7,3],0]]],
  [[6,[[6,7],[9,0]]],[[7,7],[[0,3],0]]]                    ,
  [[0,6],[5,2]]                                            ,
  [[[[5,8],3],[[9,0],8]],[7,4]]                            ,
  [[0,[[9,9],[9,4]]],[[[1,1],2],[1,[6,7]]]]                ,
  [0,[[5,7],2]]                                            ,
  [[2,[[5,4],6]],[1,[8,[7,6]]]]                            ,
  [[[1,7],[8,[5,8]]],[[[2,1],[9,1]],[[5,6],9]]]            ,
  [[1,8],[9,[4,3]]]                                        ,
  [5,[2,[[5,5],9]]]                                        ,
  [3,[8,[[2,8],[4,8]]]]                                    ,
  [[[4,9],[[5,5],0]],[9,[8,[3,0]]]]                        ,
  [[[2,[6,4]],[8,[9,9]]],[[[0,4],8],[3,[9,7]]]]            ,
  [[[[8,1],[2,4]],3],[1,[[3,3],[6,3]]]]                    ,
  [[[8,[7,3]],[1,8]],2]                                    ,
  [[8,[8,4]],[[6,[4,7]],[3,0]]]                            ,
  [[[[4,6],[8,3]],9],[9,[[8,9],[0,9]]]]                    ,
  [[3,[[2,7],[4,4]]],2]                                    ,
  [8,[[[8,6],2],[[8,9],6]]]                                ,
  [[[[5,7],[2,0]],[[0,2],[5,5]]],[[[8,5],5],[[1,3],[2,3]]]],
  [[1,6],[[9,8],[9,[4,9]]]]                                ,
  [[[[1,4],5],9],[4,[6,8]]]                                ,
  [[[[6,4],[9,0]],[[1,4],[6,6]]],[[9,[2,8]],2]]            ,
  [[[[5,9],2],[[0,0],5]],[2,1]]                            ,
  [6,[[3,2],[[3,0],0]]]                                    ,
  [[[[7,4],1],[[4,1],1]],[[3,4],4]]                        ,
  [3,[9,[9,7]]]                                            ,
  [[[3,[3,3]],[0,3]],[1,[1,8]]]                            ,
  [[8,[8,7]],[[9,2],5]]                                    ,
  [[[1,[3,9]],[5,9]],[1,5]]                                ,
  [[[[7,8],[9,7]],9],[[[9,2],[2,2]],[[9,6],8]]]            ,
  [4,[[3,5],[[1,3],[5,5]]]]                                ,
  [7,[[[0,1],2],[[3,6],5]]]                                ,
  [0,[[[2,4],[3,4]],[8,9]]]                                ,
  [[1,[[6,8],1]],[8,0]]                                    ,
  [1,1]                                                    ,
  [7,0]                                                    ,
  [[1,2],[[0,[8,3]],[[4,5],[9,7]]]]                        ,
  [[[[2,3],[5,9]],[7,[1,9]]],2]                            ,
  [[3,5],[[9,7],9]]                                        ,
  [[[[6,9],[4,8]],6],0]                                    ,
  [[[[2,4],[3,9]],[2,[9,4]]],[[[8,9],[3,1]],7]]            ,
  [[5,[[0,2],4]],[[[9,9],[7,4]],[1,5]]]                    ,
  [3,[6,[[5,4],1]]]                                        ,
  [[[2,[2,7]],2],[[4,[7,3]],5]]                            ,
  [7,[[0,[2,0]],[[9,4],6]]]                                ,
  [[4,[3,[6,2]]],9]                                        ,
  [[[0,[5,6]],[8,3]],[[7,9],[0,[9,6]]]]                    ,
  [8,[[6,4],[4,8]]]                                        ,
  [[[8,[6,8]],[5,[7,3]]],[[[7,8],5],2]]                    ,
  [[[[3,5],[4,7]],5],[[0,0],[9,[1,9]]]]                    ,
  [[7,[[1,5],9]],[[[3,4],[1,7]],[1,[7,9]]]]                ,
  [[0,[3,[4,1]]],[[[2,9],3],[4,[0,8]]]]                    ,
  [[[8,[1,6]],[[0,1],7]],[[[1,1],[0,2]],[[9,4],[9,6]]]]    ,
  [[[[6,7],0],[[6,8],9]],[[1,[6,6]],[[2,9],[4,7]]]]        ,
  [[[[5,0],[1,2]],[1,[5,1]]],[[0,4],1]]                    ,
  [[9,1],6]                                                ,
  [[7,2],[[[5,5],[4,3]],6]]                                ,
  [[9,[[0,6],9]],[[7,9],[7,1]]]                            ,
  [[[[7,3],[6,4]],[[2,5],[7,2]]],[[[4,4],0],[[9,5],[8,5]]]],
  [[[[8,8],[6,4]],[[0,2],[9,5]]],2]                        ,
  [[[[3,0],7],[9,2]],[[0,[8,6]],[[7,2],[8,5]]]]            ,
  [[0,6],[1,[9,[4,3]]]]                                    ,
  [[0,8],[[[5,0],6],[5,[2,0]]]]                            ,
  [[[[7,1],[0,3]],[[9,9],[3,5]]],[4,[8,4]]]                ,
  [7,[[1,[3,7]],[[3,4],[2,3]]]]                            ,
  [[[[2,2],[4,8]],[[3,4],0]],[[[1,5],[2,8]],5]]            ,
  [6,[[[9,1],5],[9,9]]]                                    ,
  [[[2,[8,6]],[[9,9],[6,3]]],4]                            ,
  [[[[3,2],[9,3]],8],9]                                    ,
  [[[[6,9],0],[[0,6],[1,3]]],[[5,[9,8]],[[1,5],[3,7]]]]    ,
  [[2,[4,[2,3]]],[[[6,0],[7,2]],3]]                        ,
  [[[[8,3],4],[6,[8,8]]],4]                                ,
  [[[9,8],5],[[[4,4],[6,3]],[8,6]]]                        ,
  [9,2]                                                    ,
  [[[3,4],[4,[7,0]]],[0,[4,[6,9]]]]                        ,
  [[[0,8],[3,9]],[[[3,8],6],[[9,3],6]]]                    ,
  [[[[5,6],[0,3]],1],[8,[2,9]]]                            ,
  [[[[4,2],8],[[9,3],7]],0]
];

/*var input = [
  [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]],
  [[[5,[2,8]],4],[5,[[9,9],0]]],
  [6,[[[6,2],[5,6]],[[7,6],[4,7]]]],
  [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]],
  [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]],
  [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]],
  [[[[5,4],[7,7]],8],[[8,3],8]],
  [[9,3],[[9,9],[6,[4,9]]]],
  [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]],
  [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
];*/

class Snailnum{
  
  constructor(nestedArray){
    
    this.tree = JSON.parse(JSON.stringify(nestedArray));
    this.values = this.getValuesFromTreeRecursive( JSON.parse(JSON.stringify(nestedArray)) );
    
    this.createReferenceTreeRecursive(0, this.tree);
    
  }
  
  createReferenceTreeRecursive(count,treePortion){
    
    if((typeof treePortion[0]) == "object"){
      
      count = this.createReferenceTreeRecursive(count,treePortion[0]);
      
    }else{
      
      treePortion[0] = count;
      count ++;
      
    }
    
    if((typeof treePortion[1]) == "object"){
      
      count = this.createReferenceTreeRecursive(count,treePortion[1]);
      
    }else{
      
      treePortion[1] = count;
      count ++;
      
    }
    
    return count;
    
  }
  
  getValuesFromTreeRecursive(tree){
    
    var output = [];
    
    if((typeof tree[0]) == "object"){
      output = output.concat(this.getValuesFromTreeRecursive(tree[0]));
    }else{
      output.push(tree[0]);
    }
    if((typeof tree[1]) == "object"){
      output = output.concat(this.getValuesFromTreeRecursive(tree[1]));
    }else{
      output.push(tree[1]);
    }
    
    return output;
    
  }
  
  add(snum){
    
    this.values = this.values.concat(snum.values);
    
    this.tree = JSON.parse(JSON.stringify( [this.tree, snum.tree] ));
    
    this.createReferenceTreeRecursive(0, this.tree);
    
  }
  
  reduce(){
    
    var reduced = false;
    
    while(!reduced){
      
      var changed = false;
      
      if( this.explodeRecursive(this.tree,0) ){
        
        changed = true;
        
      }else{
        
        changed = this.splitRecursive(this.tree);
        
      }
      
      reduced = !changed;
      
    }
    
  }
  
  explodeRecursive(treePortion, level){
    
    if((typeof treePortion) == "object"){
      
      if(level == 3){
        
        if((typeof treePortion[0]) == "object"){
          
          //treePortion[0] needs to "explode"
          
          if((typeof treePortion[0][0]) == "object" || (typeof treePortion[0][1]) == "object"){
            console.log("ERROR",treePortion[0]);
          }
          
          if(treePortion[0][0] > 0){
            this.values[treePortion[0][0]-1] += this.values[treePortion[0][0]];
          }
          if(treePortion[0][1]+1 < this.values.length){
            this.values[treePortion[0][1]+1] += this.values[treePortion[0][1]];
          }
          
          this.values.splice(treePortion[0][1],1);
          this.values[treePortion[0][0]] = 0;
          treePortion[0] = undefined;
          
          //The tree needs to be updated after modifications
          this.createReferenceTreeRecursive(0, this.tree);
          
          return true;
          
        }else if((typeof treePortion[1]) == "object"){
          
          //treePortion[1] needs to "explode"
          
          if((typeof treePortion[1][0]) == "object" || (typeof treePortion[1][0]) == "object"){
            console.log("ERROR",treePortion[1]);
          }
          
          if(treePortion[1][0] > 0){
            this.values[treePortion[1][0]-1] += this.values[treePortion[1][0]];
          }
          if(treePortion[1][1]+1 < this.values.length){
            this.values[treePortion[1][1]+1] += this.values[treePortion[1][1]];
          }
          
          this.values.splice(treePortion[1][1],1);
          this.values[treePortion[1][0]] = 0;
          treePortion[1] = undefined;
          
          //The tree needs to be updated after modifications
          this.createReferenceTreeRecursive(0, this.tree);
          
          return true;
          
        }
        
        return false;
        
      }else{
        
        if(this.explodeRecursive(treePortion[0], level+1)){
          return true;
        }
        
        return this.explodeRecursive(treePortion[1], level+1);
        
      }
      
    }
    
    return false;
    
  }
  
  splitRecursive(treePortion){
    
    if((typeof treePortion[0]) == "object"){
      
      if(this.splitRecursive(treePortion[0])){
        return true;
      }
      
    }else{
      
      if(this.values[treePortion[0]] >= 10){
        
        //This value needs to be split
        var pos = treePortion[0];
        
        this.values.splice( pos+1, 0, Math.ceil(this.values[pos]/2) );
        this.values[pos] = Math.floor(this.values[pos]/2);
        
        treePortion[0] = [pos,pos+1];
        
        //The tree needs to be updated after modifications
        this.createReferenceTreeRecursive(0, this.tree);
        
        return true;
        
      }
      
    }
    
    if((typeof treePortion[1]) == "object"){
      
      if(this.splitRecursive(treePortion[1])){
        return true;
      }
      
      return false;
      
    }else{
      
      if(this.values[treePortion[1]] >= 10){
        
        //This value needs to be split
        var pos = treePortion[1];
        
        this.values.splice( pos+1, 0, Math.ceil(this.values[pos]/2) );
        this.values[pos] = Math.floor(this.values[pos]/2);
        
        treePortion[1] = [pos,pos+1];
        
        //The tree needs to be updated after modifications
        this.createReferenceTreeRecursive(0, this.tree);
        
        return true;
        
      }
      
      return false;
      
    }
    
    return false;
    
  }
  
  getMagnitude(){
    return this.calculateMagnitudeRecursive(this.tree);
  }
  
  calculateMagnitudeRecursive(tree){
    
    var m = 0;
    
    if((typeof tree[0]) == "object"){
      m += 3*this.calculateMagnitudeRecursive(tree[0]);
    }else{
      m += 3*this.values[tree[0]];
    }
    if((typeof tree[1]) == "object"){
      m += 2*this.calculateMagnitudeRecursive(tree[1]);
    }else{
      m += 2*this.values[tree[1]];
    }
    
    return m;
    
  }
  
  toString(treePortion){
    
    if(treePortion == undefined){
      treePortion = this.tree;
    }
    
    var output = "[";
    
    if((typeof treePortion[0]) == "object"){
      output += this.toString(treePortion[0]);
    }else{
      output += this.values[treePortion[0]];
    }
    
    output += ",";
    
    if((typeof treePortion[1]) == "object"){
      output += this.toString(treePortion[1]);
    }else{
      output += this.values[treePortion[1]];
    }
    
    output += "]";
    
    return output;
    
  }
  
}

var result = new Snailnum(input[0]);

for(var n = 1;n < input.length;n ++){
  
  var next = new Snailnum(input[n]);
  
  result.add(next);
  result.reduce();
  
  ///console.log("Reduced:",result.toString());
  
}

///console.log("result:",result.toString());

console.log("Part 1:",result.getMagnitude());


////////////////////////////////////////////////////////////////////////////////////////////////////


var largestMagnitude = 0;

for(var a = 0;a < input.length;a ++){
  
  for(var b = 0;b < input.length;b ++){
    
    var num1 = new Snailnum(input[a]);
    num1.add(new Snailnum(input[b]));
    num1.reduce();
    var mag1 = num1.getMagnitude();
    
    if(mag1 > largestMagnitude){
      largestMagnitude = mag1;
    }
    
    
    var num2 = new Snailnum(input[b]);
    num2.add(new Snailnum(input[a]));
    num2.reduce();
    var mag2 = num2.getMagnitude();
    
    if(mag2 > largestMagnitude){
      largestMagnitude = mag2;
    }
    
  }
  
}

console.log("Part 2:",largestMagnitude);




