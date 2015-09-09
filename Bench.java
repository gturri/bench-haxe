import java.util.ArrayList;
import java.util.List;

class Bench {
  public static void main(String[] args){
    List<List<Double>> array = new ArrayList<List<Double>>();
    List<Double> l1 = new ArrayList<Double>();
    l1.add(3.0);
    array.add(l1);

    List<Double> l2 = new ArrayList<Double>();
    l2.add(7.0);
    l2.add(4.0);
    array.add(l2);

    List<Double> l3 = new ArrayList<Double>();
    l3.add(2.0);
    l3.add(4.0);
    l3.add(6.0);
    array.add(l3);

    List<Double> l4 = new ArrayList<Double>();
    l4.add(8.0);
    l4.add(5.0);
    l4.add(9.0);
    l4.add(3.0);

    double sum = 0.0;
    for ( int i = 0 ; i < 1000000 ; i++ ){
      sum += solve(array);
    }
    System.out.println(sum);
  }

  public static Double solve(List<List<Double>> t){
  	for (int idxLine = 1; idxLine < t.size(); idxLine++){
  		for (int idxCol = 0; idxCol < t.get(idxLine).size(); idxCol++){
  			if (idxCol == 0){
  				t.get(idxLine).set(idxCol, t.get(idxLine).get(idxCol) + t.get(idxLine-1).get(0));
  			} else if (idxCol == t.get(idxLine).size()-1){
  				t.get(idxLine).set(idxCol, t.get(idxLine).get(idxCol) + t.get(idxLine-1).get(idxCol-1));
  			} else {
  				t.get(idxLine).set(idxCol, t.get(idxLine).get(idxCol) + Math.max(t.get(idxLine-1).get(idxCol-1), t.get(idxLine).get(idxCol)));
  			}
  		}
  	}
  	
    Double maxi = -1.0;
    for (int i = 0; i < t.get(t.size()-1).size(); i++){
    	Double elem = t.get(t.size()-1).get(i);
    	maxi = Math.max(maxi, elem);
    }
  	return maxi;
  }
}
