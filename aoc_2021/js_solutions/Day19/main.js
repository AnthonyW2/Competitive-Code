/**

Anthony Wilson - Advent of Code 2021 - Day 19

2021-12-19

**/

var input = `--- scanner 0 ---
-809,-750,623
-853,-746,517
-136,-29,-84
318,-839,681
-474,-873,-609
727,841,-615
-464,-774,-678
-413,613,-400
660,790,-521
407,-813,737
809,365,495
336,-500,-487
306,-520,-581
-413,723,-516
9,72,-144
-638,485,266
-394,-871,-704
-579,631,334
418,-893,635
804,470,572
-365,640,-524
812,489,534
326,-543,-425
729,809,-486
-885,-695,487
-599,423,286

--- scanner 1 ---
449,611,-798
802,-370,-638
-724,508,707
-657,-438,613
559,-402,500
-617,-418,686
-718,895,-439
812,-468,-646
-528,-655,-312
-608,-540,-348
75,5,6
-760,512,753
574,-414,383
443,386,601
873,-483,-709
551,405,535
572,-563,422
440,664,-754
569,587,-716
-575,534,792
503,454,661
34,166,145
-688,-657,-293
-710,909,-311
-698,-362,567
-625,898,-447

--- scanner 2 ---
704,-944,-857
-733,515,415
-399,549,-834
502,761,-631
-364,654,-893
493,712,852
-402,629,-706
-655,439,375
664,769,-541
-534,-550,739
-698,-788,-816
-624,-569,721
-890,-797,-866
635,673,805
-545,534,401
559,633,-556
-563,-592,679
727,-804,-871
635,729,716
-42,-131,-38
508,-589,781
-683,-790,-928
782,-830,-890
489,-655,697
396,-551,798

--- scanner 3 ---
783,-601,626
-580,551,636
-925,-529,-667
665,-549,-523
-8,82,105
-579,541,761
800,-660,653
628,-439,-487
-809,-530,-545
620,418,774
-823,732,-387
-891,566,-359
-467,532,590
-521,-336,520
-525,-354,546
-115,-29,-4
-628,-367,385
-838,694,-404
549,540,-421
-806,-581,-754
558,502,-429
519,601,-521
821,-634,785
747,421,800
681,440,704
677,-467,-496

--- scanner 4 ---
705,-267,639
-584,827,-496
-33,55,-59
-618,-265,557
-647,501,615
674,-308,727
-684,608,591
413,450,680
-609,536,473
572,-670,-545
-708,-240,-869
427,393,-730
656,-688,-550
387,431,735
-748,-323,-973
-640,-253,402
-686,750,-611
420,455,-937
273,464,657
680,-254,719
-623,-332,592
429,505,-745
-744,-321,-744
-728,866,-558
426,-699,-584

--- scanner 5 ---
834,879,-701
-848,747,643
-370,-566,861
-737,713,-408
-657,-360,-496
-660,753,-420
-623,-396,-596
-62,62,50
781,530,341
424,-638,569
596,-570,591
-828,809,682
-485,-675,879
-621,-302,-404
668,-636,532
412,-838,-825
803,784,-712
-780,601,702
777,606,378
-797,708,-315
-569,-598,914
835,547,375
474,-747,-689
416,-782,-858
806,776,-610

--- scanner 6 ---
322,287,631
-662,-884,857
-141,-142,14
-513,491,-585
4,-3,106
-840,700,719
278,296,578
-814,-882,-303
637,-871,862
367,720,-516
765,-863,876
438,649,-403
-875,768,533
-397,532,-644
-764,774,672
360,416,554
430,659,-501
629,-778,865
-711,-871,711
-706,-816,-332
764,-681,-688
-673,-893,928
-791,-662,-272
728,-761,-573
783,-841,-690
-575,526,-648

--- scanner 7 ---
723,683,-372
-785,446,-893
-649,-583,-467
-501,-580,-465
-711,424,-880
-580,-524,-629
-719,487,403
-421,-436,791
-793,534,479
422,-780,737
532,612,-369
852,495,532
-405,-483,800
865,349,581
918,-625,-478
882,-482,-512
124,62,-55
-734,377,-773
874,278,570
16,-108,68
546,747,-406
-787,673,368
-417,-605,770
884,-514,-475
523,-676,644
439,-742,625

--- scanner 8 ---
-858,-680,-691
414,-517,587
625,667,442
-545,771,-555
-851,709,576
-602,685,-623
-722,-681,836
-2,154,-76
587,676,434
727,-677,-463
-515,733,-581
487,-488,566
-157,51,27
706,478,-787
526,706,534
637,-496,561
803,-740,-352
-823,-586,-854
-651,-625,786
-796,737,677
-513,-707,829
-795,780,485
687,-723,-404
709,605,-761
-767,-606,-703
687,544,-691

--- scanner 9 ---
109,7,73
-400,-440,-543
467,669,632
-30,-84,2
493,638,678
772,315,-497
678,-864,-522
486,-786,649
475,-708,755
-403,-430,-509
-538,-912,807
-513,341,481
-541,-861,616
451,712,698
-497,-958,636
413,-724,763
-641,288,436
-475,356,464
-675,712,-513
-666,600,-684
604,-729,-609
-494,-409,-454
-656,599,-559
796,438,-442
669,-761,-487
811,412,-602

--- scanner 10 ---
-518,-468,-442
-480,-360,707
356,-654,-380
469,774,401
739,538,-422
-696,447,-532
613,836,440
-813,392,-452
-846,720,431
-594,-514,-483
-630,-389,674
352,-608,-416
567,-534,637
-574,-587,-523
378,-693,-565
626,-440,604
669,-421,699
-810,929,405
-799,798,380
742,546,-515
-589,-305,834
753,410,-430
-176,104,-105
-6,-40,-35
511,721,517
-811,411,-612

--- scanner 11 ---
-301,-625,-354
-705,-484,764
-407,439,376
866,-341,-713
-767,430,-787
154,16,27
879,-346,586
436,783,430
-779,-531,688
773,-338,-705
537,679,523
866,-492,635
453,745,542
513,765,-590
-421,-649,-436
858,-327,-841
-391,486,367
-684,575,-767
-292,-572,-398
-403,608,397
-7,54,-106
-676,473,-882
495,784,-500
555,742,-529
-773,-670,799
888,-275,597

--- scanner 12 ---
575,709,-663
-520,-669,720
-582,759,436
-540,-332,-636
475,770,-693
655,-390,397
458,616,694
-465,-693,597
-746,889,-891
483,808,-702
413,744,747
-514,-362,-598
609,-364,366
-536,778,323
675,-348,323
441,578,679
-73,49,-59
-707,769,435
-582,-419,-684
-720,892,-764
617,-668,-660
-650,870,-825
-625,-708,623
526,-612,-589
718,-613,-632
46,185,7

--- scanner 13 ---
-440,691,-410
-953,891,363
-918,832,468
646,501,-442
381,-841,-740
576,447,426
-648,-353,-584
392,-640,312
-158,89,-157
-662,-377,578
462,-701,334
-772,-403,483
-443,688,-473
-584,-417,502
-915,704,388
781,608,-473
513,-820,-766
-391,673,-502
366,-767,315
540,-803,-707
530,560,445
620,505,-460
-572,-450,-527
-664,-344,-563
-60,23,14
405,559,435

--- scanner 14 ---
-799,642,-482
-610,668,689
409,462,776
501,694,-676
-736,-387,-705
623,-522,506
-698,670,-427
-607,599,-483
-192,168,-21
459,646,-763
439,-710,-426
-605,729,810
-486,-788,585
-690,-356,-731
-87,59,88
-470,-707,680
370,647,754
-592,810,815
384,-587,-402
433,608,-733
-453,-771,774
-756,-313,-747
544,-613,-387
529,-555,565
642,-617,463
455,498,780

--- scanner 15 ---
-45,111,125
849,932,816
589,-682,-407
-634,-519,-444
814,873,711
-702,-433,-387
480,-376,677
-676,-471,-357
-591,788,-733
-696,885,-694
530,967,-628
50,14,15
-496,-599,867
811,745,791
-566,-660,845
558,-336,601
-634,601,734
515,855,-626
-680,488,753
679,-700,-388
-751,762,-718
323,905,-635
668,-362,696
650,-640,-430
-509,600,744
-470,-726,864

--- scanner 16 ---
736,391,727
765,-597,557
735,-417,-729
28,87,-12
-560,-323,-329
740,354,803
-838,-302,625
681,852,-531
717,746,-623
-668,409,-594
-869,695,524
762,-699,549
794,388,786
-701,-407,580
-137,-21,125
-532,485,-649
793,-688,454
-655,-351,-286
-821,715,732
-638,441,-645
-778,-395,620
858,-485,-708
-889,656,586
660,843,-547
-703,-289,-413
810,-352,-693

--- scanner 17 ---
-488,691,763
-536,649,676
143,145,-109
950,-515,492
-354,-323,-379
775,508,-463
477,-687,-663
888,-308,485
-346,-451,-393
868,711,426
-489,780,-754
771,606,387
-439,-459,-390
680,527,-506
516,-749,-519
-450,717,733
-699,-574,713
-717,-664,732
838,-500,494
713,621,-565
-304,741,-776
745,766,402
-657,-635,690
593,-634,-621
-25,-44,-20
-340,695,-780

--- scanner 18 ---
858,-616,-298
855,-557,-356
-609,-269,-672
-841,412,515
-56,18,86
403,717,-432
-504,-511,536
-446,-227,-659
-623,974,-350
768,-587,440
-638,922,-321
467,781,654
500,793,-375
453,593,635
904,-445,457
892,-633,-352
105,134,162
-782,447,620
-620,-289,-646
-641,925,-258
-654,450,540
-489,-616,400
427,703,730
394,701,-411
857,-587,496
-579,-454,408

--- scanner 19 ---
-669,609,672
162,-147,-19
517,-725,570
-382,264,-818
686,-844,566
455,603,-473
602,657,-547
-601,-642,-787
759,-641,-815
898,-550,-766
47,-17,-148
623,-659,591
-571,617,682
-384,283,-658
-637,-529,429
-625,-412,444
-677,-596,-828
-556,680,743
461,732,-630
-476,-451,442
905,-687,-713
-499,319,-738
-621,-715,-747
804,653,371
761,480,437
693,560,351

--- scanner 20 ---
-772,521,434
-660,559,451
594,777,-568
703,-251,553
-826,865,-409
-700,-733,-772
-692,-569,-660
440,787,896
-835,818,-393
-675,-742,380
-679,625,535
-533,-703,427
-626,-597,-724
-533,-759,516
649,-388,561
-805,696,-405
443,676,930
580,-420,-503
730,-395,-594
631,-452,-584
694,-436,464
432,718,-637
430,721,-489
27,35,120
477,819,894

--- scanner 21 ---
637,-453,823
495,926,-740
-380,775,587
-572,-626,-694
742,570,865
-509,485,-368
-611,600,-445
-534,-565,494
-609,-602,-587
-534,-484,-659
723,660,847
741,-360,924
-383,783,494
700,599,805
703,-472,907
462,-746,-764
504,734,-690
-388,-516,481
-653,547,-329
-369,911,502
-317,-527,472
348,-668,-731
502,719,-802
488,-691,-745
-1,70,63

--- scanner 22 ---
355,509,452
-378,717,-376
-462,632,-339
-522,-796,-510
678,-666,731
478,454,428
543,693,-480
-746,-637,738
43,-95,68
-392,609,-533
-668,-743,749
-721,499,555
448,712,-437
-536,-779,-580
-560,-882,-592
-45,-8,-63
752,-686,773
-689,621,620
709,-620,757
294,-383,-748
-684,-751,811
510,599,-539
358,-400,-654
452,614,413
-824,534,546
277,-416,-559

--- scanner 23 ---
-599,295,-722
601,681,-857
383,788,454
-591,298,-815
577,-828,-711
-714,-808,-462
-463,284,-807
278,-550,646
-422,-516,727
-950,582,352
640,659,-672
-868,606,472
-775,-785,-504
509,-771,-625
509,739,406
429,-606,666
-182,-124,48
-804,-827,-444
-537,-618,735
-112,-17,-80
-517,-473,818
551,-858,-673
555,685,-821
449,807,346
441,-515,740
-906,529,343

--- scanner 24 ---
33,110,-60
642,719,-823
-679,-685,-841
693,689,-791
595,-609,-389
671,-666,-486
-684,872,-517
-476,920,-537
-738,-743,-844
-755,-689,-878
800,-720,652
-418,711,871
-538,940,-576
-391,-369,623
-411,-395,634
840,571,480
579,-630,-380
-349,-382,460
-523,634,866
-121,35,74
852,-691,637
760,469,469
803,-783,655
786,525,380
697,583,-747
-542,672,874`;

