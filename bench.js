'use strict';

var array = [
		  	[3],
		  	[7,4],
		  	[2,4,6],
		  	[8,5,9,3]
		  				];
var sum = 0;
for (var i = 0 ; i < 10000000 ; i++ ){
  sum += solve(array);
}
console.log(sum);

function solve(t){
  for (var idxLine = 1 ; idxLine < t.length ; idxLine++ ){
    for ( var idxCol = 0 ; idxCol < t[idxLine].length ; idxCol++ ){
      if ( idxCol == 0 ){
        t[idxLine][idxCol] += t[idxLine-1][0];
      } else if ( idxCol == t[idxLine].length - 1 ){
        t[idxLine][idxCol] += t[idxLine-1][idxCol - 1];
      } else {
        t[idxLine][idxCol] += Math.max(t[idxLine-1][idxCol-1], t[idxLine][idxCol]);
      }
    }
  }

  var maxi = -1.0;
  for (var elem = 0 ; elem < t[t.length-1].length ; elem++ ){
  	maxi = Math.max(maxi, t[t.length-1][elem]);
  }
  return maxi;
}
