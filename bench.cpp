#include <iostream>
#include <vector>

using namespace std;

double Solve(vector<vector<double> >& t)
{	
  	for (int idxLine = 1; idxLine < t.size(); idxLine++){
  		for (int idxCol = 0; idxCol < t[idxLine].size(); idxCol++){
  			if (idxCol == 0){
  				t[idxLine][idxCol] += t[idxLine-1][0];
  			} else if (idxCol == t[idxLine].size()-1){
  				t[idxLine][idxCol] += t[idxLine-1][idxCol-1];
  			} else {
  				t[idxLine][idxCol] += max(t[idxLine-1][idxCol-1], t[idxLine][idxCol]);
  			}
  		}
  	}
  	
    double maxi = -1;
    for (int i = 0; i < t[t.size()-1].size(); i++){
    	double elem = t[t.size()-1][i];
    	maxi = max(maxi, elem);
    }
  	return maxi;
}

int main()
{
	vector<vector<double> > array;
	vector<double> l1;
	l1.push_back(3);
	array.push_back(l1);
	
	vector<double> l2;
	l2.push_back(7);
	l2.push_back(4);
	array.push_back(l2);

	vector<double> l3;
	l3.push_back(2);
	l3.push_back(4);
	l3.push_back(6);	
	array.push_back(l3);
	
	vector<double> l4;
	l4.push_back(8);
	l4.push_back(5);	
	l4.push_back(9);
	l4.push_back(3);
	array.push_back(l4);
	
	double total = 0;		
	for (int i = 0; i < 1000000; i++) {
		total += Solve(array);
	}

	cout << total << endl;
	
	return 0;
}