var splitInput = input.split("\n\n");

//Each orientation describes which other value each coordinate-value should be replaced with
//A negative value indicates that the value should be made negative
var orientations = [
  //Neutral
  [1,2,3],   //Neutral
  [-2,1,3],  //90deg around 3
  [-1,-2,3], //180deg around 3
  [2,-1,3],  //270deg around 3
  
  //90deg around 2
  [3,2,-1],  //Neutral
  [3,1,2],   //90deg around 3
  [3,-2,1],  //180deg around 3
  [3,-1,-2], //270deg around 3
  
  //180deg around 2
  [-1,2,-3], //Neutral
  [2,1,-3],  //90deg around 3
  [1,-2,-3], //180deg around 3
  [-2,-1,-3],//270deg around 3
  
  //270deg around 2
  [-3,2,1],  //Neutral
  [-3,1,-2], //90deg around 3
  [-3,-2,-1],//180deg around 3
  [-3,-1,2], //270deg around 3
  
  //90deg around 1
  [1,-3,2],  //Neutral
  [-2,-3,1], //90deg around 3
  [-1,-3,-2],//180deg around 3
  [2,-3,-1], //270deg around 3
  
  //270deg around 1
  [1,3,-2],  //Neutral
  [-2,3,-1], //90deg around 3
  [-1,3,2],  //180deg around 3
  [2,3,1]    //270deg around 3
];

