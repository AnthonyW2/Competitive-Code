/**

Anthony Wilson - Advent of Code 2021 - Day 14

2021-12-14

**/

var input = `KHSNHFKVVSVPSCVHBHNP

FV -> H
SB -> P
NV -> S
BS -> K
KB -> V
HB -> H
NB -> N
VB -> P
CN -> C
CF -> N
OF -> P
FO -> K
OC -> F
BN -> V
PO -> O
OS -> B
KH -> N
BB -> C
PV -> K
ON -> K
NF -> H
BV -> K
SN -> N
PB -> S
PK -> F
PF -> S
BP -> K
SP -> K
NN -> K
FP -> N
NK -> N
SF -> P
HS -> C
OH -> C
FS -> H
VH -> N
CO -> P
VP -> H
FF -> N
KP -> B
BH -> B
PP -> F
SS -> P
CV -> S
HO -> P
PN -> K
SO -> O
NO -> O
NH -> V
HH -> F
KK -> C
VO -> B
KS -> B
SV -> O
OP -> S
VK -> H
KF -> O
CP -> H
SH -> H
NC -> S
KC -> O
CK -> H
CH -> B
KO -> O
OV -> P
VF -> V
HN -> P
FH -> P
BC -> V
HV -> N
BO -> V
PH -> P
NP -> F
FN -> F
FK -> P
SC -> C
KN -> S
NS -> S
OK -> S
HK -> O
PC -> O
BK -> O
OO -> P
BF -> N
SK -> V
VS -> B
HP -> H
VC -> V
KV -> P
FC -> H
HC -> O
HF -> S
CB -> H
CC -> B
PS -> C
OB -> B
CS -> S
VV -> S
VN -> H
FB -> N`;

var lines = input.split("\n");

//Save the polymer template
var template = lines.splice(0,1)[0];

//Remove the blank line
lines.splice(0,1);


function insertionStep(polymer, rules){
  
  for(var a = 1;a < polymer.length;a ++){
    
    var pair = polymer[a-1] + polymer[a];
    
    for(var r = 0;r < rules.length;r ++){
      
      if(pair == rules[r][0]){
        
        polymer.splice(a, 0, rules[r][1]);
        r = rules.length;
        a ++;
        
      }
      
    }
    
  }
  
}

//Store the current state of the polymer as an array
var polymer = template.split("");

var rules = [];

for(var l = 0;l < lines.length;l ++){
  
  rules[l] = lines[l].split(" -> ");
  
}

///console.log("template:",polymer);
///console.log("rules:",rules);

for(var s = 0;s < 10;s ++){
  ///console.log("step:",s);
  insertionStep(polymer, rules);
  ///console.log("polymer length:",polymer.length);
}

///console.log("polymer:",polymer);

var characters = [];
var charCounts = [];

for(var r = 0;r < rules.length;r ++){
  
  var alreadyRecorded = false;
  
  for(var c = 0;c < characters.length;c ++){
    
    if(rules[r][1] == characters[c]){
      alreadyRecorded = true;
      c = characters.length;
    }
    
  }
  
  if(!alreadyRecorded){
    
    characters.push(rules[r][1]);
    charCounts.push(0);
    
    //Count the occurrences of the character in the final polymer
    for(var a = 0;a < polymer.length;a ++){
      
      if(polymer[a] == rules[r][1]){
        
        charCounts[charCounts.length-1] ++;
        
      }
      
    }
    
  }
  
}

///console.log("characters:",characters);
///console.log("charCounts:",charCounts);

var mostCommon = 0;
var leastCommon = 0;

for(var c = 0;c < characters.length;c ++){
  
  if(charCounts[c] > charCounts[mostCommon]){
    mostCommon = c;
  }
  
  if(charCounts[c] < charCounts[leastCommon]){
    leastCommon = c;
  }
  
}

///console.log("mostCommon:",mostCommon);
///console.log("leastCommon:",leastCommon);

console.log("Part 1:",charCounts[mostCommon]-charCounts[leastCommon]);


////////////////////////////////////////////////////////////////////////////


//Part 2: Optimisation problem

