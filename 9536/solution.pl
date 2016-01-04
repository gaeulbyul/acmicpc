$te=<>;
while($te>=0){
$l=<>;
chomp$l;
@howls=split/ /,$l;
while(<>){
if(/\S+ goes (\S+)/){
@howls=grep$1 ne$_,@howls;
}else{last}
}
print"@howls\n";
$te--;}
