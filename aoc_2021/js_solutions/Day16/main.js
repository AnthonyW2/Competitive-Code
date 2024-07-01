/**

Anthony Wilson - Advent of Code 2021 - Day 16

2021-12-16

**/

var input = `020D790050D26C13EC1348326D336ACE00EC299E6A8B929ED59C880502E00A526B969F62BF35CB4FB15B93A6311F67F813300470037500043E2D4218FA000864538E905A39CAF77386E35AB01802FC01BA00021118617C1F00043A3F4748A53CF66008D00481272D73308334EDB0ED304E200D4E94CF612A49B40036C98A7CF24A53CA94C6370FBDCC9018029600ACD529CA9A4F62ACD2B5F928802F0D2665CA7D6CC013919E78A3800D3CF7794A8FC938280473057394AFF15099BA23CDD37A08400E2A5F7297F916C9F97F82D2DFA734BC600D4E3BC89CCBABCBE2B77D200412599244D4C0138C780120CC67E9D351C5AB4E1D4C981802980080CDB84E034C5767C60124F3BC984CD1E479201232C016100662D45089A00087C1084F12A724752BEFEA9C51500566759BF9BE6C5080217910CD00525B6350E8C00E9272200DCE4EF4C1DD003952F7059BCF675443005680103976997699795E830C02E4CBCE72EFC6A6218C88C9DF2F3351FCEF2D83CADB779F59A052801F2BAACDAE7F52A8190073937FE1D700439234DBB4F7374DC0CC804CF1006A0D47B8A4200F445865170401F8251662D100909401AB8803313217C680004320D43F871308D140C010E0069E7EDD1796AFC8255800052E20043E0F42A8B6400864258E51088010B85910A0F4ECE1DFE069C0229AE63D0B8DC6F82529403203305C00E1002C80AF5602908400A20240100852401E98400830021400D30029004B6100294008400B9D0023240061C000D19CACCD9005F694AEF6493D3F9948DEB3B4CC273FFD5E9AD85CFDFF6978B80050392AC3D98D796449BE304FE7F0C13CD716656BD0A6002A67E61A400F6E8029300B300B11480463D004C401889B1CA31800211162204679621200FCAC01791CF6B1AFCF2473DAC6BF3A9F1700016A3D90064D359B35D003430727A7DC464E6401594A57C93A0084CC56A662B8C00AA424989F2A9112`;

//var binary = parseInt(input,16).toString(2); //This remove leading zeros, which are necessary for this task
var binary = "";

var hexDigits = [
  "0",
  "1",
  "2",
  "3",
  "4",
  "5",
  "6",
  "7",
  "8",
  "9",
  "a",
  "b",
  "c",
  "d",
  "e",
  "f"
];

var hexBinMap = [
  "0000",
  "0001",
  "0010",
  "0011",
  "0100",
  "0101",
  "0110",
  "0111",
  "1000",
  "1001",
  "1010",
  "1011",
  "1100",
  "1101",
  "1110",
  "1111"
];

for(var c = 0;c < input.length;c ++){
  
  for(var h = 0;h < hexDigits.length;h ++){
    
    if(input.charAt(c).toLowerCase() == hexDigits[h]){
      
      binary += hexBinMap[h];
      
    }
    
  }
  
}

///console.log("Binary:",binary);


function getPacketVersionSum(packet){
  
  ///console.log("packet:",packet);
  
  if(packet.length < 10){
    return 0;
  }
  
  var version = parseInt(packet.substr(0,3),2);
  var type = parseInt(packet.substr(3,3),2);
  
  ///console.log("version:",version);
  ///console.log("type:",type);
  
  var output = version;
  
  if(type == 4){
    //Literal value
    
    var valueBin = 0;
    
    var pos = 6;
    
    while(packet.charAt(pos) != '0' && pos+4 < packet.length){
      
      valueBin += packet.substr(pos+1,4);
      
      pos += 5;
      
    }
    
    valueBin += packet.substr(pos+1,4);
    
    ///console.log("value:",parseInt(valueBin,2));
    
    //Add the version number of the packet immediately after this packet
    output += getPacketVersionSum(packet.substring(pos+5));
    
  }else{
    //Operator packet
    
    if(packet.charAt(6) == '0'){
      
      if(packet.length < 22){
        return 0;
      }
      
      var length = parseInt(packet.substr(7,15),2);
      
      ///console.log("length:",length);
      
      //Add the version number(s) of the sub-packet(s)
      output += getPacketVersionSum(packet.substr(22,length));
      
      //Add the version number of the packet immediately after this packet
      output += getPacketVersionSum(packet.substring(22+length));
      
    }else{
      
      if(packet.length < 18){
        return 0;
      }
      
      var amount = parseInt(packet.substr(7,11),2);
      
      ///console.log("amount:",amount);
      
      //Since this function only adds up the version numbers, we can skip proper parsing of this packet for now and take a shortcut:
      output += getPacketVersionSum(packet.substring(18));
      
    }
    
  }
  
  return output;
  
}


