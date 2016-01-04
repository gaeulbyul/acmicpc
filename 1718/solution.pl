chomp($p=<>);chomp($k=<>);@v=unpack'W*',$k;for$c(unpack'W*',$p){$X=($c-$v[$n++%$k=~y///c])%26;if($c==32){print$";next}print$X?chr($X+96):'z';next}
