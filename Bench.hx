class Bench
{
  static public function main():Void 
  {
  	//read input
  	var array = new Array<Array<Float>>();
  	
  	array = [
			  	[3],
			  	[7,4],
			  	[2,4,6],
			  	[8,5,9,3]
			  				];
  	
  	
  	var sum = 0.0;
  	for ( i in 0...10000000 ){
	  	sum += Solve(array);
    }
    trace(sum);
  }
  
  static private function Solve(t:Array<Array<Float>>):Float
  {
  	for ( idxLine in (1...t.length) ){
  		for ( idxCol in (0...t[idxLine].length) ){
  			if ( idxCol == 0){
  				t[idxLine][idxCol] += t[idxLine-1][0];
  			} else if (idxCol == t[idxLine].length-1){
  				t[idxLine][idxCol] += t[idxLine-1][idxCol-1];
  			} else {
  				t[idxLine][idxCol] += Math.max(t[idxLine-1][idxCol-1], t[idxLine][idxCol]);
  			}
  		}
  	} 
  
    var maxi = -1.0;
    for (elem in t[t.length-1]){
    	maxi = Math.max(maxi, elem);
    }
  	return maxi;
  }
}