class Scanner{
  
  constructor(str){
    
    console.log("Created scanner");
    
    var lines = str.split("\n");
    
    //Store the relative coordinates of all beacons in range
    this.beacons = [];
    
    for(var l = 1;l < lines.length;l ++){
      
      var coords = lines[l].split(",");
      coords[0] = parseInt(coords[0]);
      coords[1] = parseInt(coords[1]);
      coords[2] = parseInt(coords[2]);
      
      this.beacons.push(coords);
      
    }
    
    console.log(this.beacons);
    
    this.position = [];
    this.orientation = 0;
    
  }
  
  getBeacon(beacon){
    
    //Get the position of the requested beacon after accounting for orientation
    var reoriented = reorient(this.beacons[beacon],orientations[this.orientation]);
    
    //Return the absolute position of the beacon
    return [
      reoriented[0] + this.position[0],
      reoriented[1] + this.position[1],
      reoriented[2] + this.position[2]
    ];
    
  }
  
}

//Return a new set of relative coordinates of a beacon after rotating around the scanner
function reorient(pos,orientation){
  
  var output = [0,0,0];
  
  for(var a = 0;a < 3;a ++){
    
    if(orientation[a] < 0){
      
      output[a] = -pos[Math.abs(orientation[a])-1];
      
    }else{
      
      output[a] = pos[orientation[a]-1];
      
    }
    
  }
  
  return output;
  
}

