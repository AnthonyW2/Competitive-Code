/**

Anthony Wilson - Advent of Code 2021 - Day 8

2021-12-8

**/

var input = `bcedagf ebadf gcdfe gfcead bcedgf dfeca ac dgca ace cafbge | ecfdbag gecfd feadb degacbf
gab cebfag bdfg gaefd bg debga gadbef dbafgce gdafec cadbe | befdga fgdaec gdaecf bdecgaf
feabcg eacd cdbeg cfgbda dgefb cbgefad ceb dbagc ce gadbec | ec ceb aecd daec
acdbf ebfa fagdec ef bcged edcfba gcedafb facgdb ebcfd ecf | fe fce gfcdab fe
bcg eacfbg daebgfc cfabd gbfaed degfcb gc cdeg cbdfg egfdb | fbgcaed edbfg aedgbcf cg
gdbaf cefgd egdaf deab afe egcbaf afgcbd efbdga ae ecbdagf | ea deab baed ebcgfa
ecadfgb dacbfe bgedfc afebc abf dbcef ecgab adef cfgbda fa | efad baf fab af
gaefcd dgbcef fdbeac cgfab ad dca ebcdf fbdecga dbcfa beda | ad dbefc fgecad edba
egfa dcgbfe ag gac fcdbage cebag gebafc gfebc gdabfc caebd | ag bgace gefa bgfced
dfbage fedcba ga gae bcegda fagb gcedf edfag adfeb geabdfc | ga gcfde ag facdebg
ebcfad agcfbe de cfbae degcaf ebdc bcadfeg fdabe gbfad ead | decfga dbgacef daecfbg dae
cdaegb ced gdfbe bdecfa caefbdg bcdeg gabcd gaec ec bacfdg | agec degafcb cde acbdg
bcadgf bgecfd abcgfe cd acgbd fedbagc gdeba afcd abfcg cdg | dc dbacefg befcadg dgfaecb
afcbed gdfaecb bfdec dbafe fc fcda fgabce gdefba bdecg cfe | fcda fagedb fc egdcb
bdecf dgcbe befcdg bef fgbc gcaebfd bgaecd caedf fb dfabeg | dbfceg fb bfe efb
bge gedabc fdeacbg dagfec feacg gcdbf eb aebf fcaegb fgceb | gbe beg gafce be
efdbcga egfba dafcge eacb acbfg ae edbgf bgfcea dcfgba gae | agbef fcagb bafge fabcg
fcbdg fdgcea dfcgaeb dfgac fdcgab ecbgad gb edcbf afgb gbc | cafbged ebfcdga bdcfg gcb
ceb cdefag be ebfcg agecf degfbca egbfca gaeb abecdf fcgdb | gebfc acgbefd aedfbc dfcbae
bfegd bdgea fd bfcge gfabcd fdg dcgeafb ebgdcf gebfac ecdf | degfb fedbg efgcb fced
dcfabe bcga fgbed ag aecfgb cdegfa baefg aeg eabfc agedbfc | ega aeg ag cebdgfa
fgade eaf abfgd fe abcfeg cfde efcdga dbeafgc daegc acdegb | gadbfec aef fae cdgea
abe debc cdagb agedb gebcfa eadbfgc bacegd gdefa be fcabgd | gdbacfe fagde dgbace acfgdb
fea gabdf ea beacfg cgdfae cabe cgebadf cdefbg ebgaf gefcb | edfacg fae ceba efa
ed fecba bfcdae aefcd ecabdg fcagd begdafc bedf ced gafcbe | dbceaf ced ced ced
abfgc gfbcae fbace gdeabf fae bdgcfa edcab acebfdg gcfe ef | aef fe fbdcga dfaebcg
ac eac dgebcf bcaef begcf cgab cdbgeaf agdcef ebfgac bedaf | dfcaeg bgeadfc bgcafe fgbce
afdbc bgecadf gfecad dca ac aebc dfceba ebfdgc gfdab cdfbe | acbe dac cedafg ca
fadcb efc baedcf fdbge aedc ce dgaefcb cgfbda dbcfe geacfb | ce ce gbcdefa dace
fcbea ebd cbdg ceafdg cdgabe dbeagf bd ecdba fcgedba egdca | ecadbgf gdeac bd acdfge
fbd gcdafb fb bcdea aefgd fcgedab gecdab febad becf fcdeab | badef ceadb bdgacfe afdcebg
dcgfbe cgdefa fc adcf bdeagc gcead cabefdg ecf fegac egfab | gecfadb afecg dcaf bfcgade
fbgdce acge fbegc ecfbadg cfa cdbfea bgfad bafcg cgafeb ac | ac efdcba agdfb fecbad
edg efdgca edba bdegf cafgedb faebdg ed fgecab fgbea bdfgc | eagdfbc cgdbf geafcb gefdcba
gcaedb fgedcba cfbgae fgc gcfde gefdb cfaged cf dafc dgcae | gcf fgeabc bdcgea acfd
bgecdfa bgedaf dbfa cgefda db efgad gdb agbde egbcdf aegbc | dbegfc dgb bdg cgdebf
egacbfd gecfba ecd acfgd degb fbadec de efgcd cfdbeg fbgec | egbcfa cfabdge ebgd ed
ecgdb ebdfc fagdbe fe afce afgdbc bdcaef cbegdfa abfcd dfe | caefbd ef fe bfdce
egadf cd gfdace acebf ecd cefbgd fdeca gfbdae gadc dbegcaf | fdagbe adgc gdeaf dafcegb
cgdfb eabfg abd deafbg dbcega afde bfgda fdeacbg ad fbeagc | fdgeabc feagcdb gafedbc da
bdega dbace fbecd bdfgaec feadbc bfca cda ac edgfac becdgf | ac ca cad bfca
gacef bgcae fgdceab gef fgbade abdecg fgabce cdaef bgfc gf | bfagedc cbfg adbgec bgfc
agcbd eagd abgdefc gec bceaf cbdgfe gfdacb baegdc acbge ge | egc ge gade ecg
cbgda egcad acbged gb bfcad acgfedb ebcfag gdeb cbg fdacge | gadfce edgb cgfabe adbecg
fgcaeb gcbdf dbfagec fgcae da gacdef aedc adf gadfc fegabd | fagdc afbecg deca edac
abedfcg ca bedcf abefc eabfdc aegbf dfbegc dgabcf bac aecd | ecfabd ecbfa acde abc
dag fedag cdeabg dg fdeca fgbae fdgc gadfce cdaebf eagbdfc | gd dcgaebf dgfc adg
aegbf cdga fcebad ecbag ac bca cbedga cfbgade gdbce fbdgec | geabc gadc ebgfacd gadc
aeg abde ea becdfg eadcg adcfg gaefcdb ebgfca acgdbe dcbge | ebad abde cedag ae
cfa eabfd eadfcb bface gdfbca gefbda ca eacd faebdgc cgbfe | efcgb cabdgf eagcfdb cfa
gfdeca fcabe agdbfec cde efdcb dfbaeg debgf cgebfd dc bdcg | ced bcfegd egbafd gcbd
db gfdbae gbfce fbdeg degfa fadecg abed gcfdba aegfdbc dgb | dgfbae db ecbgf daeb
fcdab gcfdab ebafgdc dg fgdc deafbc gbd aebcg dacgb gdfeba | adbcg gd ebgdfac gd
afg eagdcf bagfc fg gdbefca afcdb bcage dbcage ecafgb bgfe | gf aegcb gf gaf
fgc ecfb cbgeaf cf fgabe gdbecfa gedac bfgcda fcaeg debgaf | bfeag cbgefda gbaef abfeg
adcgfe gbfaed dcfbg fdeab fgdeb ebag ge fdgecba fadbec dge | gfcdb ge ged efadcb
cebgd bdfag bcgfd agfc dgfacb fecdab cf dafegb fdgeabc fcd | bacefd acgf cdf cfd
geabfd adf dfecbag agdefc ecdga acgfd fgcbd fa cedagb ecfa | bdgcfae fbgeda gdaec af
cdgabef gfeda dacegf agfdbe dafce aec bdfac efcgba ec cedg | eabdgf egbfad gdeacf ce
febd dab fbdca acdef db cafgb cebafdg bcfdea cagdbe fgacde | dbfe aedfc afcgb efdb
caebfg caegdb efbad da dgaf beafgd edfcb dgabcfe bad ebafg | dba dgfa ad efdbc
dbcef dgfab bcag cgd dcfbag gcdfb gedfba cg degfac dbgeafc | gcd cfebd fedcb cbag
cfbag cefagb dfaceb gb febca bdaefg cgeb fbg efcagdb gcfda | cgeb fbg abefdgc bg
cebgd geacb dbcgea dc dgc gfeacb bedgf cead cgadfb debfagc | ecad eacd dc eacbg
cbdgf fdacg fdbceg dafgcb acd gadb eacgf ad gaefbdc cedafb | dac dca cfage dgab
cgebfa bfedg fb bdgfec gafed bdcf bgedc feb aecgfbd bdcega | bdgcae bfdceg efb gebdc
cadg ac cfa acegfd efdac edgcf bafde afdcgbe egdcfb eagbfc | cfdegab dbcefg cbfgdea acdgef
baecdgf dacbfe ecf gbecad dbfcg cdgea gfae ef afdgce egfdc | aegf ef fe eafg
gc baefcd gfade agc ecgb cdgfab cegaf ebgfdca ecabf agefbc | gc bgfcea afcbe ecgb
ecfad efdab ba dgcafbe adgefb agdb fba egfbcd gecabf fgedb | bcgfed dfgbe dbag fba
bdgea ce ebdfcg fdgbc defcba abdcgf dbcgefa ecd gcbed cegf | ecd egcf efcdba agfbdc
fgcade bfgade bgfcea bfaegcd cfa fgcba fgabe gcbfd abec ca | ca cabe cdbgf dcegfba
aefgbc cedaf agbdcfe agecd fecba gdcfab fd ebcfda fbde cdf | cfagbed df aecfb fbcade
cgbfae fcdbga cgdfe abde gfdea deafgb afdgb ae cbgfead fae | afe afe becfagd ea
ec gfabed aceg dafgceb dec ebcgdf dabfc eagdfc dfcae gdfae | febcgad cgae caeg gabedfc
degacfb fcdgb befc fge edgca cbdfge aedgfb fegdc ef dgbcaf | egf gfe gecdabf gef
abfcg fgeab edfbca bfgadec eaf dbeag bdgaec ef efdg edafbg | aef ecdgba aef abefg
ecdfb cgedb ge adfbeg deg geca cdeabg abdcg afcbdg fgbdeca | ge fdebc deg bedfag
fbgdac dc adgc dcb caebdf cfgbd fbagc fgcaeb bgfde fbcaegd | cgda cbd dbc cd
fgecad dabfg def bcefg cabdgef daeb dbfega ed dbagcf ebfdg | dfe edcagfb bdae agbdfce
acbdfge gb cbdfea fecagb bfg bagdfe abedf agdb gbefd fdceg | febgd bg bg acefdbg
fbcgde efbg efdbc gec dafgc cgdfe agcebd fbceda ge gfadebc | fdebc egcfdab cbdeag ebfg
eadfgc cb dabfg adcegb gecfa bcgeaf acbfg fbec dcabgfe bcg | bgceda bc aecfbg gadecfb
fbade gbedfca fceadg cafed ebfcgd cgda cfa ac dfcge ebgfca | gdfceba feabd ca ac
adefgc fdgca abfdc dgc dabgcfe cg bgedaf geabdc fcge gfead | cadgf egcf cgef gc
dgcae da efcag dgaf gedafc dfabce gbefca ecgbd egdacbf ade | eda dfag aed agfd
gdcbf feac cbgeda gca faegdcb aegdf agbfed ca dcfag edgfca | aecf acedbfg aegbcdf bcgdf
bfdagc eb adgfceb edb dcbag eabg dgfbce dcefa abcged caebd | dbe bde dfacbg begfcad
ged dfecg dfgb gebcf fdbgec dg dcaef bfdgcae eagfbc gedbca | deg degcf fgbd acgbdef
bfgda acgefb cb decafg gcdae aebdgc acb ecdb cabgd bedgafc | dbce aefgcb edbc dfbcaeg
dabfge abdfg egbdac dfabc dcbfe ac daebfcg acb acgf afcgbd | cfag ac cagf cab
fbcea fdb edgcb afcebd cdfa fdebc gfeacb df aefbdg bdfgcae | bdf bdf df afegdbc
fdea gcadeb ef efcgab cfebd adbfce dgfcb feb decba adfgbec | gaebcdf dfecba dcbfg cbdfg
dgf gfadeb dabcgf bgcd fegca bcdaf cfeadb gd fdacg gdeacbf | gfd dbgc dg dg
fgace fbcge ac geadf acde fgdbea dfaebcg acdgbf cfgeda gca | ceafg bdfeagc ca agdfe
cdgbea fc efbdg bcedfga cfba dfc dagcfb fcdgb bagcd gdcefa | cf bgefd cf fegdb
efg afbdeg cbgdae ecgadfb cedagf abedg fagbe bacfe fg gbdf | egf fgbd gf egbaf
fgebca eafcb gdefb efbdc afgdbec cd dcb efacdb dcae bdfgac | cd eadc defgbca fcbadg
fg gbdec gdaebf cfadgb dcfab bfcdage bfg gfcdb fbcdea agcf | edgcfba fgca fcgbd aegbfd
fad bfecagd aecgd dabgcf adfeg feabg bdfaeg df dbfe fcagbe | edfb febacg dfbega fad
gfedacb gcabef bgcdfe fgabe dbgfae bgc gcae cbdaf cg gbcaf | gbc eagfbcd dfgeab gace
fbac fa defba efbcad dceab afe gbdeafc afcdeg dabecg gdbfe | eaf dgfbe dfegbca efbgd
fadgbc bcef afdce egbda adcgef bca dfcaeb eacdb cb cfdaegb | cdfbag becf bc fecb
fc cdgfeb cadeg efcbga aecfg bgfae fegbad dcgebfa fce afcb | fec adfcgbe efc fc
bgcf edbcg cdbfe acfdeb egdcbf dbgae efdbgca dcg gc efgcad | dgeacfb bedcg fgcb gc
fcb acdgf eadfcg abfgc ecgba cbagdf gdebfc fbda fb ebcfgad | cfgda cfagb bf dafcg
dabgfc bcagf gfdcea fegcb fa gdbca abdf dbcaeg bdgeafc fag | ecdfag adbf cafbdg cbadfge
gfacde bc gabdce cbdagef ceb acbf cfbeag cgfeb cgeaf gfbed | gbfce cb fcab gbecf
gcefd gf gfca gcdafe adebfg fdeac fecadb ecdbg fegdcba gfd | cfga gf acfgdbe fgd
cg bcdafg dgac cdgfb gefbd aecbdf cagfeb afedbcg bafcd cbg | bfadcge dfacebg dacg gc
fe abfe gcdbfea fgcdb fce bdceag cfgeb abcge egcbaf egfcda | agfebc bgace aefb abceg
dc cdg bedga bfacg gbcad gbecda dbfecga ecbd fdbgea dfaceg | gdbacfe fbgcaed cdafeg dc
abce ead aedgb dabgf ae cedbg cedgab bdfcage edbcgf fgedca | ae bace dea ae
bceg cfeag egdfca bcfage gb feabd dabgfc fedcgba ebfga gba | agb abedfgc gceb gceb
ac gbca cdabf fca agdbcfe fcadbg efgbad edcbf fdagb cafedg | cfa bacfd fgbad ca
ebcag degfba edcgbfa ecdab ceadbf dc dcef baefd agcfbd dcb | edfab bfeadgc acedb bdc
afgdcbe afdecb cafgdb abgfec afgcd ga gac cafdb abdg dcfeg | abdg bgcfae acg afebdcg
acdfb ed befdc adfcbg fdea ecd fgbdcea bagcde cegfb cfabed | de dce dcefab bcfge
ca dgac debacg fgcbea agedb fcdgeba bca ecfbd abcde feagdb | cgedab fbaecg faegdcb ac
ebdcfag bgcdea bface beadc edcgb da badg efdcgb cefadg ead | ead ad cebdag aedbc
fdeacgb bcgfda fbeag gebcaf adfeb cage fbcge agf ag bdgecf | ag adebf abecfdg ag
bgdca fgacbed cdfeb ecfg cfebda ebgdfc fgb gf fbgead gfdbc | gbf fdbce ebcafgd cedagfb
cgefad edbf fdc dgbec fabgc bfgcd bgfcade fd gbecad egcbfd | bgcfa febd dfc edcbg
efb edfba bcade cdbgae bcfa fcegbd agdfe bf afecdb agebfcd | fb abcf beagfdc ebf
egcd gbecad ed afdcb cbefag becag cdbae gacefbd beafgd ebd | bacdf dceg ed debgfa
gfbacde bfcd cd cebad ecdafb fecba gfcabe dce degcaf egadb | bgdcefa abcfge bfcd gbaecf
cdeaf cebafg afedg fcebdag gea dgabf ge aedcgf cdge fdecba | ge eg cegd degc
faedcg dfbc acegbfd eagbcd db eabcfd fbega cfaed adb edbfa | aedfc dfbc bd gacdfe
cbeadf gebfc age ag acedbg bgaec fgcadeb afdcge adgb ecbad | dcbfeag abdg dbga efbcg
gabd fdaecb gacbf dbcfg gfcae ba cba ecfgabd gdbcaf dfcbge | acb agdcfeb ecfga eagfc
gbcfd edbfc dgacbf cgb fgaedc gadcfbe bgaf ecagbd gb gdcfa | gbadce gcb gfdacbe cdgbeaf
fgeadc bedg bfcag cbdef fge eg edfcgba fdcbae ecdfgb bgfce | gfe eg fcbead eg
acegf gecfdab dacbfg afb ab fbaec aedb gcefbd ebcdfa bcfde | dgfbaec beafdc ba fgecbd
geabcdf egdb gb cfegab febgdc abfcd bgfcd gbf fcedg dcegaf | gafecbd gfb ebdg bg
cgefb fbagc ca abdc bgfcad bcefdga gdbaf edbafg cag feagcd | cgdafb badc adfbg acg
dbeg gd efcdag gabecdf cgd cdafb badcg edcbag gcabe agcbef | gcd dgc gcdba dgc
acgdf bcgeaf fd eagdfc dgcab dcafbeg fced gfd cegfa bdegaf | fd cedfag bcdga fgd
ef cbdae dgafce afe ebgf bafedg bacgdef agbfdc abdfe dgfab | bdefa fbge dabcgf fea
aefbdg cegadfb gcd dc dfaegc dcbaeg aegfd gecfd gcefb cafd | fegadc dgbcae cd cdg
bgdefc fabgec degbac aegdf abcge cdg adcb dagbefc eadgc dc | gdc cabd acebg cd
aebgc dbc dc acde agcdb fbcgeda cgbeda edfbcg ebfcga agfbd | gabce dbcga aecd ecda
deg eacgd ebdc bcdgae fdbgac afgbedc ed bcagd bdefga gfeac | badcg gbacd egbafcd bfdagc
fdega dca gfcba adgfc bcgd bgdcfa dc befcda gbafce bfgcead | agefdcb cabefgd dbaecf bdgc
abdc afecgb gbacfde dcfeg da agdce gbace fgadeb decbga dag | bcad cbda agbec adbc
beagfd cadef bfcae fcd fbgadc gdec gdcafeb gecdaf dc efagd | defac adgfeb agdcfbe bgcafed
eg bcfag gdecabf aecbfd gce dafecg gcbea dcebag gdeb bdace | badce ge eg cbdaegf
cfdgb dgbacef fa agf ceadbg efcbga efcgda afde adceg cagdf | agf gfa edfa agf
gbfcde edcfag dfg bdcgea cfedbag fcgba fd gafcd eadf cadge | bedagcf df dcafg eadgc
edfcab bdfca gfebdc acegdb dba dfae ad bcadgef bcfde fabgc | egbacdf ad dcebfag bdafc
acbfe afgd bgcaed acg bdeafcg ag dcbfge egfdca gefcd feagc | afgd dbgcef edgcf efcgd
dacgbe bgdc abdef bgead dag dg cebagf cgedaf aegcb fdegcab | cdbg adg gd dg
fg fcdgab abdfc deacgf fdabgec dgf dageb becdfa gbcf dgfab | dafcb agecdf adegbfc adbfc
cd ecgbf cgdf efbad bcadge gecdafb abcefg ecgbfd fedbc cbd | cgefb cdb fbecd cbd
afegdb gbafc bcafed dcbaeg efgd fe ecgbfda abgfe baegd eaf | egbad gfeab ebagf gbfae
bde fdebgac febgda cbea be adcefb fecdb dcgefa bfcgd caefd | fedbacg daefbg bed fbdce
eb ecb edacg acebfg cebagdf acdgfb cbega gefb agbfc eadcfb | fcagdb cbega be bcdafe
cefbgd gdaf dea ceagd cgedaf bgcfdea ad acbeg dbeacf ecgfd | dfecg agced dfag gceafd
da gedfc cafde bcagdef fad abde fbgaec badfgc daefcb aebcf | da bcfea ad dgeabcf
gdbeca gcadb facde afbdecg fg bagf cgdaf fgd cfebgd dagcfb | decabg gecdabf dagbc afbg
fbcdega febgcd df efcdg gbced dbefag aegfc cbgead fgd dbfc | dgabef cdfb dgf cgbefda
fbgaec acfgd gcdea cbedga acdfb dfeg gf aecfdg cbgfade cgf | gdfe gcf bcadeg fcg
fgced agfb faebcd gecabf eacgf ag ecfab aebdcg dfceagb cga | agc decgabf abegdfc agbdec
aecdbg cfeab cga fadceg adbeg cgbd ecabg gc dagfbec bdfaeg | beagd cag cfabe gca
afb gaefbc egbdf aedcb bdfae gbdace cbeadfg feacbd cafd fa | bcfaed gbadecf fa gbacfe
cbedg bgadef bag fadecbg ga fbcae fbdecg egcba dgac bdcage | ag febca cdebagf ebdgcfa
bcdaf adebgc ceb ebgdcf cfeagd agedc adecb eb gfdebca agbe | bcadf bdaecg facegd abcegd
afgbdce cagdfe dfb fdecb fb dabfge cbaf dcbfea gebcd fcdea | eafdcgb dbefc afebgd defca
dbfec abfecg fgced ceb dacfb dabe agefcbd gdacfb cfebad be | be cegdf gbcefad bdae
dbg bdagfc dgbfe cgebfd dbcaef ecbdf gfeab decg gcdebaf gd | cbeadf bdg aedfbc gdefbac
fcadgb bcf bgfcae cbfdeag face gfaebd cgbfe bcegd cf befag | abcgef bcf fbc cfebg
gefab db bda eafcd fcedbag cbedaf adfcgb ecdb acefgd fedba | db bfade ebdacf afebcd
dfbeacg cedagf dgfbc bgfda gdfea fab egbfad bage efcabd ab | agbfd bfa adfbgce fab
agfcd ecgabd bdgf egadfcb agf gf dfaec dgabc bfceag bacfgd | agf fag cfdea fdcgba
gebfdc fdae gbafc fdg dfagb aefdbg df baged ebfcdag acbedg | cfegbda bgfade daefbg abdecfg
gcdba gdefba ad dga deca cedgb cgbaf gdcbae gedfcba becgfd | gfabc ad fbcag dag
gefbd eagbdcf bfg bg aegb fbcde efdgac fgadcb efgad ebafdg | gbdfe bfedc cefagbd bdfce
gfedb cbegadf bcga fdcgea aecgbd bdcfea dgc bgcde abecd cg | befcad gedcfab bcedfa agbc
cedgfa efbdagc efdabc be baedgc bdfec fdgbc ceb dfcea afbe | eb afeb ebfa cbe
fedc fgbead dacgfe ec cagebf geacd dabcg ceg fgdea gdcbefa | ceagfb ecg dcef dcagfeb
aed agfbde bgfcda deafb fgae gcebdfa gdafb bfdec ae egabdc | ea dae agfe bcgadf
bdc cb gcbdf cgdebf adecgf gdefc dfcgaeb gadbf ebgadc cebf | fbce bfec dgcfe cfbe
fdecbg fdgac df fcgae dcf gcefdab aebcgf cdabg daef gefdac | fd ecbdfg edaf fade
cagebd bacfg egbdaf gc cgdf cbafe cbfdag cgb adbfg gecbfda | gbc adbgf gc bdecag
afed eag ecgbaf egdba egafbcd dbagc aebfdg dgebf gbfced ae | aedfgbc ea ea ega
eagfc gdbefc gf cgefda gfad adgbce dcgae aefcb egf fedabgc | adfcbge cdfega gaedbcf egcaf
bcage gbfadc dgfa abdcf cbafde bgfedc fbcga fbg fedbagc gf | fg fecdabg gf adfg
dcfge edfcbga defac febdag fdaecg ceag egd cfdgb eafbdc eg | cfdae fdceg efabdc egdafb
de bcdaf feagc fdeac gead aefcdgb afecgd cagbfe dce gbedcf | ed ed de deafc
cdaeg cb bafge fbcdea afbegc gbfc bca dgfebca gbadfe gebca | bfgae fcbg fbage cbfg
agf gdab bgcfe gfecda bafecgd dcefab afbeg baefdg febad ag | fbade eabcdf ga gadb
gcaf gfbec cbdega ebdacf cadgefb cf bgcaef edbfg fcb gcbae | bdfcega gdbfe fdbge fcb
afc gbace cbfd badfe fc faebc edabcf bafcdge eadgbf gdcaef | fc cf gbaefd fcbd
cdfage cdfebag eb cgaef cbgae ebgacf acgdb aeb ebcf agfdbe | cdbfega cebf ecfagd bcfe
afbdec dfegc dfga cedfga fgcebda edfca gf cdebg bcaegf gfe | efg cfgde dgaf fdga
ecfab eda da gefcad gcad aegcfdb dgbafe fegdc ebcfgd afecd | cegfad cefda ad caedf
gbedc gbadf bgafdce faed acdbgf agbedf fgecba ef egf efbdg | fbecga fe bgcdfea aegdbf
bagcdf dacbg fdbgcae dgbfa fa afd cfba gdcefa gabced fgedb | afd af fa bfca
fbgaed fbedg baed dcfgbe bga cfgda ebgcfa ba dbgeafc dafgb | aebgcf daeb ba ab
cdga feadgcb fbdeac ad dfbcge gcdef fda gedafc adegf eagbf | efagb fdcge faged fagebdc
ceafdb ga acfde acg egcafd cebfg febcgda dcgbaf egad fcega | ecagdbf gebcf cgfeadb cgbfda`;

