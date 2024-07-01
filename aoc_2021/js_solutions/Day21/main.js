/**

Anthony Wilson - Advent of Code 2021 - Day 21

2021-12-21

**/

var input = `Player 1 starting position: 7
Player 2 starting position: 6`;
/// var input = `Player 1 starting position: 4
/// Player 2 starting position: 8`; //Example input

var lines = input.split("\n");
lines[0] = lines[0].split(" ");
lines[1] = lines[1].split(" ");

var p1start = parseInt(lines[0][lines[0].length-1],10);
var p2start = parseInt(lines[1][lines[1].length-1],10);

///console.log("p1pos:",p1pos);
///console.log("p2pos:",p2pos);

var rollCount = 0;

function rollD100(){
  
  var roll = rollCount%100 + 1;
  
  rollCount ++;
  
  return roll;
  
}

function simulateGame(p1,p2){
  
  var p1pos = p1-1;
  var p2pos = p2-1;
  
  var p1score = 0;
  var p2score = 0;
  
  while(p1score < 1000 && p2score < 1000){
    
    //Player 1 rolls 3 times and moves
    var p1roll = rollD100() + rollD100() + rollD100();
    p1pos = (p1pos+p1roll)%10;
    p1score += p1pos+1;
    
    if(p1score >= 1000){
      return [p1score,p2score];
    }
    
    //Player 2 rolls 3 times and moves
    var p2roll = rollD100() + rollD100() + rollD100();
    p2pos = (p2pos+p2roll)%10;
    p2score += p2pos+1;
    
    ///console.log(rollCount,p1pos,p2pos,p1score,p2score);
    
  }
  
  return [p1score,p2score];
  
}

var scores = simulateGame(p1start,p2start);

var lowest = scores[0];
if(scores[1] < scores[0]){
  lowest = scores[1];
}

///console.log("scores:",scores);
///console.log("rollCount:",rollCount);

console.log("Part 1:",rollCount*lowest);


////////////////////////////////////////////////////////////////////////////////////////////////////

var diceOutcomes = [];

//                    0,1,2,3,4,5,6,7,8,9
var outcomeChances = [0,0,0,0,0,0,0,0,0,0];

for(var a = 1;a <= 3;a ++){
  for(var b = 1;b <= 3;b ++){
    for(var c = 1;c <= 3;c ++){
      diceOutcomes.push(a+b+c);
      outcomeChances[a+b+c] ++;
    }
  }
}

console.log("diceOutcomes:",diceOutcomes);
console.log("outcomeChances:",outcomeChances);

/// //Take a position, and return how many universes each player wins in from that position
/// function simulateDiracGame(p1pos,p2pos,p1score,p2score){
///   
///   var p1wins = 0;
///   var p2wins = 0;
///   
///   for(var r1 = 0;r1 < diceOutcomes.length;r1 ++){
///     
///     var p1newpos = (p1pos + diceOutcomes[r1]) % 10;
///     var p1newscore = p1score+p1newpos+1;
///     
///     if(p1newscore >= 5){
///       
///       p1wins ++;
///       
///     }else{
///       
///       for(var r2 = 0;r2 < diceOutcomes.length;r2 ++){
///         
///         var p2newpos = (p2pos + diceOutcomes[r2]) % 10;
///         var p2newscore = p2score+p2newpos+1;
///         
///         if(p2newscore >= 5){
///           
///           p2wins ++;
///           
///         }else{
///           
///           var wins = simulateDiracGame(p1newpos,p2newpos,p1newscore,p2newscore);
///           p1wins += wins[0];
///           p2wins += wins[1];
///           
///         }
///         
///       }
///       
///     }
///     
///   }
///   
///   return [p1wins,p2wins];
///   
/// }

//Take a position, and return how many universes each player wins in from that position
function simulateDiracGame(p1pos,p2pos,p1score,p2score){
  
  var p1wins = 0n;
  var p2wins = 0n;
  
  //P1 roll
  for(var r1 = 3;r1 < outcomeChances.length;r1 ++){
    
    var p1newpos = (p1pos + r1) % 10;
    var p1newscore = p1score+p1newpos+1;
    
    if(p1newscore >= 21){
      
      p1wins += BigInt(outcomeChances[r1]);
      
    }else{
      
      //P2 roll
      for(var r2 = 3;r2 < outcomeChances.length;r2 ++){
        
        var p2newpos = (p2pos + r2) % 10;
        var p2newscore = p2score+p2newpos+1;
        
        if(p2newscore >= 21){
          
          p2wins += BigInt(outcomeChances[r1]*outcomeChances[r2]);
          
        }else{
          
          //Use recursion to simulate the next turn
          var wins = simulateDiracGame(p1newpos,p2newpos,p1newscore,p2newscore);
          p1wins += wins[0]*BigInt(outcomeChances[r1]*outcomeChances[r2]);
          p2wins += wins[1]*BigInt(outcomeChances[r1]*outcomeChances[r2]);
          
        }
        
      }
      
    }
    
  }
  
  return [p1wins,p2wins];
  
}

var wins = simulateDiracGame(p1start-1,p2start-1,0,0);

console.log("Result:",wins);

var highest = (wins[0] < wins[1] ? wins[1] : wins[0]);

console.log("Part 2:",highest);