function insertionStepOptimised(pairTypes, pairCounts, rules){
  
  newPairTypes = JSON.parse(JSON.stringify(pairTypes));
  
  newPairCounts = [];
  for(var p = 0;p < pairTypes.length;p ++){
    newPairCounts[p] = 0;
  }
  
  for(var p = 0;p < pairTypes.length;p ++){
    
    for(var r = 0;r < rules.length;r ++){
      
      if(pairTypes[p] == rules[r][0]){
        
        var newPair1 = pairTypes[p].charAt(0) + rules[r][1];
        var newPair2 = rules[r][1] + pairTypes[p].charAt(1);
        
        //Match the new pairs to their corresponding pair counts
        var pair1ID = undefined;
        var pair2ID = undefined;
        
        for(var t = 0;t < newPairTypes.length;t ++){
          
          if(newPair1 == newPairTypes[t]){
            pair1ID = t;
          }
          if(newPair2 == newPairTypes[t]){
            pair2ID = t;
          }
          
        }
        
        
        //Add the count of the original pair to the corresponding pair counts, or add a new pair type & count if necessary
        
        if(pair1ID == undefined){
          newPairTypes.push(newPair1);
          newPairCounts.push(pairCounts[p]);
        }else{
          newPairCounts[pair1ID] += pairCounts[p];
        }
        
        if(pair2ID == undefined){
          newPairTypes.push(newPair2);
          newPairCounts.push(pairCounts[p]);
        }else{
          newPairCounts[pair2ID] += pairCounts[p];
        }
        
      }
      
    }
    
  }
  
  return [newPairTypes, newPairCounts];
  
}


var pairs = [];

var pairTypes = [];

var pairCounts = [];

var polymer = template.split("");

var firstChar = polymer[0];
var lastChar = polymer[polymer.length-1];

for(var a = 1;a < polymer.length;a ++){
  
  var pair = polymer[a-1] + polymer[a];
  
  pairs.push(pair);
  
}

///console.log("pairs:",pairs);

for(var p = 0;p < pairs.length;p ++){
  
  var alreadyRecorded = false;
  
  for(var t = 0;t < pairTypes.length;t ++){
    
    if(pairs[p] == pairTypes[t]){
      
      alreadyRecorded = true;
      pairCounts[t] ++;
      t = pairTypes.length;
      
    }
    
  }
  
  if(!alreadyRecorded){
    
    pairTypes.push(pairs[p]);
    pairCounts.push(1);
    
  }
  
}

///console.log("pairTypes:",pairTypes);
///console.log("pairCounts:",pairCounts);


//Execute 40 iterations
for(var i = 0;i < 40;i ++){
  var output = insertionStepOptimised(pairTypes, pairCounts, rules);
  pairTypes = output[0];
  pairCounts = output[1];
}

///console.log("pairTypes:",pairTypes);
///console.log("pairCounts:",pairCounts);


//[Use the "characters" array from part 1]
charCounts = [];
for(var c = 0;c < characters.length;c ++){
  charCounts[c] = 0;
}

//Count all characters present in final array of pairs
for(var p = 0;p < pairTypes.length;p ++){
  
  for(var c = 0;c < characters.length;c ++){
    
    if(pairTypes[p].charAt(0) == characters[c]){
      
      charCounts[c] += pairCounts[p];
      
    }
    if(pairTypes[p].charAt(1) == characters[c]){
      
      charCounts[c] += pairCounts[p];
      
    }
    
  }
  
}

///console.log("charCounts:",charCounts);

var firstCharID = 0;
var lastCharID = 0;

//Identify the IDs of the first & last characters
for(var c = 0;c < characters.length;c ++){
  
  if(firstChar == characters[c]){
    firstCharID = c;
  }
  if(lastChar == characters[c]){
    lastCharID = c;
  }
  
}

//Subtract 1 from the counts of the first & last characters
charCounts[firstCharID] --;
charCounts[lastCharID] --;

//Divide all character counts by 2
for(var c = 0;c < characters.length;c ++){
  charCounts[c] = charCounts[c]/2;
}

//Add 1 to the counts of the first & last characters
charCounts[firstCharID] ++;
charCounts[lastCharID] ++;

///console.log("characters:",characters);
///console.log("charCounts:",charCounts);


mostCommon = 0;
leastCommon = 0;

for(var c = 0;c < characters.length;c ++){
  
  if(charCounts[c] > charCounts[mostCommon]){
    mostCommon = c;
  }
  
  if(charCounts[c] < charCounts[leastCommon]){
    leastCommon = c;
  }
  
}

console.log("Part 2:",charCounts[mostCommon]-charCounts[leastCommon]);