//Compare two 3D positions
function positionsMatch(pos1, pos2){
  
  if(pos1[0] == pos2[0] && pos1[1] == pos2[1] && pos1[2] == pos2[2]){
    return true;
  }
  
  return false;
  
}

var scanners = [];

for(var s = 0;s < splitInput.length;s ++){
  
  scanners.push( new Scanner(splitInput[s]) );
  
}

scanners[0].position = [0,0,0];


//The growth rate of the following code is O(S^3 * B^4), where S = number of scanners, and B = average number of beacons each scanner can see

//Continue looping the process of identifying the scanner positions until all are found
var complete = true; /// DISABLED FOR PART 2
while(!complete){
  
  //Loop through all scanners to find their positions (the first one is known, so start at 1 instead of 0)
  for(var s0 = 1;s0 < scanners.length;s0 ++){
    
    console.log("Finding position of scanner:",s0);
    
    //Check if the scanner is already in a known position
    if(scanners[s0].position.length != 3){
      
      //Loop through all scanners to compare data to find scanners[s0]'s position
      for(var s1 = 0;s1 < scanners.length && scanners[s0].position.length != 3;s1 ++){
        
        console.log("Comparing with position of scanner:",s1);
        
        //Ensure scanners[s1] has a known position, and is also not the current scanner
        if(scanners[s1].position.length == 3 && s0 != s1){
          
          //Loop through all 24 possible orientations of scanners[s0]
          for(var o = 0;o < 24;o ++){
            
            //Loop through all the beacons of scanners[s0]
            for(var b0 = 0;b0 < scanners[s0].beacons.length;b0 ++){
              
              //Calculate the new relative coordinates of the beacon - accounting for orientation
              var beacon0rel = reorient(scanners[s0].beacons[b0],orientations[o]);
              
              //Loop through all the beacons of scanners[s1]
              for(var b1 = 0;b1 < scanners[s1].beacons.length;b1 ++){
                
                //Match the position of scanners[s0].beacons[b0] and scanners[s1].beacons[b1] by re-aligning scanners[s0]
                
                //Get the (absolute) coordinates of the beacon, accounting for orientation and scanner position
                var beacon1abs = scanners[s1].getBeacon(b1);
                
                //Calculate a possible position of scanners[s0] by lining up one of its beacons (b0) with that of scanners[s1] (b1)
                var scanner0PosGuess = [
                  beacon1abs[0] - beacon0rel[0],
                  beacon1abs[1] - beacon0rel[1],
                  beacon1abs[2] - beacon0rel[2]
                ];
                
                
                //Find how many beacon positions match between the two scanners, assuming scanner[s0]'s position
                var matches = 0;
                
                //Loop through both sets of beacons again
                for(var b2 = 0;b2 < scanners[s0].beacons.length;b2 ++){
                  for(var b3 = 0;b3 < scanners[s1].beacons.length;b3 ++){
                    
                    var beacon2rel = reorient(scanners[s0].beacons[b2],orientations[o]);
                    var beacon2abs = [
                      beacon2rel[0] + scanner0PosGuess[0],
                      beacon2rel[1] + scanner0PosGuess[1],
                      beacon2rel[2] + scanner0PosGuess[2]
                    ];
                    
                    var beacon3abs = scanners[s1].getBeacon(b3);
                    
                    //Check if the 2 beacon positions match
                    if(positionsMatch(beacon2abs,beacon3abs)){
                      matches ++;
                    }
                    
                  }
                }
                
                if(matches > 1){
                  console.log("matches:",matches);
                }
                
                if(matches >= 12){
                  
                  console.log("Found position of scanner:",s0);
                  
                  //Set the necessary values
                  scanners[s0].position = scanner0PosGuess;
                  scanners[s0].orientation = o;
                  
                  //Set the values to exit the layers of loops
                  b1 = scanners[s1].beacons.length;
                  b0 = scanners[s0].beacons.length;
                  o = 24;
                  
                }
                
              }
              
            }
            
          }
          
        }
        
      }
      
    }
    
    if(scanners[s0].position.length != 3){
      console.log("WARNING - scanner position not found");
    }
    
  }
  
  var complete = true;
  
  console.log("Final scanner positions:");
  for(var s = 0;s < scanners.length;s ++){
    
    console.log(s,scanners[s].position,scanners[s].orientation);
    
    if(scanners[s].position.length != 3){
      complete = false;
    }
    
  }
  
}