console.log("Part 1:",getPacketVersionSum(binary));


/////////////////////////////////////////////////////////////////////////////////////


//Take in a packet in the form of binary digits in a string
//Return two values in an array:
// 0 -> The resulting value of this packet
// 1 -> Remainder of the string to be processed
function evaluatePacket(packet){
  
  ///console.log("packet:",packet);
  
  if(packet.length < 10){
    return [];
  }
  
  var version = parseInt(packet.substr(0,3),2);
  var type = parseInt(packet.substr(3,3),2);
  
  ///console.log("version:",version);
  ///console.log("type:",type);
  
  if(type == 4){
    //Literal value
    
    var valueBin = 0;
    
    var pos = 6;
    
    while(packet.charAt(pos) != '0' && pos+4 < packet.length){
      
      valueBin += packet.substr(pos+1,4);
      
      pos += 5;
      
    }
    
    valueBin += packet.substr(pos+1,4);
    
    ///console.log("value:",parseInt(valueBin,2));
    
    //Return the resulting value and the remainder of the packet string
    return [
      parseInt(valueBin,2),
      packet.substring(pos+5)
    ];
    
  }else{
    //Operator packet
    
    var values = [];
    
    var remainder = "";
    
    if(packet.charAt(6) == '0'){
      
      if(packet.length < 22){
        return 0;
      }
      
      var length = parseInt(packet.substr(7,15),2);
      
      ///console.log("length:",length);
      
      var remainingString = packet.substr(22,length);
      
      while(remainingString.length > 6){
        
        var result = evaluatePacket(remainingString);
        
        values.push(result[0]);
        
        remainingString = result[1];
        
      }
      
      remainder = packet.substring(22+length);
      
    }else{
      
      if(packet.length < 18){
        return 0;
      }
      
      var amount = parseInt(packet.substr(7,11),2);
      
      ///console.log("amount:",amount);
      
      var remainingString = packet.substring(18);
      
      for(var a = 0;a < amount;a ++){
        
        var result = evaluatePacket(remainingString);
        
        values.push(result[0]);
        
        remainingString = result[1];
        
      }
      
      remainder = remainingString;
      
    }
    
    //Use the type ID to process the values in some way
    
    var output = 0;
    
    if(type == 0){
      //Sum of values
      
      for(var v = 0;v < values.length;v ++){
        output += values[v];
      }
      
    }else if(type == 1){
      //Product of values
      
      output = values[0];
      for(var v = 1;v < values.length;v ++){
        output *= values[v];
      }
      
    }else if(type == 2){
      //Minimum value
      
      output = values[0];
      for(var v = 1;v < values.length;v ++){
        if(output > values[v]){
          output = values[v];
        }
      }
      
    }else if(type == 3){
      //Maximum value
      
      output = values[0];
      for(var v = 1;v < values.length;v ++){
        if(output < values[v]){
          output = values[v];
        }
      }
      
    }else if(type == 5){
      //Greater than
      
      if(values[0] > values[1]){
        output = 1;
      }else{
        output = 0;
      }
      
    }else if(type == 6){
      //Less than
      
      if(values[0] < values[1]){
        output = 1;
      }else{
        output = 0;
      }
      
    }else if(type == 7){
      //Equal
      
      if(values[0] == values[1]){
        output = 1;
      }else{
        output = 0;
      }
      
    }
    
    return [output,remainder];
    
  }
  
  return false;
  
}

//Wrapper function for evaluatePacket()
//Takes in the binary representation of the transmission in the form of a string
function evaluateTransmission(transmission){
  
  //Since the main transmission should only contain a single packet, this is all that is needed
  
  var result = evaluatePacket(transmission);
  
  ///console.log("result:",result);
  
  return result[0];
  
}


console.log("Part 2:",evaluateTransmission(binary));