var lines = input.split("\n");

for(var l = 0;l < lines.length;l ++){
  
  lines[l] = lines[l].split(" | ");
  
  lines[l][0] = lines[l][0].split(" ");
  lines[l][1] = lines[l][1].split(" ");
  
}

var count = 0;

for(var l = 0;l < lines.length;l ++){
  
  for(var a = 0;a < lines[l][1].length;a ++){
    
    var len = lines[l][1][a].length;
    
    if(len == 2 || len == 3 || len == 4 || len == 7){
      count ++;
    }
    
  }
  
}

console.log("Part 1:",count);


/////////////////////////////////////////////////////////////////


var sumPart2 = 0;

var letters = ["a","b","c","d","e","f","g"];

function identifyLetter(letter){
  for(var l = 0;l < letters.length;l ++){
    if(letter == letters[l]){
      return l;
    }
  }
  return undefined;
}

for(var l = 0;l < lines.length;l ++){
  
  //By default:
  //0 = abcefg
  //1 = cf
  //2 = acdeg
  //3 = acdfg
  //4 = bcdf
  //5 = abdfg
  //6 = abdefg
  //7 = acf
  //8 = abcdefg
  //9 = abcdfg
  
  //Occurrences:
  //a -> 8
  //b -> 6
  //c -> 8
  //d -> 7
  //e -> 4
  //f -> 9
  //g -> 7
  
  //Store the list of letters which makes up each number 0-9
  var nums = ["","","","","","","","","",""];
  
  //         a  b  c  d  e  f  g
  var map = ["","","","","","",""];
  
  for(var n = 0;n <= 9;n ++){
    
    for(var a = 0;a < lines[l][0].length;a ++){
      
      var len = lines[l][0][a].length;
      
      if(len == 2){
        nums[1] = lines[l][0][a];
        
      }else if(len == 3){
        nums[7] = lines[l][0][a];
        
      }else if(len == 4){
        nums[4] = lines[l][0][a];
        
      }else if(len == 7){
        nums[8] = lines[l][0][a];
        
      }
      
    }
    
  }
  
  
  //Use a combination of methods to identify the mapping of each letter
  
  //Store how many occurrences of each letter a-g were found in the given (unordered) list of numbers 0-9
  //                 a b c d e f g
  var occurrences = [0,0,0,0,0,0,0];
  
  for(var a = 0;a < lines[l][0].length;a ++){
    
    for(var b = 0;b < lines[l][0][a].length;b ++){
      
      occurrences[identifyLetter(lines[l][0][a].charAt(b))] ++;
      
    }
    
  }
  
  ///console.log("occurrences:",occurrences);
  
  //Use the number 1 to work out which letters map to "c" and "f"
  for(var a = 0;a < 7;a ++){
    for(var b = 0;b < 2;b ++){
      
      if(letters[a] == nums[1].charAt(b)){
        
        if(occurrences[a] == 9){
          map[5] = nums[1].charAt(b);
        }else{
          map[2] = nums[1].charAt(b);
        }
        
        b = 2;
      }
      
    }
  }
  
  //Work out which letter maps to "a"
  for(var a = 0;a < 3;a ++){
    
    var alreadyFound = false;
    
    for(var b = 0;b < 7;b ++){
      
      if(map[b] == nums[7].charAt(a)){
        alreadyFound = true;
        b = 7;
      }
      
    }
    
    if(!alreadyFound){
      map[0] = nums[7].charAt(a);
      a = 3;
    }
    
  }
  
  //Work out which letters map to "b" and "e"
  for(var a = 0;a < 7;a ++){
    if(occurrences[a] == 6){
      //6 occurrences -> b
      map[1] = letters[a];
    }else if(occurrences[a] == 4){
      //4 occurrences -> e
      map[4] = letters[a];
    }
  }
  
  //Work out which letter maps to "d"
  for(var a = 0;a < 4;a ++){
    
    var alreadyFound = false;
    
    for(var b = 0;b < 7;b ++){
      
      if(map[b] == nums[4].charAt(a)){
        alreadyFound = true;
        b = 7;
      }
      
    }
    
    var letterNum = identifyLetter(nums[4].charAt(a));
    
    if(occurrences[letterNum] == 7){
      
      map[3] = letters[letterNum];
      
    }
    
  }
  
  //Work out which letter maps to "g"
  for(var a = 0;a < 7;a ++){
    if(occurrences[a] == 7){
      var alreadyFound = false;
      for(var b = 0;b < 7;b ++){
        if(map[b] == letters[a]){
          alreadyFound = true;
          b = 7;
        }
      }
      if(!alreadyFound){
        map[6] = letters[a];
      }
    }
  }
  
  ///console.log("map:",map);
  
  //Now that all the letter mappings are known, the numbers can be reconstructed
  nums[0] = map[0]+map[1]+map[2]+map[4]+map[5]+map[6];
  nums[2] = map[0]+map[2]+map[3]+map[4]+map[6];
  nums[3] = map[0]+map[2]+map[3]+map[5]+map[6];
  nums[5] = map[0]+map[1]+map[3]+map[5]+map[6];
  nums[6] = map[0]+map[1]+map[3]+map[4]+map[5]+map[6];
  nums[9] = map[0]+map[1]+map[2]+map[3]+map[5]+map[6];
  
  ///console.log("nums:",nums);
  
  
  var output = 0;
  
  for(var a = 0;a < 4;a ++){
    
    //Identify which number a sequence of letters refers to
    var sequence = lines[l][1][a];
    
    var match = 0;
    
    //Loop through numbers 0-9
    for(var b = 0;b <= 9;b ++){
      
      //Start with a basic length match test
      if(nums[b].length == sequence.length){
        
        var equalLetters = true;
        
        //Loop through letters a-g
        for(var c = 0;c < 7;c ++){
          
          if(nums[b].includes(letters[c])){
            if(!sequence.includes(letters[c])){
              equalLetters = false;
            }
          }
          
          if(sequence.includes(letters[c])){
            if(!nums[b].includes(letters[c])){
              equalLetters = false;
            }
          }
          
        }
        
        if(equalLetters){
          match = b;
          b = 9;
        }
        
      }
      
    }
    
    output += match * Math.pow(10,4-a-1);
    
  }
  
  ///console.log("output:",output);
  
  sumPart2 += output;
  
}

console.log("Part 2:",sumPart2);