//Get all the absolute positions of all beacons, as seen from all scanners
var allBeaconAbsPositions = [];
for(var s = 0;s < scanners.length;s ++){
  for(var b = 0;b < scanners[s].beacons.length;b ++){
    
    allBeaconAbsPositions.push( scanners[s].getBeacon(b) );
    
  }
}

console.log("Length of allBeaconAbsPositions:",allBeaconAbsPositions.length);

//Remove duplicate beacons from the allBeaconAbsPositions list
for(var a = 0;a < allBeaconAbsPositions.length;a ++){
  
  for(var b = a+1;b < allBeaconAbsPositions.length;b ++){
    
    if(positionsMatch(allBeaconAbsPositions[a],allBeaconAbsPositions[b])){
      
      allBeaconAbsPositions.splice(b,1);
      b --;
      
    }
    
  }
  
}

console.log("allBeaconAbsPositions:",allBeaconAbsPositions);

console.log("Part 1:",allBeaconAbsPositions.length);


////////////////////////////////////////////////////////////////////////////////////////////////////

var part2input = [
  [ 0, 0, 0 ]           ,
  [ -1186, -1165, 5986 ],
  [ -100, 21, 3416 ]    ,
  [ -118, -2462, 2293 ] ,
  [ 1221, 68, 4633 ]    ,
  [ -1281, 1232, 1114 ] ,
  [ -1339, -89, 5953 ]  ,
  [ -1320, 1154, 4747 ] ,
  [ -1343, -2449, 3420 ],
  [ 1061, -2385, 3500 ] ,
  [ -46, -1264, 5830 ]  ,
  [ -1334, -13, 4750 ]  ,
  [ 26, -2408, 3603 ]   ,
  [ -98, -1296, 2399 ]  ,
  [ 36, -2327, 4698 ]   ,
  [ -139, -1192, 3578 ] ,
  [ -1294, -101, 1155 ] ,
  [ -122, -1177, 4698 ] ,
  [ -48, 69, 2247 ]     ,
  [ -1316, -1136, 7025 ],
  [ -1332, 1189, 3446 ] ,
  [ 26, 57, 4708 ]      ,
  [ 1101, -2397, 4724 ] ,
  [ -1283, -1317, 4793 ],
  [ -62, -53, 1206 ]
];

var largestDistance = 0;

for(var s0 = 0;s0 < part2input.length;s0 ++){
  for(var s1 = 0;s1 < part2input.length;s1 ++){
    
    var dist = Math.abs(part2input[s0][0] - part2input[s1][0]) + Math.abs(part2input[s0][1] - part2input[s1][1]) + Math.abs(part2input[s0][2] - part2input[s1][2]);
    
    if(dist > largestDistance){
      largestDistance = dist;
    }
    
  }
}


console.log("Part 2:",largestDistance);




